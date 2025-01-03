pub mod builder;
pub mod thread;
pub mod ticks;

use crate::bindings as c_wut;
pub use builder::Builder;
use core::time::Duration;
use thiserror::Error;
pub use thread::{Thread, ThreadError};

#[derive(Debug, Error)]
pub enum JoinError {
    #[error("This thread was detached")]
    Detached,
}

pub struct JoinHandle {
    thread: Thread,
}

unsafe impl Send for JoinHandle {}
unsafe impl Sync for JoinHandle {}

impl JoinHandle {
    pub fn new(thread: Thread) -> Self {
        Self { thread: thread }
    }

    pub fn thread(&self) -> &Thread {
        &self.thread
    }

    pub fn join(self) -> Result<i32, JoinError> {
        let mut result = 0;
        let detached = unsafe { c_wut::OSJoinThread(self.thread.raw(), &mut result) };

        match detached {
            0 => Err(JoinError::Detached),
            _ => Ok(result),
        }
    }
}

impl Drop for JoinHandle {
    fn drop(&mut self) {
        unsafe {
            c_wut::OSDetachThread(self.thread.raw());
        }
    }
}

pub fn current() -> Thread {
    Thread::new(unsafe { c_wut::OSGetCurrentThread() })
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
    Thread::new(unsafe { c_wut::OSGetDefaultThread(core.into()) })
}

pub fn num_threads() -> i32 {
    unsafe { c_wut::OSCheckActiveThreads() }
}

/// Exit the current thread with a exit code.
///
/// Be careful when calling this function in the main thread!
pub fn terminate(exit_code: i32) {
    unsafe {
        c_wut::OSExitThread(exit_code);
    }
}

//

pub fn spawn<F>(f: F) -> Result<JoinHandle, ThreadError>
where
    F: FnOnce() + Send + 'static,
{
    Builder::default().spawn(f)
}

pub fn sleep(duration: Duration) {
    unsafe {
        c_wut::OSSleepTicks(ticks::nanos_to_ticks(duration.as_nanos() as u64) as i64);
    }
}
