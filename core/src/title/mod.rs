//! Information about foreground application.

use wut_sys as sys;

/// Get the title ID of the current foreground application.
///
/// Lists of title ID are available online.
pub fn current_title() -> u64 {
    unsafe { sys::OSGetTitleID() }
}
