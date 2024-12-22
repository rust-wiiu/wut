//! Filesystem

use crate::{
    bindings::{self as c_wut, FSInit},
    path::{Component, Path, PathBuf, MAIN_SEPARATOR},
    rrc::{ResourceGuard, Rrc},
};
use alloc::{
    boxed::Box,
    ffi::CString,
    string::{String, ToString},
};
use core::{
    ffi::{self, CStr},
    str::Utf8Error,
    time::Duration,
};
use flagset::{flags, FlagSet};
use thiserror::Error;

pub(crate) static FS: Rrc<fn(), fn()> = Rrc::new(
    || unsafe {
        c_wut::FSInit();
    },
    || unsafe {
        c_wut::FSShutdown();
    },
);

#[derive(Debug, Error)]
pub enum FilesystemError {
    #[error("Some unknown eror code was returned. Should never happen!")]
    Unknown(i32),
    #[error("Conversion into utf-8 encoding failed")]
    InvalidCharacters(#[from] Utf8Error),
    #[error("Object at given path cannot be found")]
    NotFound,
    #[error("Object was read to end")]
    AllRead, // not sure if this also applies to files or just to directories (then maybe change the name)
}

impl TryFrom<i32> for FilesystemError {
    type Error = FilesystemError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            c_wut::FS_STATUS_OK => Ok(Self::Unknown(value)),
            c_wut::FS_STATUS_END => Err(Self::AllRead),
            c_wut::FS_STATUS_NOT_FOUND => Err(Self::NotFound),
            _ => Err(Self::Unknown(value)),
        }
    }
}

struct IoHandler<'a> {
    // not entirely sure why Box is required, but I think it has something to do with copied/moved memory, which the API apperently doesnt like. So: BOX IS REQUIRED. Trust me.
    client: Box<c_wut::FSClient>,
    block: Box<c_wut::FSCmdBlock>,
    error_mask: c_wut::FSErrorFlag,
    _resource: ResourceGuard<'a>,
}

impl<'a> IoHandler<'_> {
    fn new() -> Result<Self, FilesystemError> {
        let mut io = Self {
            client: Box::new(c_wut::FSClient::default()),
            block: Box::new(c_wut::FSCmdBlock::default()),
            error_mask: c_wut::FS_ERROR_FLAG_ALL,
            _resource: FS.acquire(),
        };

        let status = unsafe { c_wut::FSAddClient(io.client.as_mut(), c_wut::FS_ERROR_FLAG_ALL) };
        FilesystemError::try_from(status)?;

        unsafe {
            c_wut::FSInitCmdBlock(io.block.as_mut());
        }

        Ok(io)
    }
}

impl<'a> Drop for IoHandler<'_> {
    fn drop(&mut self) {
        unsafe { c_wut::FSDelClient(self.client.as_mut(), self.error_mask) };
    }
}

pub fn current_dir() -> Result<PathBuf, FilesystemError> {
    let mut io = IoHandler::new()?;
    let mut buffer: [ffi::c_char; 256] = [0; 256];

    let status = unsafe {
        c_wut::FSGetCwd(
            io.client.as_mut(),
            io.block.as_mut(),
            buffer.as_mut_ptr(),
            (buffer.len() - 1) as u32,
            io.error_mask,
        )
    };
    FilesystemError::try_from(status)?;

    Ok(PathBuf::try_from(buffer.as_ptr())?)
}

pub struct ReadDir {
    handle: c_wut::FSDirectoryHandle,
    base: PathBuf,
}

impl Iterator for ReadDir {
    type Item = Result<DirEntry, FilesystemError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut io = match IoHandler::new() {
            Ok(io) => io,
            Err(e) => return Some(Err(e)),
        };
        let mut entry = c_wut::FSDirectoryEntry::default();

        let status = unsafe {
            c_wut::FSReadDir(
                io.client.as_mut(),
                io.block.as_mut(),
                self.handle,
                &mut entry,
                io.error_mask,
            )
        };
        match FilesystemError::try_from(status) {
            Ok(_) => (),
            Err(FilesystemError::AllRead) => return None,
            Err(e) => return Some(Err(e)),
        };

        let mut entry = match DirEntry::try_from(entry) {
            Ok(entry) => entry,
            Err(error) => return Some(Err(error)),
        };
        entry.path = self.base.join(&entry.path);
        Some(Ok(entry))
    }
}

impl Drop for ReadDir {
    fn drop(&mut self) {
        let mut io = IoHandler::new().unwrap();

        let status = unsafe {
            c_wut::FSCloseDir(
                io.client.as_mut(),
                io.block.as_mut(),
                self.handle,
                io.error_mask,
            )
        };
        FilesystemError::try_from(status).unwrap();
    }
}

