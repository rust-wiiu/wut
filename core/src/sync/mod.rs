//! Useful synchronization primitives.

pub use core::sync::*;

mod event;
mod lazy_lock;
pub mod mpmc;
pub mod mpsc;
mod mutex;
mod once_lock;
mod rwlock;
mod semaphore;

pub use event::{AutoEvent, ManualEvent};
pub use lazy_lock::LazyLock;
pub use mutex::{Mutex, MutexError};
pub use once_lock::OnceLock;
// pub use rwlock::RwLock;
pub use semaphore::Semaphore;

/// Trickery for constant mutex
pub type ConstMutex<T> = LazyLock<Mutex<T>>;
