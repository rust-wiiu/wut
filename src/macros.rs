//! Macros

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        extern crate alloc;
        use alloc::{fmt::format, ffi::CString};

        let s = CString::new(format(format_args!($($arg)*))).unwrap();
        // idk why Rust complaints here, _print is unsafe
        #[allow(unused_unsafe)]
        unsafe {
            $crate::io::_print(&s);
        }
    }};
}
