//!
//! Module level documentation
//!

use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};
use thiserror::Error;
use wut_sys as sys;

/// CafeOS Mutex
///
/// Usage similar to std::sync::Mutex
pub struct Mutex<T> {
    inner: UnsafeCell<T>,
    mutex: UnsafeCell<sys::OSMutex>,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

#[derive(Debug, Error)]
pub enum MutexError {
    #[error("Thread locking mutex has paniced")]
    Poisoned,
    #[error("Try waiting for thread failed")]
    AlreadyLocked,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Create a new mutex
    ///
    /// As mutexes must be registerd by the OS, they cannot be created with a `const` function. As a workaround use [ConstMutex][super::ConstMutex].
    pub fn new(inner: T) -> Self {
        let mut mutex = sys::OSMutex::default();
        unsafe {
            sys::OSInitMutex(&mut mutex);
        }

        Self {
            inner: UnsafeCell::new(inner),
            mutex: UnsafeCell::new(mutex),
        }
    }

    /// Lock mutex in current thread
    pub fn lock(&self) -> Result<MutexGuard<T>, MutexError> {
        unsafe {
            sys::OSLockMutex(self.mutex.get());
        }
        Ok(MutexGuard { mutex: self })
    }

    /// Try to lock mutex in current thread
    pub fn try_lock(&self) -> Result<MutexGuard<T>, MutexError> {
        unsafe {
            let res = sys::OSTryLockMutex(self.mutex.get());
            match res {
                1 => Ok(MutexGuard { mutex: self }),
                _ => Err(MutexError::AlreadyLocked),
            }
        }
    }

    /// Get current mutex count
    pub fn count(&self) -> i32 {
        unsafe { (*self.mutex.get()).count }
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.inner.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.inner.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        // check for panic and poison thread if needed
        unsafe {
            sys::OSUnlockMutex(self.mutex.mutex.get());
        }
    }
}
