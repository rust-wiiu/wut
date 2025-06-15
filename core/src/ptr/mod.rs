//! Validate memory access by raw pointers.
//!
//! This module provides checks for validation of pointers. If non-statically known addresses are accessed via pointers it is highly recommended to check for validity to avoid system crashes.
//!
//! # Example
//!
//! ```rust
//! use wut::ptr::is_valid;
//!
//! let ptr = get_address_at_runtime() as *const u32;
//! if is_valid(ptr) {
//!     let value = use_pointer(ptr);
//! }
//! ```

use wut_sys as sys;

pub use core::ptr::*;

/// Check if pointer is inside virtual memory bounds.
///
/// Useful when loading a pointer from memory.
#[inline]
pub fn is_valid<T>(ptr: *const T) -> bool {
    unsafe { sys::OSIsAddressValid(ptr as u32) == 1 }
}