#[derive(Debug)]
pub struct DirEntry {
    path: PathBuf,
    meta: Metadata,
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

#[derive(Debug)]
pub struct Metadata(c_wut::FSStat);

impl TryFrom<c_wut::FSDirectoryEntry> for DirEntry {
    type Error = FilesystemError;
    fn try_from(value: c_wut::FSDirectoryEntry) -> Result<Self, Self::Error> {
        Ok(Self {
            path: PathBuf::try_from(value.name.as_ptr())?,
            meta: Metadata(value.info),
        })
    }
}

pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir, FilesystemError> {
    let mut io = IoHandler::new()?;
    let mut handle = c_wut::FSDirectoryHandle::default();

    let path = PathBuf::from(path.as_ref());
    let str = CString::new(path.as_str()).unwrap();

    let status = unsafe {
        c_wut::FSOpenDir(
            io.client.as_mut(),
            io.block.as_mut(),
            str.as_c_str().as_ptr(),
            &mut handle,
            io.error_mask,
        )
    };
    FilesystemError::try_from(status)?;

    Ok(ReadDir { handle, base: path })
}

pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata, FilesystemError> {
    let mut io = IoHandler::new()?;
    let mut info = c_wut::FSStat::default();

    let path = path.as_ref();
    let str = CString::new(path.as_str()).unwrap();

    let status = unsafe {
        c_wut::FSGetStat(
            io.client.as_mut(),
            io.block.as_mut(),
            str.as_c_str().as_ptr(),
            &mut info,
            io.error_mask,
        )
    };
    FilesystemError::try_from(status)?;

    Ok(Metadata(info))
}

pub fn absolute<P: AsRef<Path>>(path: P) -> Result<PathBuf, FilesystemError> {
    let mut path = PathBuf::from(path.as_ref());
    let mut result = PathBuf::new();

    if path.is_relative() {
        path = current_dir()?.join(path);
    }

    for part in path.components() {
        // crate::println!("{:?}", part);
        match part {
            Component::RootDir => {
                result.push(MAIN_SEPARATOR.to_string());
            }
            Component::CurDir => (),
            Component::ParentDir => {
                result.pop();
            }
            Component::Normal(name) => {
                result.push(name);
            }
        }
    }

    Ok(result)
}

pub trait PathBufExt {
    fn exists(&self) -> bool;
}

impl PathBufExt for PathBuf {
    fn exists(&self) -> bool {
        metadata(self).is_ok()
    }
}

pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<(), FilesystemError> {
    let mut io = IoHandler::new()?;

    let str = CString::new(path.as_ref().as_str()).unwrap();

    let status = unsafe {
        c_wut::FSMakeDir(
            io.client.as_mut(),
            io.block.as_mut(),
            str.as_c_str().as_ptr(),
            io.error_mask,
        )
    };
    FilesystemError::try_from(status)?;

    Ok(())
}

pub fn exists<P: AsRef<Path>>(path: P) -> Result<bool, FilesystemError> {
    match metadata(path) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

pub enum FileMode {
    /// Open for reading. The file must exist.
    Read,
    /// Open for writing. Creates an empty file or truncates an existing file.
    Write,
    /// Open for appending. Writes data at the end of the file. Creates the file if it does not exist.
    Append,
    /// Open for reading and writing. The file must exist.
    ReadWrite,
    /// Open for reading and writing. Creates an empty file or truncates an existing file.
    ReadWriteCreate,
    /// Open for reading and appending. The file is created if it does not exist.
    ReadAppendCreate,
}

impl FileMode {
    pub fn as_c_str(&self) -> &CStr {
        match self {
            FileMode::Read => c"r",
            FileMode::Write => c"w",
            FileMode::Append => c"a",
            FileMode::ReadWrite => c"r+",
            FileMode::ReadWriteCreate => c"w+",
            FileMode::ReadAppendCreate => c"a+",
        }
    }
}

pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    create: bool,
    truncate: bool,
}

impl OpenOptions {
    pub fn new() -> Self {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            create: false,
            truncate: false,
        }
    }

    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }

    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        self
    }

    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    fn to_file_mode(&self) -> FileMode {
        match (
            self.read,
            self.write,
            self.append,
            self.create,
            self.truncate,
        ) {
            // (read, write, append, create, truncate)
            (true, false, false, false, _) => FileMode::Read,
            (false, true, false, true, true) => FileMode::Write,
            (false, false, true, true, _) => FileMode::Append,
            (true, true, false, false, _) => FileMode::ReadWrite,
            (true, true, false, true, true) => FileMode::ReadWriteCreate,
            (true, _, true, true, _) => FileMode::ReadAppendCreate,
            _ => panic!("Invalid combination of options"),
        }
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<File, FilesystemError> {
        todo!()
    }
}

pub struct File {
    handle: c_wut::FSFileHandle,
    path: PathBuf,
}

impl File {
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, FilesystemError> {
        let mut io = IoHandler::new()?;

        let path = PathBuf::from(path.as_ref());
        let str = CString::new(path.as_str()).unwrap();
        let mode = FileMode::Write.as_c_str();
        let mut handle = c_wut::FSFileHandle::default();

        let status = unsafe {
            c_wut::FSOpenFile(
                io.client.as_mut(),
                io.block.as_mut(),
                str.as_c_str().as_ptr(),
                mode.as_ptr(),
                &mut handle,
                io.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        // Ok(())
        todo!()
    }

    // pub fn create_buffered

    pub fn create_new<P: AsRef<Path>>(path: P) -> Result<Self, FilesystemError> {
        todo!()
    }

    pub fn metadata(&self) -> Result<Metadata, FilesystemError> {
        todo!()
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, FilesystemError> {
        todo!()
    }

    // pub fn open_buffered
}
