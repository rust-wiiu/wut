use crate::{
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};
static mut CWD: OnceLock<PathBuf> = OnceLock::new();

/// Returns the current working directory as a [`PathBuf`].
///
/// Will always be the root directory (`/`) in the beginning.
#[allow(static_mut_refs)]
pub fn current_dir() -> Result<PathBuf, fs::FilesystemError> {
    Ok(unsafe { CWD.get_or_init(|| PathBuf::from("/")) }.clone())
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
#[allow(static_mut_refs)]
pub fn set_current_dir<P: AsRef<Path>>(path: P) -> Result<(), fs::FilesystemError> {
    fs::exists(path.as_ref())?;

    let a = unsafe { CWD.get_mut_or_init(|| PathBuf::from("/")) };
    *a = path.as_ref().to_path_buf();

    Ok(())
}
