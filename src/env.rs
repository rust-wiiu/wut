use crate::{
    fs::{FilesystemError, FsHandler},
    path::{Path, PathBuf},
    sync::OnceLock,
};

static mut HANDLER: OnceLock<FsHandler> = OnceLock::new();

pub fn current_dir() -> Result<PathBuf, FilesystemError> {
    let fs = unsafe { HANDLER.get_mut_or_try_init(|| FsHandler::new()) }?;

    fs.get_working_dir()
}

pub fn set_current_dir<P: AsRef<Path>>(path: P) -> Result<(), FilesystemError> {
    let fs = unsafe { HANDLER.get_mut_or_try_init(|| FsHandler::new()) }?;

    fs.set_working_dir(path)
}
