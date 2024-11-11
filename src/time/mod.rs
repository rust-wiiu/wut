// time

mod datetime;
pub use datetime::DateTime;

use crate::bindings as c_wut;

/// SystemTime in ... ? Tbh, I'm not sure about the unit
pub struct SystemTime(i64);

impl SystemTime {
    pub fn now() -> Self {
        unsafe { Self(c_wut::OSGetTime()) }
    }
}
