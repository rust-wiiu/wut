#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        extern crate alloc;
        use alloc::{fmt::format, ffi::CString};

        let s = CString::new(format(format_args!($($arg)*))).unwrap();
        unsafe {
            $crate::bindings::WHBLogWrite(s.as_ptr());
        }
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        extern crate alloc;
        use alloc::{fmt::format, ffi::CString};

        let s = CString::new(format(format_args!($($arg)*))).unwrap();
        unsafe {
            $crate::bindings::WHBLogPrint(s.as_ptr());
        }
    }};
}
