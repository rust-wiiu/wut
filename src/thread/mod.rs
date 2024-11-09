use crate::bindings::*;

pub mod builder;
pub mod thread;
pub use builder::Builder;
pub use thread::Thread;

pub fn current() -> Thread {
    unsafe {
        let t = OSGetCurrentThread();
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
    unsafe { Thread::from(*OSGetDefaultThread(core.into())) }
}

pub fn num_threads() -> i32 {
    unsafe { OSCheckActiveThreads() }
}

pub fn spawn() -> Thread {
    todo!()
}
