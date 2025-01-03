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
    loop {}

    /*
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
    */

    /*
    use alloc::string::ToString;

    let (file, line) = if let Some(location) = info.location() {
        (
            alloc::ffi::CString::new(location.file()).unwrap(),
            location.line(),
        )
    } else {
        (alloc::ffi::CString::from(c"<unknown>"), 0)
    };
    let msg = alloc::ffi::CString::new(info.message().to_string()).unwrap();

    unsafe {
        crate::bindings::OSPanic(file.as_ptr(), line, msg.as_ptr());
    }
    crate::thread::sleep(core::time::Duration::from_secs(5));

    loop {}
    */

    /*
    use crate::{screen, thread, time};
    use alloc::format;

    let msg = if let Some(location) = info.location() {
        format!(
            "Panic!\n{}\n[{} : Ln {}, Col {}]",
            info.message(),
            location.file(),
            location.line(),
            location.column()
        )
    } else {
        format!("Panic!\n{}", info.message())
    };

    let tv = screen::tv();
    let drc = screen::drc();
    tv.enable();
    drc.enable();

    let n = 15;
    for i in (0..n).rev() {
        tv.fill(screen::Color::black());
        drc.fill(screen::Color::black());

        tv.text(&msg, 0.5, 0.45, screen::TextAlign::Center);
        drc.text(&msg, 0.5, 0.45, screen::TextAlign::Center);

        let bar = format!("|{:n$}|", "-".repeat(i));
        tv.text(
            &bar,
            0.5 - (n as f32 * 0.0075),
            0.8,
            screen::TextAlign::Left,
        );
        drc.text(
            &bar,
            0.5 - (n as f32 * 0.0075),
            0.8,
            screen::TextAlign::Left,
        );

        tv.update();
        drc.update();
        thread::sleep(time::Duration::from_secs(1));
    }

    // process::exit();
    unsafe {
        bindings::SYSLaunchMenu();
        // bindings::ProcUIDrawDoneRelease();
        // bindings::ProcUIShutdown();
        // process::exit();

        // bindings::ProcUIShutdown();

        // bindings::_Exit(-1)
        // bindings::OSShutdown();

        // bindings::OSPanic(c"idk".as_ptr(), 123, c"Panic".as_ptr());
        bindings::OSFatal(c"Fatal".as_ptr());
    }
    // while process::running() {}

    todo!()
    */
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
