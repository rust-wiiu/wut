#![no_std]

extern crate alloc;

use core::{alloc::GlobalAlloc, ffi, panic::PanicInfo};

pub mod bindings;
pub mod logging;
pub mod process;

#[cfg(feature = "default_panic_handler")]
#[panic_handler]
fn panic_handler(_panic_info: &PanicInfo) -> ! {
    unsafe {
        bindings::OSFatal("PANIC!".as_ptr() as *const i8);
    }
    loop {}
}

pub struct WiiUAllocator;

unsafe impl GlobalAlloc for WiiUAllocator {
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

pub mod prelude {
    pub use crate::{print, println};

    use crate::WiiUAllocator;
    #[global_allocator]
    static GLOBAL_ALLOCATOR: WiiUAllocator = WiiUAllocator;
}
