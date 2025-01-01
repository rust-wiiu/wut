use core::{
    cell::UnsafeCell,
    fmt,
    marker::PhantomData,
    mem::MaybeUninit,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::atomic::{AtomicBool, Ordering},
};

/// A synchronization primitive which can nominally be written to only once.
///
/// This type is a thread-safe [`OnceCell`], and can be used in statics.
pub struct OnceLock<T> {
    is_init: AtomicBool,
    value: UnsafeCell<MaybeUninit<T>>,
    _marker: PhantomData<T>,
}

impl<T> OnceLock<T> {
    /// Creates a new empty cell.
    #[inline]
    #[must_use]
    pub const fn new() -> OnceLock<T> {
        OnceLock {
            is_init: AtomicBool::new(false),
            value: UnsafeCell::new(MaybeUninit::uninit()),
            _marker: PhantomData,
        }
    }

    /// Gets the reference to the underlying value.
    ///
    /// Returns `None` if the cell is empty, or being initialized. This method never blocks.
    #[inline]
    pub fn get(&self) -> Option<&T> {
        if self.is_initialized() {
            Some(unsafe { self.get_unchecked() })
        } else {
            None
        }
    }

    /// Gets the mutable reference to the underlying value.
    ///
    /// Returns `None` if the cell is empty. This method never blocks.
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_initialized() {
            Some(unsafe { self.get_unchecked_mut() })
        } else {
            None
        }
    }

    /// Blocks the current thread until the cell is initialized.
    #[inline]
    pub fn wait(&self) -> &T {
        // wait
        todo!();
        unsafe { self.get_unchecked() }
    }

    /// Sets the contents of this cell to `value`.
    ///
    /// May block if another thread is currently attempting to initialize the cell. The cell is guaranteed to contain a value when set returns, though not necessarily the one provided.
    ///
    /// Returns `Ok(())` if the cell's value was set by this call.
    #[inline]
    pub fn set(&self, value: T) -> Result<(), T> {
        if self
            .is_init
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            unsafe {
                (*self.value.get()).as_mut_ptr().write(value);
            }
            Ok(())
        } else {
            Err(value)
        }
    }

    /// Sets the contents of this cell to `value` if the cell was empty, then returns a reference to it.
    ///
    /// May block if another thread is currently attempting to initialize the cell. The cell is guaranteed to contain a value when set returns, though not necessarily the one provided.
    ///
    /// Returns `Ok(&value)` if the cell was empty and `Err(&current_value, value)` if it was full.
    #[inline]
    pub fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {
        let mut value = Some(value);
        let res = self.get_or_init(|| value.take().unwrap());
        match value {
            None => Ok(res),
            Some(value) => Err((res, value)),
        }
    }

    /// Gets the contents of the cell, initializing it with `f` if the cell was empty.
    ///
    /// Many threads may call `get_or_init` concurrently with different initializing functions, but it is guaranteed that only one function will be executed.
    ///
    /// # Panics
    ///
    /// If `f` panics, the panic is propagated to the caller, and the cell remains uninitialized.
    ///
    /// It is an error to reentrantly initialize the cell from `f`. The exact outcome is unspecified. Current implementation deadlocks, but this may be changed to a panic in the future.
    #[inline]
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        match self.get_or_try_init(|| Ok::<T, ()>(f())) {
            Ok(val) => val,
            Err(_) => panic!("This can never happen!"),
        }
    }

    /// Gets the mutable reference of the contents of the cell, initializing it with `f` if the cell was empty.
    ///
    /// This method never blocks.
    ///
    /// # Panics
    ///
    /// If `f` panics, the panic is propagated to the caller, and the cell remains uninitialized.
    #[inline]
    pub fn get_mut_or_init<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        match self.get_mut_or_try_init(|| Ok::<T, ()>(f())) {
            Ok(val) => val,
            Err(_) => panic!("This can never happen!"),
        }
    }

    /// Gets the contents of the cell, initializing it with `f` if the cell was empty. If the cell was empty and `f` failed, an error is returned.
    ///
    /// # Panics
    ///
    /// If `f` panics, the panic is propagated to the caller, and the cell remains uninitialized.
    ///
    /// It is an error to reentrantly initialize the cell from `f`. The exact outcome is unspecified. Current implementation deadlocks, but this may be changed to a panic in the future.
    #[inline]
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        // Fast path check
        // NOTE: We need to perform an acquire on the state in this method
        // in order to correctly synchronize `LazyLock::force`. This is
        // currently done by calling `self.get()`, which in turn calls
        // `self.is_initialized()`, which in turn performs the acquire.
        if let Some(value) = self.get() {
            return Ok(value);
        }
        self.initialize(f)?;

        debug_assert!(self.is_initialized());

        // SAFETY: The inner value has been initialized
        Ok(unsafe { self.get_unchecked() })
    }

    /// Gets the mutable reference of the contents of the cell, initializing it with `f` if the cell was empty. If the cell was empty and `f` failed, an error is returned.
    ///
    /// This method never blocks.
    ///
    /// # Panics
    ///
    /// If `f` panics, the panic is propagated to the caller, and the cell remains uninitialized.
    #[inline]
    pub fn get_mut_or_try_init<F, E>(&mut self, f: F) -> Result<&mut T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        if self.get().is_none() {
            self.initialize(f)?;
        }
        debug_assert!(self.is_initialized());
        // SAFETY: The inner value has been initialized
        Ok(unsafe { self.get_unchecked_mut() })
    }

    /// Consumes the `OnceLock`, returning the wrapped value. Returns `None` if the cell was empty.
    #[inline]
    pub fn into_inner(mut self) -> Option<T> {
        self.take()
    }

    /// Takes the value out of this `OnceLock`, moving it back to an uninitialized state.
    ///
    /// Has no effect and returns `None` if the `OnceLock` hasn't been initialized.
    ///
    /// Safety is guaranteed by requiring a mutable reference.
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        if self.is_initialized() {
            self.is_init.store(false, Ordering::SeqCst);
            // SAFETY: `self.value` is initialized and contains a valid `T`.
            // `self.once` is reset, so `is_initialized()` will be false again
            // which prevents the value from being read twice.
            unsafe { Some((&mut *self.value.get()).assume_init_read()) }
        } else {
            None
        }
    }

    #[inline]
    fn is_initialized(&self) -> bool {
        self.is_init.load(Ordering::Acquire)
    }

    #[cold]
    // #[optimize(size)]
    fn initialize<F, E>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        let mut res: Result<(), E> = Ok(());
        let slot = &self.value;

        match f() {
            Ok(value) => {
                unsafe { (&mut *slot.get()).write(value) };
            }
            Err(e) => {
                res = Err(e);
            }
        }
        res
    }

    /// # Safety
    ///
    /// The value must be initialized
    #[inline]
    unsafe fn get_unchecked(&self) -> &T {
        unsafe { (&*self.value.get()).assume_init_ref() }
    }

    /// # Safety
    ///
    /// The value must be initialized
    #[inline]
    unsafe fn get_unchecked_mut(&mut self) -> &mut T {
        unsafe { (&mut *self.value.get()).assume_init_mut() }
    }
}

