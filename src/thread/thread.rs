/// twinny!
use crate::bindings::*;
use alloc::{
    ffi::{CString, IntoStringError},
    string::String,
};
use core::cell::UnsafeCell;
use flagset::{flags, FlagSet};

flags! {
    pub enum ThreadAttribute: u8 {
        Cpu0 = OS_THREAD_ATTRIB_AFFINITY_CPU0 as u8,
        Cpu1 = OS_THREAD_ATTRIB_AFFINITY_CPU1 as u8,
        Cpu2 = OS_THREAD_ATTRIB_AFFINITY_CPU2 as u8,
        CpuAny = OS_THREAD_ATTRIB_AFFINITY_ANY as u8,
        Detached = OS_THREAD_ATTRIB_DETACHED as u8,
        StackUsage = OS_THREAD_ATTRIB_STACK_USAGE as u8,
        Unknown = OS_THREAD_ATTRIB_UNKNOWN as u8
    }

    pub enum ThreadState: u8 {
        None = OS_THREAD_STATE_NONE as u8,
        Ready = OS_THREAD_STATE_READY as u8,
        Running = OS_THREAD_STATE_RUNNING as u8,
        Waiting = OS_THREAD_STATE_WAITING as u8,
        Moribund = OS_THREAD_STATE_MORIBUND as u8
    }
}

pub struct Thread {
    os_thread: UnsafeCell<OSThread>,
}

impl Thread {
    pub fn name(&self) -> Result<String, IntoStringError> {
        unsafe { CString::from_raw((*self.os_thread.get()).name as *mut i8).into_string() }
    }

    pub fn id(&self) -> u16 {
        unsafe { (*self.os_thread.get()).id }
    }

    pub fn attributes(&self) -> FlagSet<ThreadAttribute> {
        unsafe { FlagSet::<ThreadAttribute>::new_truncated((*self.os_thread.get()).attr) }
    }

    pub fn state(&self) -> FlagSet<ThreadState> {
        unsafe { FlagSet::<ThreadState>::new_truncated((*self.os_thread.get()).state) }
    }

    pub fn priority(&self) -> i32 {
        unsafe { (*self.os_thread.get()).priority }
    }
}

impl From<OSThread> for Thread {
    fn from(value: OSThread) -> Self {
        Thread {
            os_thread: UnsafeCell::new(value),
        }
    }
}

impl Into<OSThread> for Thread {
    fn into(self) -> OSThread {
        unsafe { *self.os_thread.get() }
    }
}
