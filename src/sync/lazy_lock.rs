use super::State;
use core::{
    cell::UnsafeCell,
    fmt::Debug,
    mem::ManuallyDrop,
    ops::Deref,
    panic::{RefUnwindSafe, UnwindSafe},
};

union Data<T, F> {
    value: ManuallyDrop<T>,
    f: ManuallyDrop<F>,
}

pub struct LazyLock<T, F = fn() -> T> {
    state: UnsafeCell<State>,
    data: UnsafeCell<Data<T, F>>,
}

impl<T, F: FnOnce() -> T> LazyLock<T, F> {
    #[inline]
    pub const fn new(f: F) -> Self {
        Self {
            state: UnsafeCell::new(State::Incomplete),
            data: UnsafeCell::new(Data {
                f: ManuallyDrop::new(f),
            }),
        }
    }

    // #[inline]
    // pub(crate) fn preinit(value: T) -> Self {
    //     Self {
    //         state: UnsafeCell::new(State::Complete),
    //         data: UnsafeCell::new(Data {
    //             value: ManuallyDrop::new(value),
    //         }),
    //     }
    // }

    #[inline]
    pub fn into_inner(this: Self) -> Result<T, F> {
        match unsafe { &*this.state.get() } {
            State::Poisoned => panic_poisoned(),
            state => {
                let this = ManuallyDrop::new(this);
                let data = unsafe { core::ptr::read(&this.data) }.into_inner();
                match state {
                    State::Incomplete => Err(ManuallyDrop::into_inner(unsafe { data.f })),
                    State::Complete => Ok(ManuallyDrop::into_inner(unsafe { data.value })),
                    State::Poisoned => unreachable!(),
                }
            }
        }
    }

    #[inline]
    pub fn force_mut(this: &mut LazyLock<T, F>) -> &mut T {
        #[cold]
        unsafe fn really_init_mut<T, F: FnOnce() -> T>(this: &mut LazyLock<T, F>) -> &mut T {
            struct PoisonOnPanic<'a, T, F>(&'a mut LazyLock<T, F>);
            impl<T, F> Drop for PoisonOnPanic<'_, T, F> {
                #[inline]
                fn drop(&mut self) {
                    let state = self.0.state.get_mut();
                    *state = State::Poisoned;
                }
            }

            let f = unsafe { ManuallyDrop::take(&mut this.data.get_mut().f) };

            let guard = PoisonOnPanic(this);
            let data = f();
            guard.0.data.get_mut().value = ManuallyDrop::new(data);
            guard.0.state = UnsafeCell::new(State::Complete);
            core::mem::forget(guard);
            //
            unsafe { &mut this.data.get_mut().value }
        }

        match unsafe { &*this.state.get() } {
            State::Poisoned => panic_poisoned(),
            State::Complete => unsafe { &mut this.data.get_mut().value },
            State::Incomplete => unsafe { really_init_mut(this) },
        }
    }

    #[inline]
    pub fn force(this: &LazyLock<T, F>) -> &T {
        if *unsafe { &*this.state.get() } == State::Incomplete {
            let data = unsafe { &mut *this.data.get() };
            let f = unsafe { ManuallyDrop::take(&mut data.f) };
            let value = f();
            data.value = ManuallyDrop::new(value);
        }
        unsafe { &*(*this.data.get()).value }
    }
}

impl<T, F> LazyLock<T, F> {
    #[inline]
    pub fn get_mut(this: &mut LazyLock<T, F>) -> Option<&mut T> {
        match unsafe { &*this.state.get() } {
            State::Complete => Some(unsafe { &mut this.data.get_mut().value }),
            _ => None,
        }
    }

    #[inline]
    pub fn get(this: &LazyLock<T, F>) -> Option<&T> {
        match unsafe { &*this.state.get() } {
            State::Complete => Some(unsafe { &(*this.data.get()).value }),
            _ => None,
        }
    }
}

impl<T, F> Drop for LazyLock<T, F> {
    fn drop(&mut self) {
        match unsafe { &*self.state.get() } {
            State::Poisoned => {}
            State::Incomplete => unsafe { ManuallyDrop::drop(&mut self.data.get_mut().f) },
            State::Complete => unsafe {
                ManuallyDrop::drop(&mut self.data.get_mut().value);
            },
        }
    }
}

impl<T, F: FnOnce() -> T> Deref for LazyLock<T, F> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        LazyLock::force(self)
    }
}

impl<T: Default> Default for LazyLock<T> {
    #[inline]
    fn default() -> Self {
        LazyLock::new(T::default)
    }
}

impl<T: Debug, F> Debug for LazyLock<T, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut d = f.debug_tuple("LazyLock");
        match LazyLock::get(self) {
            Some(v) => d.field(v),
            None => d.field(&format_args!("<uninit>")),
        };
        d.finish()
    }
}

#[cold]
#[inline(never)]
fn panic_poisoned() -> ! {
    panic!("LazyLock instance has previously been poisoned")
}

unsafe impl<T: Sync + Send, F: Send> Sync for LazyLock<T, F> {}
impl<T: RefUnwindSafe + UnwindSafe, F: UnwindSafe> RefUnwindSafe for LazyLock<T, F> {}
impl<T: UnwindSafe, F: UnwindSafe> UnwindSafe for LazyLock<T, F> {}
