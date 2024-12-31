// time

pub use core::time::*;

mod datetime;
pub use datetime::DateTime;

use crate::bindings as c_wut;

/// A measurement of the system clock, useful for interacting with external entities such as the file system or other processes.
///
/// Since the system clock can be arbitrarily set, `SystemTime` measurements do not have a real-time meaning. Earlier timestamps could have occurred at a later real time, and identical timestamps could have occurred at different real times. However, it is the base for time measurements available on the Wii U and can be considered the ground truth.
///
/// Unlike most systems that use the Unix Epoch as their reference point, this clock is anchored to `2000-01-01T00:00:00Z`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SystemTime(c_wut::OSTime);

impl SystemTime {
    pub fn now() -> Self {
        Self(unsafe { c_wut::OSGetTime() })
    }
}

impl From<c_wut::OSTime> for SystemTime {
    fn from(value: c_wut::OSTime) -> Self {
        Self(value)
    }
}
