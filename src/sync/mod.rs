mod lazy_lock;
mod mutex;
mod once_lock;
mod rwlock;

pub use lazy_lock::LazyLock;
pub use mutex::{Mutex, MutexError};
pub use once_lock::OnceLock;
pub use rwlock::RwLock;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum State {
    Poisoned,
    Incomplete,
    Complete,
}
