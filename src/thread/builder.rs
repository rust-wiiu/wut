// use crate::bindings as c_wut;
use crate::thread::thread::*;
use alloc::string::String;
use flagset::FlagSet;

pub struct Builder {
    name: String,
    attribute: FlagSet<ThreadAttribute>,
    priority: i32,
    stack_size: usize,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            name: "Rust Thread".into(),
            attribute: ThreadAttribute::CpuAny.into(),
            priority: 15,
            stack_size: 128 * 1024,
        }
    }
}

impl Builder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn attribute(mut self, attributes: impl Into<FlagSet<ThreadAttribute>>) -> Self {
        self.attribute = attributes.into();
        self
    }

    /// Set thread priority.
    /// Used by scheduler.
    ///
    /// `0` is highest priority, `31` is lowest priority.
    ///
    ///
    pub fn priority(mut self, priority: impl Into<i32>) -> Self {
        self.priority = priority.into();
        self
    }

    /// Set thread stack size (bytes).
    pub fn stack_size(mut self, stack_size: impl Into<usize>) -> Self {
        self.stack_size = stack_size.into();
        self
    }

    pub fn create() -> Thread {
        todo!()
    }

    pub fn spawn() -> Thread {
        todo!()
    }
}
