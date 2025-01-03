#![no_std]

pub extern crate alloc;
extern crate flagset;
extern crate thiserror;

pub mod bindings;
pub mod env;
pub mod fs;
pub mod gamepad;
pub mod io;
mod macros;
pub mod net;
pub mod path;
pub mod process;
pub mod rrc;
pub mod screen;
pub mod sync;
pub mod thread;
pub mod time;

use core::{alloc::GlobalAlloc, ffi};

pub mod prelude {
    pub use crate::println;
    pub use alloc::format;
}

#[cfg(feature = "default_panic_handler")]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    use crate::{screen, time};
    use alloc::format;

    let msg = if let Some(location) = info.location() {
        format!(
            "Panic!\n\n{}\n\n[{} : Ln {}, Col {}]",
            info.message(),
            location.file(),
            location.line(),
            location.column()
        )
    } else {
        format!("Panic!\n\n{}", info.message())
    };

    let tv = screen::tv();
    let drc = screen::drc();
    tv.enable();
    drc.enable();

    for i in (0..=15).rev() {
        // Clear the screens
        tv.fill(screen::Color::black());
        drc.fill(screen::Color::black());

        // Display the message on both screens
        tv.text(&msg, 0.5, 0.30, screen::TextAlign::Center);
        drc.text(&msg, 0.5, 0.30, screen::TextAlign::Center);

        // Render the progress bar
        let timer = format!("Restarting console in {}", i - 1);
        tv.text(&timer, 0.5, 0.8, screen::TextAlign::Center);
        drc.text(&timer, 0.5, 0.8, screen::TextAlign::Center);

        // Update screens
        tv.update();
        drc.update();

        thread::sleep(time::Duration::from_secs(1));
    }

    process::to_menu()
}

pub struct WiiUAllocator;

unsafe impl GlobalAlloc for WiiUAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        // bindings::MEMallocFromDe();
        bindings::MEMAllocFromDefaultHeapEx.unwrap()(layout.size() as u32, layout.align() as i32)
            as *mut u8

        // bindings::MEMFreeToDefaultHeap(core::ptr::null());

        // bindings::memalign(
        //     layout.align() as ffi::c_ulong,
        //     layout.size() as ffi::c_ulong,
        // )
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        // bindings::free(ptr.cast::<ffi::c_void>());
        bindings::MEMFreeToDefaultHeap.unwrap()(ptr as *mut ffi::c_void);
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: WiiUAllocator = WiiUAllocator;
