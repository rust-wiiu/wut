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
            c_wut::OSReportWarn(c"WHBLogCafeInit() failed!".as_ptr());
        }
    }

    if stdout.contains(Stdout::Console) {
        if c_wut::WHBLogConsoleInit() == 0 {
            // stdout ^= Stdout::Console;
            c_wut::OSReportWarn(c"WHBLogConsoleInit() failed!".as_ptr());
        } else {
            c_wut::WHBLogConsoleSetColor(0xFF000000);
        }
    }

    if stdout.contains(Stdout::Module) {
        if c_wut::WHBLogModuleInit() == 0 {
            stdout ^= Stdout::Module;
            c_wut::OSReportWarn(c"WHBLogModuleInit() failed!".as_ptr());
        }
    }
    if stdout.contains(Stdout::Udp) {
        if c_wut::WHBLogUdpInit() == 0 {
            stdout ^= Stdout::Udp;
            c_wut::OSReportWarn(c"WHBLogUdpInit() failed!".as_ptr());
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
