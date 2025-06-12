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

extern crate alloc;

pub use wut_core::*;
pub use wut_macros as macros;
pub use wut_math::*;
pub use wut_sys as sys;

/// The `wut` prelude
pub mod prelude {
    pub use alloc::format;
    pub use alloc::string::{String, ToString};
    pub use alloc::vec;
    pub use alloc::vec::*;
    pub use core::alloc::{GlobalAlloc, Layout};
    pub use core::prelude::rust_2024::*;
    pub use wut_core::println;
    pub use wut_macros::main;
    pub use wut_math::FloatingMathExt;
}
