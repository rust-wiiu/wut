// process.rs

use crate::{bindings as c_wut, logger}; // fs, screen
use flagset::FlagSet;

/// Initialize important stuff.
///
/// This function is required to be ran as soon as possible in `main`. If using the `#[wut_main]` macro, you mustn't call it manually.
pub fn init(stdout: impl Into<FlagSet<logger::Channel>>) {
    unsafe {
        c_wut::WHBProcInit();
        let _ = logger::init(stdout.into()).unwrap();
    }
    crate::println!("process::init");
}

/// Initialize important stuff.
///
/// This function is required to be ran as late as possible in `main`. If using the `#[wut_main]` macro, you mustn't call it manually.
pub fn deinit() {
    crate::println!("process::deinit");
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
/// Typically ran instead of `loop{...}`.
pub fn running() -> bool {
    unsafe {
        if c_wut::ProcUIIsRunning() != 0 {
            c_wut::WHBProcIsRunning() != 0
        } else {
            false
        }
    }
}

/// Terminates the process in an abnormal fashion.
///
/// The function will never return and will immediately terminate the current process in a platform specific "abnormal" manner.
///
/// Note that because this function never returns, and that it terminates the process, no destructors on the current stack or any other thread's stack will be run.
pub fn exit() -> ! {
    crate::println!("process::exit");
    unsafe {
        c_wut::SYSLaunchMenu();
    }
    while running() {}
    loop {
        unsafe {
            c_wut::_Exit(-1);
        }
    }
}

/// Like [exit][crate::process::exit] but forces a reboot of the console after exit.
pub fn reboot() -> ! {
    unsafe {
        c_wut::OSForceFullRelaunch();
    }
    exit()
}
