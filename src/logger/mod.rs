// logger

use crate::bindings as c_wut;
use core::ffi::CStr;
use flagset::{flags, FlagSet};
pub use Channel::{Cafe, Console, Module, Udp};

// region: stdout

pub(crate) static mut STDOUT: u8 = 0;

flags! {
    pub enum Channel: u8 {
        /// Default Wii U logging system
        Cafe,
        /// Write to screen. Requires exclusive access over screen.
        Console,
        /// Write to WUMS Logging Module.
        Module,
        /// Write to UDP server on port ...
        Udp
    }
}

pub(crate) unsafe fn _stdout_init(stdout: FlagSet<Channel>) {
    let mut stdout = stdout;
    if stdout.contains(Channel::Cafe) {
        if c_wut::WHBLogCafeInit() == 0 {
            stdout ^= Channel::Cafe;

            let msg = c"WHBLogCafeInit() failed!\n";
            c_wut::OSConsoleWrite(msg.as_ptr(), msg.to_bytes_with_nul().len() as u32);
        }
    }

    if stdout.contains(Channel::Console) {
        // for some reason, this returns FALSE on success
        if c_wut::WHBLogConsoleInit() != 0 {
            stdout ^= Channel::Console;

            let msg = c"WHBLogConsoleInit() failed!\n";
            c_wut::OSConsoleWrite(msg.as_ptr(), msg.to_bytes_with_nul().len() as u32);
        } else {
            c_wut::WHBLogConsoleSetColor(0xFF000000);
        }
    }

    if stdout.contains(Channel::Module) {
        if c_wut::WHBLogModuleInit() == 0 {
            stdout ^= Channel::Module;

            let msg = c"WHBLogModuleInit() failed!\n";
            c_wut::OSConsoleWrite(msg.as_ptr(), msg.to_bytes_with_nul().len() as u32);
        }
    }
    if stdout.contains(Channel::Udp) {
        if c_wut::WHBLogUdpInit() == 0 {
            stdout ^= Channel::Udp;

            let msg = c"WHBLogUdpInit() failed!\n";
            c_wut::OSConsoleWrite(msg.as_ptr(), msg.to_bytes_with_nul().len() as u32);
        }
    }

    STDOUT = stdout.bits();
}

pub(crate) unsafe fn _stdout_deinit() {
    let stdout: FlagSet<Channel> = FlagSet::new_truncated(STDOUT);

    if stdout.contains(Channel::Cafe) {
        c_wut::WHBLogCafeDeinit();
    }

    if stdout.contains(Channel::Console) {
        c_wut::WHBLogConsoleFree();
    }

    if stdout.contains(Channel::Module) {
        c_wut::WHBLogModuleDeinit();
    }
    if stdout.contains(Channel::Udp) {
        c_wut::WHBLogUdpDeinit();
    }
}

pub unsafe fn _print(str: &CStr) {
    c_wut::WHBLogPrint(str.as_ptr());

    if FlagSet::new_unchecked(STDOUT).contains(Channel::Console) {
        c_wut::WHBLogConsoleDraw();
    }
}

// endregion
