// io

use crate::bindings as c_wut;
use core::ffi::CStr;
use flagset::{flags, FlagSet};

pub(crate) static mut STDOUT: u8 = 0;

flags! {
    pub enum Stdout: u8 {
        Cafe,
        Console,
        Module,
        Udp
    }
}

pub use Stdout::{Cafe, Console, Module, Udp};

pub(crate) unsafe fn _stdout_init(stdout: FlagSet<Stdout>) {
    let mut stdout = stdout;
    if stdout.contains(Stdout::Cafe) {
        if c_wut::WHBLogCafeInit() == 0 {
            stdout ^= Stdout::Cafe;

            let msg = c"WHBLogCafeInit() failed!\n";
            c_wut::OSConsoleWrite(msg.as_ptr(), msg.to_bytes_with_nul().len() as u32);
        }
    }

    if stdout.contains(Stdout::Console) {
        // for some reason, this returns FALSE on success
        if c_wut::WHBLogConsoleInit() != 0 {
            stdout ^= Stdout::Console;

            let msg = c"WHBLogConsoleInit() failed!\n";
            c_wut::OSConsoleWrite(msg.as_ptr(), msg.to_bytes_with_nul().len() as u32);
        } else {
            c_wut::WHBLogConsoleSetColor(0xFF000000);
        }
    }

    if stdout.contains(Stdout::Module) {
        if c_wut::WHBLogModuleInit() == 0 {
            stdout ^= Stdout::Module;

            let msg = c"WHBLogModuleInit() failed!\n";
            c_wut::OSConsoleWrite(msg.as_ptr(), msg.to_bytes_with_nul().len() as u32);
        }
    }
    if stdout.contains(Stdout::Udp) {
        if c_wut::WHBLogUdpInit() == 0 {
            stdout ^= Stdout::Udp;

            let msg = c"WHBLogUdpInit() failed!\n";
            c_wut::OSConsoleWrite(msg.as_ptr(), msg.to_bytes_with_nul().len() as u32);
        }
    }

    STDOUT = stdout.bits();
}

pub(crate) unsafe fn _stdout_deinit() {
    let stdout: FlagSet<Stdout> = FlagSet::new_truncated(STDOUT);

    if stdout.contains(Stdout::Cafe) {
        c_wut::WHBLogCafeDeinit();
    }

    if stdout.contains(Stdout::Console) {
        c_wut::WHBLogConsoleFree();
    }

    if stdout.contains(Stdout::Module) {
        c_wut::WHBLogModuleDeinit();
    }
    if stdout.contains(Stdout::Udp) {
        c_wut::WHBLogUdpDeinit();
    }
}

pub unsafe fn _print(str: &CStr) {
    c_wut::WHBLogPrint(str.as_ptr());

    if FlagSet::new_unchecked(STDOUT).contains(Stdout::Console) {
        c_wut::WHBLogConsoleDraw();
    }
}
