use crate::bindings as c_wut;

/// Get the title ID of the current foreground application.
///
/// Lists of title ID are available online.
pub fn current_title() -> u64 {
    unsafe { c_wut::OSGetTitleID() }
}
