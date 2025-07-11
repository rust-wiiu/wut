// ticks

use wut_sys as sys;

#[inline]
pub fn timer_clock_speed() -> u32 {
    unsafe { (*sys::OSGetSystemInfo()).busClockSpeed / 4 }
}

#[inline]
pub fn nanos_to_ticks(nanos: u64) -> u64 {
    (nanos * timer_clock_speed() as u64 / 31_250) / 32_000
}

#[inline]
pub fn ticks_to_nanos(ticks: u64) -> u64 {
    (ticks * 32_000) / (timer_clock_speed() as u64 / 31_250)
}