unsafe impl<T: Sync + Send> Sync for OnceLock<T> {}
unsafe impl<T: Send> Send for OnceLock<T> {}

impl<T: RefUnwindSafe + UnwindSafe> RefUnwindSafe for OnceLock<T> {}
impl<T: UnwindSafe> UnwindSafe for OnceLock<T> {}

impl<T> Default for OnceLock<T> {
    #[inline]
    fn default() -> Self {
        OnceLock::new()
    }
}

impl<T: fmt::Debug> fmt::Debug for OnceLock<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_tuple("OnceLock");
        match self.get() {
            Some(v) => d.field(v),
            None => d.field(&format_args!("<uninit>")),
        };
        d.finish()
    }
}

impl<T: Clone> Clone for OnceLock<T> {
    #[inline]
    fn clone(&self) -> OnceLock<T> {
        let cell = Self::new();
        if let Some(value) = self.get() {
            match cell.set(value.clone()) {
                Ok(()) => (),
                Err(_) => unreachable!(),
            }
        }
        cell
    }
}

impl<T> From<T> for OnceLock<T> {
    #[inline]
    fn from(value: T) -> Self {
        let cell = Self::new();
        match cell.set(value) {
            Ok(()) => cell,
            Err(_) => unreachable!(),
        }
    }
}

impl<T: PartialEq> PartialEq for OnceLock<T> {
    #[inline]
    fn eq(&self, other: &OnceLock<T>) -> bool {
        self.get() == other.get()
    }
}

impl<T: Eq> Eq for OnceLock<T> {}

impl<T> Drop for OnceLock<T> {
    #[inline]
    fn drop(&mut self) {
        if self.is_initialized() {
            // SAFETY: The cell is initialized and being dropped, so it can't
            // be accessed again. We also don't touch the `T` other than
            // dropping it, which validates our usage of #[may_dangle].
            unsafe { (&mut *self.value.get()).assume_init_drop() };
        }
    }
}
