#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::panic::PanicInfo;

#[cfg(feature = "default_panic_handler")]
#[panic_handler]
fn panic_handler(_panic_info: &PanicInfo) -> ! {
    unsafe {
        OSFatal("PANIC!".as_ptr() as *const i8);
    }
    loop {}
}

include!("bindings.rs");
