#![no_std]

extern crate alloc;
// extern crate compiler_builtins;
extern crate flagset;
extern crate thiserror;

pub mod bindings;
pub mod gamepad;
pub mod io;
mod macros;
pub mod net;
pub mod process;
pub mod screen;
pub mod sync;
pub mod thread;
pub mod time;

pub use core::alloc::{GlobalAlloc, Layout};
use core::ffi;

pub mod prelude {
    pub use crate::println;
}

#[cfg(feature = "default_panic_handler")]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        crate::println!(
            "Panic! - {} [{} : Ln {}, Col {}]",
            info.message(),
            location.file(),
            location.line(),
            location.column(),
        );
    } else {
        crate::println!("Panic! - {}", info.message());
    }

    loop {}
}

pub struct WiiUAllocator;

unsafe impl GlobalAlloc for WiiUAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        bindings::memalign(
            layout.align() as ffi::c_ulong,
            layout.size() as ffi::c_ulong,
        ) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        bindings::free(ptr.cast::<ffi::c_void>());
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: WiiUAllocator = WiiUAllocator;
