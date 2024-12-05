//! Filesystem

use crate::bindings::{self as c_wut, FSInit};
use crate::path::{Component, Path, PathBuf, MAIN_SEPARATOR};
use crate::rrc::{ResourceGuard, Rrc};
use alloc::boxed::Box;
use alloc::ffi::CString;
use alloc::string::{String, ToString};
use core::ffi::{self, CStr};
use core::str::Utf8Error;
use core::time::Duration;
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
}

impl TryFrom<i32> for FilesystemError {
    type Error = FilesystemError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            c_wut::FS_STATUS_OK => Ok(Self::Unknown(value)),
            _ => Err(Self::Unknown(value)),
        }
    }
}

#[allow(non_snake_case)]
struct IoHandler<'a> {
    // not entirely sure why Box is required, but I think it has something to do with copied/moved memory, which the API apperently doesnt like. So: BOX IS REQUIRED. Trust me.
    client: Box<c_wut::FSClient>,
    block: Box<c_wut::FSCmdBlock>,
    errorMask: c_wut::FSErrorFlag,
    _resource: ResourceGuard<'a>,
}

impl<'a> IoHandler<'_> {
    fn new() -> Result<Self, FilesystemError> {
        let mut io = Self {
            client: Box::new(c_wut::FSClient::default()),
            block: Box::new(c_wut::FSCmdBlock::default()),
            errorMask: c_wut::FS_ERROR_FLAG_ALL,
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
        unsafe { c_wut::FSDelClient(self.client.as_mut(), self.errorMask) };
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
            io.errorMask,
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
                io.errorMask,
            )
        };
        match FilesystemError::try_from(status) {
            Ok(_) => (),
            // Err(-2) => return None,
            Err(e) => return None,
        };

        Some(DirEntry::try_from(entry))
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
                io.errorMask,
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

    let path = path.as_ref();

    let status = unsafe {
        c_wut::FSOpenDir(
            io.client.as_mut(),
            io.block.as_mut(),
            path.as_c_str().as_ptr(),
            &mut handle,
            io.errorMask,
        )
    };
    FilesystemError::try_from(status)?;

    Ok(ReadDir {
        handle,
        base: PathBuf::from(path),
    })
}

pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata, FilesystemError> {
    let mut io = IoHandler::new()?;
    let mut info = c_wut::FSStat::default();

    let path = path.as_ref();

    let status = unsafe {
        c_wut::FSGetStat(
            io.client.as_mut(),
            io.block.as_mut(),
            path.as_c_str().as_ptr(),
            &mut info,
            io.errorMask,
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
