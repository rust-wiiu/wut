use crate::{
    bindings as c_wut,
    thread::{
        thread::{Thread, ThreadAttribute, ThreadError},
        JoinHandle,
    },
    GLOBAL_ALLOCATOR,
};
use alloc::{alloc::Layout, boxed::Box, string::String};
use core::{alloc::GlobalAlloc, ffi};
use flagset::FlagSet;

pub struct Builder {
    name: Option<String>,
    attribute: FlagSet<ThreadAttribute>,
    priority: i32,
    stack_size: usize,
    quantum: u32,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            name: None,
            attribute: ThreadAttribute::default().into(),
            priority: 15,
            stack_size: 128 * 1024,
            quantum: 100_000, // ticks
        }
    }
}

impl Builder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn attribute(mut self, attributes: impl Into<FlagSet<ThreadAttribute>>) -> Self {
        self.attribute = attributes.into();
        self
    }

    /// Set thread priority.
    /// Used by scheduler.
    ///
    /// `0` is highest priority, `31` is lowest priority.
    ///
    ///
    pub fn priority(mut self, priority: impl Into<i32>) -> Self {
        self.priority = priority.into();
        self
    }

    /// Set thread stack size (bytes).
    pub fn stack_size(mut self, stack_size: impl Into<usize>) -> Self {
        self.stack_size = stack_size.into();
        self
    }

    /// Set a thread's run quantum.
    ///
    /// This is the maximum amount of time the thread can run for before being forced to yield.
    pub fn quantum(mut self, quantum: u32) -> Self {
        self.quantum = quantum;
        self
    }

    pub fn spawn<F>(self, f: F) -> Result<JoinHandle, ThreadError>
    where
        F: FnOnce() + Send + 'static,
    {
        unsafe { self.spawn_unchecked(f) }
    }

    pub unsafe fn spawn_unchecked<F>(self, f: F) -> Result<JoinHandle, ThreadError>
    where
        F: FnOnce() + Send,
    {
        Ok(JoinHandle::new(unsafe { self.spawn_unchecked_(f)? }))
    }

    /// There 100% is a better and more rusty way of doing this, but it works for now
    unsafe fn spawn_unchecked_<F>(self, f: F) -> Result<Thread, ThreadError>
    where
        F: FnOnce() + Send,
    {
        let layout = Layout::new::<c_wut::OSThread>();
        let thread = GLOBAL_ALLOCATOR.alloc_zeroed(layout) as *mut c_wut::OSThread;
        if thread.is_null() {
            return Err(ThreadError::AllocationFailed);
        }

        let layout = Layout::from_size_align(self.stack_size, 16).unwrap();
        let stack = GLOBAL_ALLOCATOR.alloc_zeroed(layout);
        if stack.is_null() {
            return Err(ThreadError::AllocationFailed);
        }

        let f: Box<Box<dyn FnOnce()>> = Box::new(Box::new(f));

        if c_wut::OSCreateThread(
            thread,
            Some(thread_entry),
            0,
            Box::into_raw(f) as *mut _,
            (stack as usize + layout.size()) as *mut _,
            layout.size() as u32,
            self.priority,
            self.attribute.bits(),
        ) == 0
        {
            return Err(ThreadError::ThreadCreationFailed);
        }

        if let Some(name) = self.name {
            let layout = Layout::array::<u8>(name.len() + 1).unwrap();
            let thread_name = GLOBAL_ALLOCATOR.alloc_zeroed(layout);

            core::ptr::copy_nonoverlapping(name.as_ptr(), thread_name, name.len());

            c_wut::OSSetThreadName(thread, thread_name as *const _);
        }
        c_wut::OSSetThreadDeallocator(thread, Some(thread_dealloc));
        c_wut::OSSetThreadRunQuantum(thread, self.quantum);

        c_wut::OSContinueThread(thread);

        Ok(Thread::from(thread))
    }
}

/// Thread entry point
///
/// Abuses `argv` to pass `FnOnce()` into thread
unsafe extern "C" fn thread_entry(_argc: ffi::c_int, argv: *mut *const ffi::c_char) -> ffi::c_int {
    let closure = unsafe { Box::from_raw(argv as *mut Box<dyn FnOnce()>) };
    closure();
    0
}

unsafe extern "C" fn thread_dealloc(thread: *mut c_wut::OSThread, stack: *mut ffi::c_void) {
    // let stack_size = (*thread).stackStart as usize - (*thread).stackEnd as usize;
    // let layout = Layout::from_size_align(stack_size, 16).unwrap();

    // likely misuse but since I ignore layout in dealloc its should be fine
    // I could create a CStr and get the length from there but ig not needed
    let layout = Layout::new::<()>();
    GLOBAL_ALLOCATOR.dealloc(stack as *mut u8, layout);

    // likely misuse but since I ignore layout in dealloc its should be fine
    let layout = Layout::new::<()>();
    GLOBAL_ALLOCATOR.dealloc((*thread).name as *mut u8, layout);

    // let layout = Layout::new::<c_wut::OSThread>();
    let layout = Layout::new::<()>();
    GLOBAL_ALLOCATOR.dealloc(thread as *mut u8, layout);
}
