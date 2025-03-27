//! Wii U Webbrowser

use crate::bindings as c_wut;

/// Opens the Wii U browser. Pass `None` to go the default page.
pub fn browser(url: Option<&str>) {
    let url = if let Some(url) = url {
        Some(alloc::ffi::CString::new(url).unwrap())
    } else {
        None
    };

    let mut args = c_wut::SysAppBrowserArgs {
        stdArgs: c_wut::SYSStandardArgsIn {
            argString: core::ptr::null_mut(),
            size: 0,
        },
        url: url
            .as_ref()
            .map_or(core::ptr::null_mut(), |v| v.as_ptr() as *mut _),
        urlSize: url.as_ref().map_or(0, |v| v.count_bytes() as u32),
    };

    unsafe {
        c_wut::SYSSwitchToBrowser(&mut args);
    }
}

/// Opens a single tab in the Wii U Browser.
pub fn single_tab(url: &str) {
    let url = alloc::ffi::CString::new(url).unwrap();

    let mut args = c_wut::SysAppBrowserArgs {
        stdArgs: c_wut::SYSStandardArgsIn {
            argString: core::ptr::null_mut(),
            size: 0,
        },
        url: url.as_ptr() as *mut _,
        urlSize: url.count_bytes() as u32,
    };

    unsafe {
        c_wut::SYSSwitchToBrowserForViewer(&mut args);
    }
}
