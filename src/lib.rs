//! # Wii U Toolchain
//!
//! The Wii U Toolchain (wut) is the foundation of writing Rust Homebrew software for the Nintendo Wii U™. It offers *similar* behaviour and useability as the `std` crate for normal Rust. However, for now it is is a `no_std` envionment, needs the [devkitPro WUT](https://github.com/devkitPro/wut) toolchain as a base, and has no "official" Rust support. It can be seem more like a crate with batteries included.
//!
//! # Tutorial & Information
//!
//! If you are new to writing Rust on the Wii U, or code for the Wii U in general, I recommend you start by reading the [Book for U](https://rust-wiiu.github.io/book-for-u) guide book.
//!
//! # System libraries
//!
//! If you need to look up the documentation for the underlying system libraries, you can look up the [WUT documentation](https://wut.devkitpro.org/) or search in the respective repositories on [github/devkitPro](https://github.com/devkitPro/) or [github/wiiu-env](https://github.com/wiiu-env).

#![no_std]

pub extern crate alloc;
pub extern crate flagset;
pub extern crate math;
extern crate thiserror;

pub extern crate wut_macros;
pub use wut_macros::{main, ShaderAttributes};

pub extern crate sys;

pub use sys::bindings;

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

/// The `wut` prelude
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
