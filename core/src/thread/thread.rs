//! Thread

use alloc::{
    ffi::NulError,
    string::{String, ToString},
};
use core::ffi::CStr;
use flagset::{FlagSet, flags};
use thiserror::Error;
use wut_sys as sys;

flags! {
    #[derive(Default)]
    pub enum ThreadAttribute: sys::OSThreadAttributes {
        Cpu0 = sys::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_AFFINITY_CPU0 as sys::OSThreadAttributes,
        Cpu1 = sys::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_AFFINITY_CPU1 as sys::OSThreadAttributes,
        Cpu2 = sys::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_AFFINITY_CPU2 as sys::OSThreadAttributes,
        #[default]
        CpuAny = sys::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_AFFINITY_ANY as sys::OSThreadAttributes,
        Detached = sys::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_DETACHED as sys::OSThreadAttributes,
        StackUsage = sys::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_STACK_USAGE as sys::OSThreadAttributes,
        Unknown = sys::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_UNKNOWN as sys::OSThreadAttributes
    }

    pub enum ThreadState: sys::OSThreadState {
        // None = sys::OS_THREAD_STATE_NONE as u8,
        /// Thread is ready to run.
        Ready = sys::OS_THREAD_STATE::OS_THREAD_STATE_READY as sys::OSThreadState,
        /// Thread is running.
        Running = sys::OS_THREAD_STATE::OS_THREAD_STATE_RUNNING as sys::OSThreadState,
        /// Thread is waiting, i.e. on a mutex
        Waiting = sys::OS_THREAD_STATE::OS_THREAD_STATE_WAITING as sys::OSThreadState,
        /// Thread is about to terminate.
        Moribund = sys::OS_THREAD_STATE::OS_THREAD_STATE_MORIBUND as sys::OSThreadState
    }
}

/// Thread
///
/// Note: `Thread` is not the owner of any data but a stack pointer. Dropping just drops the pointer and no underlying data.
///
#[derive(Debug, Copy, Clone)]
pub struct Thread(*mut sys::OSThread);

/// I think I should split this up as the errors don't really correlate
/// But keep this for now
#[derive(Debug, Error)]
pub enum ThreadError {
    #[error("")]
    AllocationFailed,
    #[error("")]
    ThreadCreationFailed,
    #[error("")]
    NullPointer,
    #[error("")]
    InternalZeroByte(#[from] NulError),
}

impl From<*mut sys::OSThread> for Thread {
    fn from(value: *mut sys::OSThread) -> Self {
        Self(value)
    }
}

impl Thread {
    pub fn name(&self) -> Result<String, ThreadError> {
        unsafe {
            let char_p = (*self.0).name;
            if char_p.is_null() {
                Err(ThreadError::NullPointer)
            } else {
                Ok(CStr::from_ptr(char_p).to_string_lossy().to_string())
            }
        }
    }

    pub fn attributes(&self) -> FlagSet<ThreadAttribute> {
        unsafe { FlagSet::<ThreadAttribute>::new_truncated((*self.0).attr) }
    }

    pub fn state(&self) -> FlagSet<ThreadState> {
        unsafe { FlagSet::<ThreadState>::new_truncated((*self.0).state) }
    }

    pub fn priority(&self) -> i32 {
        unsafe { (*self.0).basePriority }
    }

    pub fn park(&self) {
        unsafe { sys::OSSuspendThread(self.0) };
    }

    pub fn unpark(&self) {
        unsafe {
            sys::OSContinueThread(self.0);
        }
    }

    pub fn try_unpark(&self) {
        todo!()
    }

    pub fn cancel(&self) {
        unsafe {
            sys::OSCancelThread(self.0);
        }
    }

    pub fn running(&self) -> bool {
        unsafe {
            sys::OSTestThreadCancel();
        }
        self.state().contains(ThreadState::Running)
    }

    pub unsafe fn raw(&self) -> &mut sys::OSThread {
        unsafe { &mut (*self.0) }
    }
}

unsafe impl Sync for Thread {}
unsafe impl Send for Thread {}
