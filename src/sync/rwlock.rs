use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct RwLock<T> {
    readers: AtomicUsize,
    writer: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    /// Creates a new `RwLock`.
    pub const fn new(data: T) -> Self {
        RwLock {
            readers: AtomicUsize::new(0),
            writer: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires a read lock.
    pub fn read(&self) -> RwLockReadGuard<T> {
        loop {
            // Wait until there's no writer
            while self.writer.load(Ordering::Acquire) > 0 {}

            // Increment the reader count
            self.readers.fetch_add(1, Ordering::Acquire);

            // Double-check if a writer was added while we incremented readers
            if self.writer.load(Ordering::Acquire) == 0 {
                break;
            }

            // If there’s a writer, decrement readers and retry
            self.readers.fetch_sub(1, Ordering::Release);
        }

        RwLockReadGuard { lock: self }
    }

    /// Acquires a write lock.
    pub fn write(&self) -> RwLockWriteGuard<T> {
        // Wait until there's no writer
        while self.writer.fetch_add(1, Ordering::Acquire) > 0 {
            self.writer.fetch_sub(1, Ordering::Release);
        }

        // Wait until there are no readers
        while self.readers.load(Ordering::Acquire) > 0 {}

        RwLockWriteGuard { lock: self }
    }

    fn release_read(&self) {
        self.readers.fetch_sub(1, Ordering::Release);
    }

    fn release_write(&self) {
        self.writer.fetch_sub(1, Ordering::Release);
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
