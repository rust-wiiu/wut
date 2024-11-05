// process.rs

use crate::bindings::*;

pub struct Process {}

impl Process {
    pub fn new() -> Self {
        unsafe {
            WHBProcInit();
        }

        Process {}
    }

    fn shutdown(&self) {
        unsafe {
            WHBProcShutdown();
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        self.shutdown();
    }
}
