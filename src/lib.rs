#![no_std]

extern crate alloc;
extern crate flagset;

use core::ffi;

pub mod bindings;
pub mod io;
mod macros;
pub mod process;
pub mod sync;
pub mod thread;
pub mod time;

pub mod prelude {
    pub use crate::println;
}

#[cfg(feature = "default_panic_handler")]
#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        bindings::OSFatal(c"PANIC!".as_ptr());
    }
    loop {}
}

pub struct WiiUAllocator;

unsafe impl core::alloc::GlobalAlloc for WiiUAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        bindings::memalign(
            layout.align().max(16) as ffi::c_ulong,
            layout.size() as ffi::c_ulong,
        ) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        bindings::free(ptr.cast::<ffi::c_void>());
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: WiiUAllocator = WiiUAllocator;
