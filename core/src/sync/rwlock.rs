/*
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

pub struct RwLock<T> {
    readers: AtomicUsize,
    writer: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    /// Creates a new `RwLock`.
    pub const fn new(data: T) -> Self {
        RwLock {
            readers: AtomicUsize::new(0),
            writer: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires a read lock.
    pub fn read(&self) -> RwLockReadGuard<T> {
        while self.writer.load(Ordering::SeqCst) {
            // Busy-wait while a writer holds the lock
            // core::hint::spin_loop();
        }
        self.readers.fetch_add(1, Ordering::SeqCst);

        RwLockReadGuard { lock: self }
    }

    /// Acquires a write lock.
    pub fn write(&self) -> RwLockWriteGuard<T> {
        while self.readers.load(Ordering::SeqCst) > 0 || self.writer.load(Ordering::SeqCst) {
            // Busy-wait while readers exist or a writer holds the lock
            // core::hint::spin_loop();
        }
        self.writer.store(true, Ordering::SeqCst);

        RwLockWriteGuard { lock: self }
    }

    fn release_read(&self) {
        self.readers.fetch_sub(1, Ordering::SeqCst);
    }

    fn release_write(&self) {
        self.writer.store(false, Ordering::SeqCst);
    }
}

/// RAII guard for read access.
pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.release_read();
    }
}

impl<T> core::ops::Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

/// RAII guard for write access.
pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.release_write();
    }
}

impl<T> core::ops::Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> core::ops::DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}
*/
