#![no_std]

pub extern crate alloc;
pub extern crate flagset;
pub extern crate math;
extern crate thiserror;

pub extern crate wut_macros;
pub use wut_macros::*;

pub extern crate sys;

pub use sys::bindings as bindings;

#[cfg(feature = "collections")]
pub mod collections;
pub mod dynamic_loading;
pub mod env;
pub mod font;
pub mod foreground;
pub mod fs;
pub mod gamepad;
pub mod gx2;
pub mod logger;
pub mod net;
pub mod path;
pub mod process;
pub mod ptr;
pub mod rrc;
pub mod screen;
pub mod sync;
pub mod thread;
pub mod time;
pub mod title;

mod utils;

pub mod prelude {
    pub use crate::println;
    pub use alloc::format;
    pub use alloc::string::{String, ToString};
    pub use alloc::vec;
    pub use alloc::vec::*;
    pub use core::alloc::{GlobalAlloc, Layout};
    pub use core::prelude::rust_2021::*;
    pub use math::FloatingMathExt;
}
