use crate::{bindings as c_wut, GLOBAL_ALLOCATOR};

pub mod builder;
pub mod thread;
mod ticks;

use alloc::{self, boxed::Box};
pub use builder::Builder;
use core::{alloc::GlobalAlloc, ffi, time::Duration};
pub use thread::Thread;
use thread::ThreadAttribute;

pub struct JoinHandle {
    thread: *mut c_wut::OSThread,
}

impl JoinHandle {
    pub fn join() /* -> Result<T>*/ {}
}

/*
pub fn current() -> Thread {
    unsafe {
        let t = c_wut::OSGetCurrentThread();
        Thread::from(*t)
    }
}

pub enum CpuCore {
    Core0,
    Core1,
    Core2,
}

impl Into<u32> for CpuCore {
    fn into(self) -> u32 {
        match self {
            CpuCore::Core0 => 0,
            CpuCore::Core1 => 1,
            CpuCore::Core2 => 2,
        }
    }
}

pub fn default_thread(core: CpuCore) -> Thread {
    unsafe { Thread::from(*c_wut::OSGetDefaultThread(core.into())) }
}
*/

pub fn num_threads() -> i32 {
    unsafe { c_wut::OSCheckActiveThreads() }
}

//

pub fn spawn<F, T>(f: F)
/* -> ... */
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    // Builder::default().spawn(f);
}

//

pub fn temp<F>(f: F) -> Result<Option<JoinHandle>, ()>
where
    F: FnOnce() + 'static,
{
    use alloc::alloc::Layout;

    let mut thread = c_wut::OSThread::default();

    let layout = Layout::from_size_align(10 * 1028, 16).unwrap();
    let stack = unsafe { GLOBAL_ALLOCATOR.alloc_zeroed(layout) };

    let cb: Box<Box<dyn FnOnce()>> = Box::new(Box::new(f));

    crate::println!("alloc stack: {:x}", stack as usize);

    unsafe {
        c_wut::OSCreateThread(
            &mut thread,
            Some(thread_entry),
            0,
            Box::into_raw(cb) as *mut _,
            (stack as usize + layout.size()) as *mut _,
            layout.size() as u32,
            15,
            ThreadAttribute::CpuAny as u8,
        );
        c_wut::OSResumeThread(&mut thread);

        c_wut::OSSetThreadDeallocator(&mut thread, Some(thread_dealloc));

        let mut _res = 0;
        c_wut::OSJoinThread(&mut thread, &mut _res);
    }

    Err(())
}

unsafe extern "C" fn thread_entry(_argc: ffi::c_int, argv: *mut *const ffi::c_char) -> ffi::c_int {
    let closure = unsafe { Box::from_raw(argv as *mut Box<dyn FnOnce()>) };
    closure();
    0
}

unsafe extern "C" fn thread_dealloc(thread: *mut c_wut::OSThread, stack: *mut ffi::c_void) {
    crate::println!("dealloc stack: {:x}", stack as usize);

    // let _ = GLOBAL_ALLOCATOR.dealloc(stack, Layout::from_size_align(10 * 1028, 16).unwrap());
}

pub fn sleep(duration: Duration) {
    unsafe {
        c_wut::OSSleepTicks(ticks::nanos_to_ticks(duration.as_nanos() as u64) as i64);
    }
}
