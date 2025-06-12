//! A module for working with processes.
//!
//! This module provides functions to initialize and deinitialize the application process, check if the process should continue running or exit, and reboot the system.

use crate::logger; // fs, screen
use flagset::FlagSet;
use wut_sys as sys;

/// Initialize application process.
///
/// This function is required to be ran as soon as possible in `main`. If using the `#[wut::main]` macro, you mustn't call it manually.
pub fn init(stdout: impl Into<FlagSet<logger::Channel>>) {
    unsafe {
        sys::WHBProcInit();
        let _ = logger::init(stdout.into()).unwrap();
        sys::OSReportInfo(c"process::init".as_ptr());
    }
}

/// Deinitialize application process.
///
/// This function is required to be ran as late as possible in `main`. If using the `#[wut_main]` macro, you mustn't call it manually.
pub fn deinit() {
    unsafe {
        sys::OSReportInfo(c"process::deinit".as_ptr());
    }
    // screen::OSSCREEN.clear();
    // fs::FS.clear();

    logger::deinit();

    if running() {
        exit();
    }
}

/// Check if the OS wants to move application out of foreground.
///
/// Should be ran in reasonable intervals or OS may be unresponseable.
/// Typically ran with `while process:running() {...}`.
pub fn running() -> bool {
    unsafe { sys::ProcUIIsRunning() != 0 && sys::WHBProcIsRunning() != 0 }
}

/// Terminates the process in an abnormal fashion.
///
/// The function will never return and will immediately terminate the current process in a platform specific "abnormal" manner.
///
/// Note that because this function never returns, and that it terminates the process, no destructors on the current stack or any other thread's stack will be run.
pub fn exit() -> ! {
    unsafe {
        sys::OSReportInfo(c"process::exit".as_ptr());
        sys::SYSLaunchMenu();
    }
    while running() {}
    loop {
        unsafe {
            sys::_Exit(-1);
        }
    }
}

/// Like [exit] but forces a reboot of the console after exit.
pub fn reboot() -> ! {
    unsafe {
        sys::OSForceFullRelaunch();
    }
    exit()
}
