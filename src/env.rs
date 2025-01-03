use crate::{
    fs::{FilesystemError, FsHandler},
    path::{Path, PathBuf},
    sync::OnceLock,
};

static mut HANDLER: OnceLock<FsHandler> = OnceLock::new();

/// Returns the current working directory as a [`PathBuf`].
///
/// Will always be the root directory (`/`) in the beginning.
#[allow(static_mut_refs)]
pub fn current_dir() -> Result<PathBuf, FilesystemError> {
    let fs = unsafe { HANDLER.get_mut_or_try_init(|| FsHandler::new()) }?;
    fs.get_working_dir()
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
pub fn set_current_dir<P: AsRef<Path>>(path: P) -> Result<(), FilesystemError> {
    let fs = unsafe { HANDLER.get_mut_or_try_init(|| FsHandler::new()) }?;
    fs.set_working_dir(path)
}
