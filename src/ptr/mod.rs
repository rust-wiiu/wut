use crate::bindings as c_wut;

/// Check if pointer is inside virtual memory bounds.
///
/// Useful when loading a pointer from memory.
#[inline]
pub fn is_valid<T>(ptr: *const T) -> bool {
    unsafe { c_wut::OSIsAddressValid(ptr as u32) == 1 }
}
