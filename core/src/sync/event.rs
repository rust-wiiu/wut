use crate::time::{Duration, SystemTime};
use core::cell::UnsafeCell;
use wut_sys as sys;

/// Auto events are automatically reset after signaling.
pub struct AutoEvent(UnsafeCell<sys::OSEvent>);

/// Manual events need to be manually reset after signaling.
pub struct ManualEvent(UnsafeCell<sys::OSEvent>);

impl AutoEvent {
    /// Create new auto event.
    pub fn new() -> Self {
        let mut event = sys::OSEvent::default();
        unsafe {
            sys::OSInitEvent(&mut event, 0, sys::OSEventMode::OS_EVENT_MODE_AUTO);
        }
        Self(UnsafeCell::new(event))
    }

    fn inner(&self) -> &mut sys::OSEvent {
        unsafe { &mut *self.0.get() }
    }

    /// Signal that event is active. Activate one thread and reset immediately after.
    pub fn signal(&self) {
        unsafe {
            sys::OSSignalEvent(self.inner());
        }
    }

    /// Signal that event is active. Activate all waiting threads and reset afterwards.
    pub fn signal_all(&self) {
        unsafe {
            sys::OSSignalEventAll(self.inner());
        }
    }

    /// Wait until event is active.
    pub fn wait(&self) {
        unsafe {
            sys::OSWaitEvent(self.inner());
        }
    }

    /// Wait until event is active with timeout. Return `true` on timeout.
    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        unsafe {
            sys::OSWaitEventWithTimeout(self.inner(), Into::<SystemTime>::into(timeout).into()) == 1
        }
    }
}

impl ManualEvent {
    /// Create new manual event.
    pub fn new() -> Self {
        let mut event = sys::OSEvent::default();
        unsafe {
            sys::OSInitEvent(&mut event, 0, sys::OSEventMode::OS_EVENT_MODE_MANUAL);
        }
        Self(UnsafeCell::new(event))
    }

    fn inner(&self) -> &mut sys::OSEvent {
        unsafe { &mut *self.0.get() }
    }

    /// Signal that event is active.
    pub fn signal(&self) {
        unsafe {
            sys::OSSignalEvent(self.inner());
        }
    }

    /// Reset event to deactive state.
    pub fn reset(&self) {
        unsafe {
            sys::OSResetEvent(self.inner());
        }
    }

    /// Wait until event is active.
    pub fn wait(&self) {
        unsafe {
            sys::OSWaitEvent(self.inner());
        }
    }

    /// Wait until event is active with timeout. Return `true` on timeout.
    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        unsafe {
            sys::OSWaitEventWithTimeout(self.inner(), Into::<SystemTime>::into(timeout).into()) == 1
        }
    }
}

// Safety implementations
unsafe impl Send for AutoEvent {}
unsafe impl Sync for AutoEvent {}

unsafe impl Send for ManualEvent {}
unsafe impl Sync for ManualEvent {}
