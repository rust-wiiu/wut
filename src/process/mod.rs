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

/// Check if the OS wants to move application out of foreground.
///
/// Should be ran in resonable intervals or OS may be unresponseable.
/// Typically ran instead of `loop{...}`.
pub fn running() -> bool {
    unsafe { c_wut::WHBProcIsRunning() != 0 }
}

/// Like [exit][crate::process::exit] but forces a reboot of the console after exit.
pub fn reboot() -> ! {
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

/// Terminates the process in an abnormal fashion.
///
/// The function will never return and will immediately terminate the current process in a platform specific "abnormal" manner.
///
/// Note that because this function never returns, and that it terminates the process, no destructors on the current stack or any other thread's stack will be run.
pub fn exit() -> ! {
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

// old exit (but not really exit).
// pub fn exit() {
//     unsafe {
//         io::_stdout_deinit();
//         // screen::OSSCREEN.clear();
//         // fs::FS.clear();
//         c_wut::WHBProcShutdown();
//     }
// }
