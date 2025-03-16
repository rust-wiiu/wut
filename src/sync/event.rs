use crate::{
    bindings as c_wut,
    time::{Duration, SystemTime},
};
use core::cell::UnsafeCell;

/// Auto events are automatically reset after signaling.
pub struct AutoEvent(UnsafeCell<c_wut::OSEvent>);

/// Manual events need to be manually reset after signaling.
pub struct ManualEvent(UnsafeCell<c_wut::OSEvent>);

impl AutoEvent {
    /// Create new auto event.
    pub fn new() -> Self {
        let mut event = c_wut::OSEvent::default();
        unsafe {
            c_wut::OSInitEvent(&mut event, 0, c_wut::OSEventMode::OS_EVENT_MODE_AUTO);
        }
        Self(UnsafeCell::new(event))
    }

    fn inner(&self) -> &mut c_wut::OSEvent {
        unsafe { &mut *self.0.get() }
    }

    /// Signal that event is active. Activate one thread and reset immediately after.
    pub fn signal(&self) {
        unsafe {
            c_wut::OSSignalEvent(self.inner());
        }
    }

    /// Signal that event is active. Activate all waiting threads and reset afterwards.
    pub fn signal_all(&self) {
        unsafe {
            c_wut::OSSignalEventAll(self.inner());
        }
    }

    /// Wait until event is active.
    pub fn wait(&self) {
        unsafe {
            c_wut::OSWaitEvent(self.inner());
        }
    }

    /// Wait until event is active with timeout. Return `true` on timeout.
    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        unsafe {
            c_wut::OSWaitEventWithTimeout(self.inner(), Into::<SystemTime>::into(timeout).into())
                == 1
        }
    }
}

impl ManualEvent {
    /// Create new manual event.
    pub fn new() -> Self {
        let mut event = c_wut::OSEvent::default();
        unsafe {
            c_wut::OSInitEvent(&mut event, 0, c_wut::OSEventMode::OS_EVENT_MODE_MANUAL);
        }
        Self(UnsafeCell::new(event))
    }

    fn inner(&self) -> &mut c_wut::OSEvent {
        unsafe { &mut *self.0.get() }
    }

    /// Signal that event is active.
    pub fn signal(&self) {
        unsafe {
            c_wut::OSSignalEvent(self.inner());
        }
    }

    /// Reset event to deactive state.
    pub fn reset(&self) {
        unsafe {
            c_wut::OSResetEvent(self.inner());
        }
    }

    /// Wait until event is active.
    pub fn wait(&self) {
        unsafe {
            c_wut::OSWaitEvent(self.inner());
        }
    }

    /// Wait until event is active with timeout. Return `true` on timeout.
    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        unsafe {
            c_wut::OSWaitEventWithTimeout(self.inner(), Into::<SystemTime>::into(timeout).into())
                == 1
        }
    }
}

// Safety implementations
unsafe impl Send for AutoEvent {}
unsafe impl Sync for AutoEvent {}

unsafe impl Send for ManualEvent {}
unsafe impl Sync for ManualEvent {}
