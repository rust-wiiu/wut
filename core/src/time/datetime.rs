// datetime

use crate::time::SystemTime;
use core::{cmp::Ordering, fmt};
use wut_sys as sys;

/// WiiU local system datetime.
///
/// Does not care about UTC, timezones or leap seconds (?).
#[derive(Debug, PartialEq, Eq)]
pub struct DateTime {
    /// Seconds after the minute. The range is 0-59.
    pub sec: i32,
    /// Minutes after the hour. The range is 0-59.
    pub min: i32,
    /// Hours since midnight. The range is 0-23.
    pub hour: i32,
    /// Day of the month. The range is 1-31.
    pub day: i32,
    /// Month since January. The range is 0-11.
    pub month: i32,
    /// Years in AD. The range is 1-....
    pub year: i32,
    /// Days since Sunday. The range is 0-6.
    pub weekday: i32,
    /// Days since January 1. The range is 0-365.
    pub yearday: i32,
    /// Milliseconds after the second. The range is 0-999.
    pub millisec: i32,
    /// Microseconds after the millisecond. The range is 0-999.
    pub microsec: i32,
}

impl DateTime {
    pub fn now() -> Self {
        Self::from(SystemTime::now())
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self {
            sec: 0,
            min: 0,
            hour: 0,
            day: 1,
            month: 0,
            year: 1,
            weekday: 0,
            yearday: 0,
            millisec: 0,
            microsec: 0,
        }
    }
}

impl From<sys::OSCalendarTime> for DateTime {
    fn from(value: sys::OSCalendarTime) -> Self {
        Self {
            sec: value.tm_sec,
            min: value.tm_min,
            hour: value.tm_hour,
            day: value.tm_mday,
            month: value.tm_mon,
            year: value.tm_year,
            weekday: value.tm_wday,
            yearday: value.tm_yday,
            millisec: value.tm_msec,
            microsec: value.tm_usec,
        }
    }
}

impl From<sys::OSTime> for DateTime {
    fn from(value: sys::OSTime) -> Self {
        let mut cal = sys::OSCalendarTime::default();
        unsafe {
            sys::OSTicksToCalendarTime(value, &mut cal);
        }
        Self::from(cal)
    }
}

impl From<SystemTime> for DateTime {
    fn from(value: SystemTime) -> Self {
        Self::from(value.0)
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then_with(|| self.month.cmp(&other.month))
            .then_with(|| self.day.cmp(&other.day))
            .then_with(|| self.hour.cmp(&other.hour))
            .then_with(|| self.min.cmp(&other.min))
            .then_with(|| self.sec.cmp(&other.sec))
            .then_with(|| self.millisec.cmp(&other.millisec))
            .then_with(|| self.microsec.cmp(&other.microsec))
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}/{:02}/{:02} {:02}:{:02}:{:02}",
            self.year,
            self.month + 1,
            self.day,
            self.hour,
            self.min,
            self.sec
        )
    }
}
