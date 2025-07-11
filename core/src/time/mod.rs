//! Temporal quantification.

pub use core::time::*;

mod datetime;
pub use datetime::DateTime;

use crate::thread::ticks::{nanos_to_ticks, ticks_to_nanos};
use core::ops;
use wut_sys as sys;

/// A measurement of the system clock, useful for interacting with external entities such as the file system or other processes.
///
/// Since the system clock can be arbitrarily set, `SystemTime` measurements do not have a real-time meaning. Earlier timestamps could have occurred at a later real time, and identical timestamps could have occurred at different real times. However, it is the base for time measurements available on the Wii U and can be considered the ground truth.
///
/// Unlike most systems that use the Unix Epoch as their reference point, this clock is anchored to `2000-01-01T00:00:00Z`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SystemTime(sys::OSTime);

impl SystemTime {
    pub fn now() -> Self {
        Self(unsafe { sys::OSGetTime() })
    }

    pub fn elapsed(&self) -> Result<Duration, ()> {
        let delta = SystemTime::now().0 - self.0;
        if delta >= 0 {
            Ok(Duration::from(SystemTime(delta)))
        } else {
            Err(())
        }
    }

    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration, ()> {
        let delta = self.0 - earlier.0;
        if delta >= 0 {
            Ok(Duration::from(SystemTime(delta)))
        } else {
            Err(())
        }
    }
}

impl ops::Add<Duration> for SystemTime {
    type Output = SystemTime;
    fn add(self, rhs: Duration) -> Self::Output {
        SystemTime(self.0 + Into::<SystemTime>::into(rhs).0)
    }
}

impl ops::AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += Into::<SystemTime>::into(rhs).0
    }
}

impl ops::Sub<Duration> for SystemTime {
    type Output = SystemTime;
    fn sub(self, rhs: Duration) -> Self::Output {
        SystemTime(self.0 - Into::<SystemTime>::into(rhs).0)
    }
}

impl ops::SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 -= Into::<SystemTime>::into(rhs).0
    }
}

impl From<sys::OSTime> for SystemTime {
    fn from(value: sys::OSTime) -> Self {
        Self(value)
    }
}

impl Into<sys::OSTime> for SystemTime {
    fn into(self) -> sys::OSTime {
        self.0
    }
}

impl From<SystemTime> for Duration {
    fn from(value: SystemTime) -> Self {
        Self::from_nanos(ticks_to_nanos(value.0 as u64))
    }
}

impl Into<SystemTime> for Duration {
    fn into(self) -> SystemTime {
        SystemTime(nanos_to_ticks(self.as_nanos() as u64) as i64)
    }
}
