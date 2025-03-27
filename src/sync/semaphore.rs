use crate::bindings as c_wut;
use core::cell::UnsafeCell;

pub struct Semaphore(UnsafeCell<c_wut::OSSemaphore>);

impl Semaphore {
    pub fn new(initial: i32) -> Self {
        let mut semaphore = c_wut::OSSemaphore::default();

        unsafe {
            c_wut::OSInitSemaphore(&mut semaphore, initial);
        }

        Self(UnsafeCell::new(semaphore))
    }

    #[inline]
    fn inner(&self) -> &mut c_wut::OSSemaphore {
        unsafe { &mut *self.0.get() }
    }

    #[inline]
    pub fn count(&self) -> i32 {
        unsafe { c_wut::OSGetSemaphoreCount(self.inner()) }
    }

    #[inline]
    pub fn signal(&self) -> i32 {
        unsafe { c_wut::OSSignalSemaphore(self.inner()) }
    }

    #[inline]
    pub fn wait(&self) -> i32 {
        unsafe { c_wut::OSWaitSemaphore(self.inner()) }
    }

    #[inline]
    pub fn try_wait(&self) -> i32 {
        unsafe { c_wut::OSTryWaitSemaphore(self.inner()) }
    }
}

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}
