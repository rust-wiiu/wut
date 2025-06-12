#![no_std]

extern crate alloc;

#[cfg(feature = "collections")]
pub mod collections;
pub mod dynload;
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
