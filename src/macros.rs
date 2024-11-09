#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        extern crate alloc;
        use alloc::{fmt::format, ffi::CString};

        let s = CString::new(format(format_args!($($arg)*))).unwrap();
        unsafe {
            $crate::io::_print(&s);
        }
    }};
}
