// use super::State;
use core::{
    cell::UnsafeCell,
    fmt,
    fmt::Debug,
    mem::ManuallyDrop,
    ops::Deref,
    sync::atomic::{AtomicU8, Ordering},
};

#[derive(Debug)]
enum ExclusiveState {
    Incomplete,
    Initializing,
    Complete,
    Poisoned,
}

impl Into<u8> for ExclusiveState {
    fn into(self) -> u8 {
        match self {
            Self::Incomplete => 0,
            Self::Initializing => 1,
            Self::Complete => 2,
            Self::Poisoned => 3,
        }
    }
}

impl From<u8> for ExclusiveState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Incomplete,
            1 => Self::Initializing,
            2 => Self::Complete,
            3 => Self::Poisoned,
            _ => unreachable!(),
        }
    }
}

struct State(AtomicU8);

impl State {
    const fn new() -> Self {
        Self(AtomicU8::new(0))
    }

    fn get(&self) -> ExclusiveState {
        ExclusiveState::from(self.0.load(Ordering::Acquire))
    }

    fn set(&self, state: ExclusiveState) {
        self.0.store(state.into(), Ordering::Release);
    }

    fn is_complete(&self) -> bool {
        matches!(self.get(), ExclusiveState::Complete)
    }

    fn try_initialize<F>(&self, f: F) -> bool
    where
        F: FnOnce(),
    {
        if self
            .0
            .compare_exchange(
                ExclusiveState::Incomplete.into(),
                ExclusiveState::Initializing.into(),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
        {
            // here would be the place to catch a panic and poison
            f();

            self.set(ExclusiveState::Complete);
            true
        } else {
            false
        }
    }
}

union Data<T, F> {
    value: ManuallyDrop<T>,
    f: ManuallyDrop<F>,
}

pub struct LazyLock<T, F = fn() -> T> {
    state: State,
    data: UnsafeCell<Data<T, F>>,
}

unsafe impl<T: Sync + Send, F: Send> Sync for LazyLock<T, F> {}
unsafe impl<T: Send, F: Send> Send for LazyLock<T, F> {}

impl<T, F: FnOnce() -> T> LazyLock<T, F> {
    /// Creates a new lazy value with the given initializing function.
    #[inline]
    pub const fn new(f: F) -> LazyLock<T, F> {
        LazyLock {
            state: State::new(),
            data: UnsafeCell::new(Data {
                f: ManuallyDrop::new(f),
            }),
        }
    }

    fn initialize(&self) {
        self.state.try_initialize(|| {
            let data = unsafe { &mut *self.data.get() };
            let f = unsafe { ManuallyDrop::take(&mut data.f) };
            let value = f();
            data.value = ManuallyDrop::new(value);
        });
    }

    pub fn force(this: &LazyLock<T, F>) -> &T {
        if !this.state.is_complete() {
            this.initialize();
        }

        // while !this.state.is_complete() {}

        match this.state.get() {
            ExclusiveState::Complete => unsafe { &(*this.data.get()).value },
            ExclusiveState::Poisoned => panic!("LazyLock has been poisoned"),
            ExclusiveState::Incomplete => unreachable!(),
            ExclusiveState::Initializing => unreachable!(),
        }
    }

    /// Returns a reference to the value if initialized, or `None` if not.
    pub fn get(this: &LazyLock<T, F>) -> Option<&T> {
        match this.state.get() {
            ExclusiveState::Complete => Some(unsafe { &(*this.data.get()).value }),
            _ => None,
        }
    }
}

impl<T, F: FnOnce() -> T> Deref for LazyLock<T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        LazyLock::force(self)
    }
}

impl<T: Default> Default for LazyLock<T> {
    fn default() -> LazyLock<T> {
        LazyLock::new(T::default)
    }
}

impl<T: fmt::Debug, F: FnOnce() -> T> fmt::Debug for LazyLock<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match LazyLock::get(self) {
            Some(v) => f.debug_tuple("LazyLock").field(v).finish(),
            None => f
                .debug_tuple("LazyLock")
                .field(&format_args!("<uninit>"))
                .finish(),
        }
    }
}

impl<T, F> Drop for LazyLock<T, F> {
    fn drop(&mut self) {
        match self.state.get() {
            ExclusiveState::Incomplete => unsafe { ManuallyDrop::drop(&mut (*self.data.get()).f) },
            ExclusiveState::Complete => unsafe {
                ManuallyDrop::drop(&mut (*self.data.get()).value)
            },
            ExclusiveState::Initializing => {}
            ExclusiveState::Poisoned => {}
        }
    }
}
