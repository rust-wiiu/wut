//! Inspection of the process' emulated environment.
//!
//! This module contains functions which emulate the behavior of a process environment. As CafeOS does not have a real process environment, this module provides a way to manage the current working directory and other environment-related tasks.

use crate::{
    fs,
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};

static CWD: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::from("/")));

/// Constants associated with the current target
pub mod consts {
    /// A string describing the architecture of the CPU that is currently in use. An example value may be: `"x86"`, `"arm"` or `"riscv64"`.
    pub const ARCH: &str = "powerpc";
    /// A string describing the family of the operating system. An example value may be: `"unix"`, or `"windows"`.
    pub const FAMILY: &str = "unix";
    /// A string describing the specific operating system in use. An example value may be: `"linux"`, or `"freebsd"`.
    pub const OS: &str = "cafeos";
    /// Specifies the filename prefix, if any, used for shared libraries on this platform.
    /// This is either `"lib"` or an empty string. (`""`).
    pub const DLL_PREFIX: &str = "";
    /// Specifies the filename suffix, if any, used for shared libraries on this platform.
    /// An example value may be: `".so"`, `".elf"`, or `".dll"`.
    ///
    /// The possible values are identical to those of [`DLL_EXTENSION`], but with the leading period included.
    pub const DLL_SUFFIX: &str = ".rpl";
    /// Specifies the file extension, if any, used for shared libraries on this platform that goes after the dot.
    /// An example value may be: `"so"`, `"elf"`, or `"dll"`.
    pub const DLL_EXTENSION: &str = "rpl";
    /// Specifies the filename suffix, if any, used for executable binaries on this platform.
    /// An example value may be: `".exe"`, or `".efi"`.
    ///
    /// The possible values are identical to those of [`EXE_EXTENSION`], but with the leading period included.
    pub const EXE_EXTENSION: &str = "rpx";
    /// Specifies the file extension, if any, used for executable binaries on this platform.
    /// An example value may be: `"exe"`, or an empty string (`""`).
    pub const EXE_SUFFIX: &str = ".rpx";
}

/// Returns the current working directory as a [`PathBuf`].
///
/// Will always be the root directory (`/`) in the beginning.
pub fn current_dir() -> Result<PathBuf, fs::FilesystemError> {
    Ok(CWD.lock().unwrap().clone())
}

/// Changes the current working directory to the specified path.
///
/// # Errors
///
/// Returns an [`FilesystemError`][fs::FilesystemError] if the path is invalid.
/// Possible cases:
///
/// * Directory does not exist.
/// * Insufficient permissions to access the directory.
pub fn set_current_dir<P: AsRef<Path>>(path: P) -> Result<(), fs::FilesystemError> {
    fs::exists(path.as_ref())?;
    *CWD.lock().unwrap() = path.as_ref().to_path_buf();
    Ok(())
}
