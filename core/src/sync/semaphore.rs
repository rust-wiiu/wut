use core::cell::UnsafeCell;
use wut_sys as sys;

pub struct Semaphore(UnsafeCell<sys::OSSemaphore>);

impl Semaphore {
    pub fn new(initial: i32) -> Self {
        let mut semaphore = sys::OSSemaphore::default();

        unsafe {
            sys::OSInitSemaphore(&mut semaphore, initial);
        }

        Self(UnsafeCell::new(semaphore))
    }

    #[inline]
    fn inner(&self) -> &mut sys::OSSemaphore {
        unsafe { &mut *self.0.get() }
    }

    #[inline]
    pub fn count(&self) -> i32 {
        unsafe { sys::OSGetSemaphoreCount(self.inner()) }
    }

    #[inline]
    pub fn signal(&self) -> i32 {
        unsafe { sys::OSSignalSemaphore(self.inner()) }
    }

    #[inline]
    pub fn wait(&self) -> i32 {
        unsafe { sys::OSWaitSemaphore(self.inner()) }
    }

    #[inline]
    pub fn try_wait(&self) -> i32 {
        unsafe { sys::OSTryWaitSemaphore(self.inner()) }
    }
}

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}
