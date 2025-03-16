use crate::{bindings as c_wut, GLOBAL_ALLOCATOR};
use core::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
};

pub struct Semaphore(*mut c_wut::OSSemaphore);

impl Semaphore {
    pub fn new(initial: i32) -> Self {
        let layout = Layout::new::<c_wut::OSSemaphore>();
        let semaphore = unsafe { GLOBAL_ALLOCATOR.alloc_zeroed(layout) } as *mut c_wut::OSSemaphore;

        crate::println!("{:?}", unsafe { *semaphore }.tag);

        unsafe {
            c_wut::OSInitSemaphore(semaphore, initial);
        }

        crate::println!("{:?}", unsafe { *semaphore }.tag);

        Self(semaphore)
    }

    pub fn test(&self) -> i32 {
        unsafe { *self.0 }.count
    }

    #[inline]
    pub fn count(&self) -> i32 {
        unsafe { c_wut::OSGetSemaphoreCount(self.0) }
    }

    #[inline]
    pub fn signal(&self) -> i32 {
        unsafe { c_wut::OSSignalSemaphore(self.0) }
    }

    #[inline]
    pub fn wait(&self) -> i32 {
        unsafe { c_wut::OSWaitSemaphore(self.0) }
    }

    #[inline]
    pub fn try_wait(&self) -> i32 {
        unsafe { c_wut::OSTryWaitSemaphore(self.0) }
    }
}

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}
