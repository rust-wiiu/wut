//! Env
//! 
//! Emulate the behaviour of a process’s environment.

use crate::{
    fs,
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};

static CWD: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::from("/")));

pub mod consts {
    pub const ARCH: &str = "powerpc32";
    pub const DLL_EXTENSION: &str = "rpl";
    pub const DLL_PREFIX: &str = "";
    pub const DLL_SUFFIX: &str = ".rpl";
    pub const EXE_EXTENSION: &str = "rpx";
    pub const EXE_SUFFIX: &str = ".rpx";
    pub const FAMILY: &str = "unix";
    pub const OS: &str = "cafeos";
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
/// Returns an [`FilesystemError`] if the path is invalid.
/// Possible cases:
///
/// * Directory does not exist.
/// * Insufficient permissions to access the directory.
pub fn set_current_dir<P: AsRef<Path>>(path: P) -> Result<(), fs::FilesystemError> {
    fs::exists(path.as_ref())?;
    *CWD.lock().unwrap() = path.as_ref().to_path_buf();
    Ok(())
}
