#![no_std]

extern crate alloc;

mod bindings;
pub use bindings::*;

#[cfg(feature = "panic_handler")]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    use alloc::{alloc::GlobalAlloc, format};

    let (file, line, column, msg) = if let Some(location) = info.location() {
        (
            location.file(),
            location.line(),
            location.column(),
            info.message(),
        )
    } else {
        ("<unknown>", 0, 0, info.message())
    };

    let file = alloc::ffi::CString::new(format!("File: {file} - {line}:{column}")).unwrap();
    let msg = alloc::ffi::CString::new(format!("Reason: {}", msg.as_str().unwrap_or(""))).unwrap();

    unsafe {
        OSScreenInit();

        let tv_buffer_size = OSScreenGetBufferSizeEx(OSScreenID::SCREEN_TV) as usize;
        let drc_buffer_size = OSScreenGetBufferSizeEx(OSScreenID::SCREEN_DRC) as usize;

        let tv_layout = alloc::alloc::Layout::from_size_align(tv_buffer_size, 0x100).unwrap();
        let tv_buffer = GLOBAL_ALLOCATOR.alloc(tv_layout);
        let drc_layout = alloc::alloc::Layout::from_size_align(drc_buffer_size, 0x100).unwrap();
        let drc_buffer = GLOBAL_ALLOCATOR.alloc(drc_layout);

        OSScreenSetBufferEx(OSScreenID::SCREEN_TV, tv_buffer as *mut _);
        OSScreenSetBufferEx(OSScreenID::SCREEN_DRC, drc_buffer as *mut _);

        OSScreenEnableEx(OSScreenID::SCREEN_TV, 1);
        OSScreenEnableEx(OSScreenID::SCREEN_DRC, 1);

        OSScreenClearBufferEx(OSScreenID::SCREEN_TV, 0x00000000);
        OSScreenClearBufferEx(OSScreenID::SCREEN_DRC, 0x00000000);

        OSScreenPutFontEx(OSScreenID::SCREEN_TV, 0, 0, c"Panic!".as_ptr());
        OSScreenPutFontEx(OSScreenID::SCREEN_TV, 0, 1, file.as_ptr());
        OSScreenPutFontEx(OSScreenID::SCREEN_TV, 0, 2, msg.as_ptr());

        OSScreenPutFontEx(OSScreenID::SCREEN_DRC, 0, 0, c"Panic!".as_ptr());
        OSScreenPutFontEx(OSScreenID::SCREEN_DRC, 0, 1, file.as_ptr());
        OSScreenPutFontEx(OSScreenID::SCREEN_DRC, 0, 2, msg.as_ptr());

        DCFlushRange(tv_buffer as *mut _, tv_buffer_size as u32);
        OSScreenFlipBuffersEx(OSScreenID::SCREEN_TV);
        DCFlushRange(tv_buffer as *mut _, tv_buffer_size as u32);
        OSScreenFlipBuffersEx(OSScreenID::SCREEN_DRC);

        // Sleep 5 seconds
        let ticks_per_second = (*OSGetSystemInfo()).busClockSpeed as i64 / 4;
        for _ in 1..=5 {
            OSSleepTicks(ticks_per_second);
        }

        OSScreenShutdown();
        GLOBAL_ALLOCATOR.dealloc(tv_buffer, tv_layout);
        GLOBAL_ALLOCATOR.dealloc(drc_buffer, drc_layout);
    }

    // reboot
    unsafe {
        use bindings::{
            OSForceFullRelaunch, ProcUIIsRunning, SYSLaunchMenu, WHBProcIsRunning, _Exit,
        };

        OSForceFullRelaunch();
        SYSLaunchMenu();
        while ProcUIIsRunning() != 0 && WHBProcIsRunning() != 0 {}
        loop {
            _Exit(-1);
        }
    }
}

pub struct WiiUAllocator;
unsafe impl alloc::alloc::GlobalAlloc for WiiUAllocator {
    unsafe fn alloc(&self, layout: alloc::alloc::Layout) -> *mut u8 {
        let size = layout.size() as u32;
        let align = layout.align() as i32;

        debug_assert!((align as u32).is_power_of_two());
        debug_assert!(align > 0);

        // align < 4 (under at least some circumstances) crashes the system
        (if align < 4 {
            bindings::MEMAllocFromDefaultHeap.unwrap()(size)
        } else {
            bindings::MEMAllocFromDefaultHeapEx.unwrap()(size, align)
        }) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: alloc::alloc::Layout) {
        bindings::MEMFreeToDefaultHeap.unwrap()(ptr as *mut core::ffi::c_void);
    }
}

#[global_allocator]
pub static GLOBAL_ALLOCATOR: WiiUAllocator = WiiUAllocator;
