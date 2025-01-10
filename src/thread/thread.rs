//! Thread

use crate::bindings as c_wut;
use alloc::{
    ffi::{CString, IntoStringError},
    string::String,
};
use flagset::{flags, FlagSet};

flags! {
    #[derive(Default)]
    pub enum ThreadAttribute: c_wut::OSThreadAttributes {
        Cpu0 = c_wut::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_AFFINITY_CPU0 as c_wut::OSThreadAttributes,
        Cpu1 = c_wut::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_AFFINITY_CPU1 as c_wut::OSThreadAttributes,
        Cpu2 = c_wut::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_AFFINITY_CPU2 as c_wut::OSThreadAttributes,
        #[default]
        CpuAny = c_wut::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_AFFINITY_ANY as c_wut::OSThreadAttributes,
        Detached = c_wut::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_DETACHED as c_wut::OSThreadAttributes,
        StackUsage = c_wut::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_STACK_USAGE as c_wut::OSThreadAttributes,
        Unknown = c_wut::OS_THREAD_ATTRIB::OS_THREAD_ATTRIB_UNKNOWN as c_wut::OSThreadAttributes
    }

    pub enum ThreadState: c_wut::OSThreadState {
        // None = c_wut::OS_THREAD_STATE_NONE as u8,
        /// Thread is ready to run.
        Ready = c_wut::OS_THREAD_STATE::OS_THREAD_STATE_READY as c_wut::OSThreadState,
        /// Thread is running.
        Running = c_wut::OS_THREAD_STATE::OS_THREAD_STATE_RUNNING as c_wut::OSThreadState,
        /// Thread is waiting, i.e. on a mutex
        Waiting = c_wut::OS_THREAD_STATE::OS_THREAD_STATE_WAITING as c_wut::OSThreadState,
        /// Thread is about to terminate.
        Moribund = c_wut::OS_THREAD_STATE::OS_THREAD_STATE_MORIBUND as c_wut::OSThreadState
    }
}

pub struct Thread(*mut c_wut::OSThread);

/// I think I should split this up as the errors don't really correlate
/// But keep this for now
#[derive(Debug)]
pub enum ThreadError {
    AllocationFailed,
    ThreadCreationFailed,
    NullPointer,
    InvalidUtf8(IntoStringError),
}

impl Thread {
    pub fn new(thread: *mut c_wut::OSThread) -> Self {
        Self(thread)
    }

    pub fn name(&self) -> Result<String, ThreadError> {
        // unsafe { CString::from_raw((*self.0).name as *mut i8).into_string() }
        unsafe {
            let char_p = (*self.0).name;
            if char_p.is_null() {
                Err(ThreadError::NullPointer)
            } else {
                match CString::from_raw(char_p as *mut i8).into_string() {
                    Ok(str) => Ok(str),
                    Err(err) => Err(ThreadError::InvalidUtf8(err)),
                }
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
        unsafe { c_wut::OSSuspendThread(self.0) };
    }

    pub fn unpark(&self) {
        unsafe {
            c_wut::OSContinueThread(self.0);
        }
    }

    pub fn try_unpark(&self) {
        todo!()
    }

    pub unsafe fn raw(&self) -> &mut c_wut::OSThread {
        &mut (*self.0)
    }
}
