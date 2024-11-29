// process.rs

use crate::bindings as c_wut;
use crate::io;
use crate::screen;
use flagset::FlagSet;

pub fn default() {
    unsafe {
        c_wut::WHBProcInit();
        io::_stdout_init(io::Stdout::Cafe.into());
    }
}

pub fn custom(stdout: impl Into<FlagSet<io::Stdout>>) {
    unsafe {
        c_wut::WHBProcInit();
        io::_stdout_init(stdout.into());
    }
}

pub fn exit() {
    unsafe {
        io::_stdout_deinit();
        screen::OSSCREEN.clear();
        c_wut::WHBProcShutdown();
    }
}

pub fn running() -> bool {
    unsafe { c_wut::WHBProcIsRunning() != 0 }
}
