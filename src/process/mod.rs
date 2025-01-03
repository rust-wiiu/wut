// process.rs

use crate::{bindings as c_wut, fs, io, screen};
use flagset::FlagSet;

/// Initialize important stuff.
///
/// This function is required to be ran as soon as possible in `main` - Features may not work properly if this is not done.
/// You can alternatively run [custom][crate::process::custom] for more controll.
pub fn default() {
    unsafe {
        c_wut::WHBProcInit();
        io::_stdout_init(io::Stdout::Cafe.into());
    }
}

/// Initialize important stuff.
///
/// This function is required to be ran as soon as possible in `main` - Features may not work properly if this is not done.
/// You can alternatively run [custom][crate::process::custom] to use defaults.
pub fn custom(stdout: impl Into<FlagSet<io::Stdout>>) {
    unsafe {
        c_wut::WHBProcInit();
        io::_stdout_init(stdout.into());
    }
}

/// Uninitialize/free important stuff.
///
/// This function is required to be ran as late as possible in `main` - not doing this may result in memory leaks.
pub fn exit() {
    unsafe {
        io::_stdout_deinit();
        // screen::OSSCREEN.clear();
        fs::FS.clear();
        c_wut::WHBProcShutdown();
    }
    // to_menu()
}

/// Check if the OS wants to move application out of foreground.
///
/// Should be ran in resonable intervals or OS may be unresponseable.
/// Typically ran instead of `loop{...}`.
pub fn running() -> bool {
    unsafe { c_wut::WHBProcIsRunning() != 0 }
}

/// Forcefully exit the application and return to main menu with a full system reload - this clears any leaking memory.
pub fn to_menu() -> ! {
    unsafe {
        c_wut::SYSLaunchMenu();
        c_wut::OSForceFullRelaunch();
    }
    while running() {}
    loop {
        unsafe {
            c_wut::_Exit(-1);
        }
    }
}
