//! Filesystem

use crate::{
    bindings as c_wut,
    path::{Component, Path, PathBuf, MAIN_SEPARATOR},
    rrc::{ResourceGuard, Rrc},
    time::SystemTime,
};
use alloc::{
    boxed::Box,
    ffi::CString,
    string::{String, ToString},
    vec::Vec,
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

trait AsHandle {
    type Handle: Copy;
    fn as_handle(&self) -> Self::Handle;
}

// region: FsHandler

pub struct FsHandler<'a> {
    // not entirely sure why Box is required, but I think it has something to do with copied/moved memory, which the API apperently doesnt like. So: BOX IS REQUIRED. Trust me.
    client: Box<c_wut::FSClient>,
    block: Box<c_wut::FSCmdBlock>,
    error_mask: c_wut::FSErrorFlag,
    _resource: ResourceGuard<'a>,
}

impl<'a> FsHandler<'_> {
    pub fn new() -> Result<Self, FilesystemError> {
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

    // #TEST
    pub fn exists<P: AsRef<Path>>(&mut self, path: P) -> Result<bool, FilesystemError> {
        let _ = self.metadata_path(path)?;
        Ok(true)
    }

    // #TEST
    pub fn metadata_path<P: AsRef<Path>>(&mut self, path: P) -> Result<Metadata, FilesystemError> {
        let str = CString::new(path.as_ref().as_str()).unwrap();
        let mut stat = c_wut::FSStat::default();

        let status = unsafe {
            c_wut::FSGetStat(
                self.client.as_mut(),
                self.block.as_mut(),
                str.as_ptr(),
                &mut stat,
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(Metadata::from(stat))
    }

    // #TEST
    pub fn remove<P: AsRef<Path>>(&mut self, path: P) -> Result<(), FilesystemError> {
        let str = CString::new(path.as_ref().as_str()).unwrap();

        let status = unsafe {
            c_wut::FSRemove(
                self.client.as_mut(),
                self.block.as_mut(),
                str.as_ptr(),
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(())
    }

    // #TEST
    pub fn get_working_dir(&mut self) -> Result<PathBuf, FilesystemError> {
        const SIZE: usize = c_wut::FS_MAX_PATH as usize + 1;
        let mut buffer: [ffi::c_char; SIZE] = [0; SIZE];

        let status = unsafe {
            c_wut::FSGetCwd(
                self.client.as_mut(),
                self.block.as_mut(),
                buffer.as_mut_ptr() as *mut _,
                (SIZE - 1) as u32,
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(PathBuf::try_from(buffer.as_ptr())?)
    }

    // #TEST
    pub fn set_working_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<(), FilesystemError> {
        let str = CString::new(path.as_ref().as_str()).unwrap();

        let status = unsafe {
            c_wut::FSChangeDir(
                self.client.as_mut(),
                self.block.as_mut(),
                str.as_ptr(),
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(())
    }

    // region: File

    // #TEST
    pub fn metadata_file(&mut self, file: &File) -> Result<Metadata, FilesystemError> {
        let handle = file.as_handle();
        let mut stat = c_wut::FSStat::default();

        let status = unsafe {
            c_wut::FSGetStatFile(
                self.client.as_mut(),
                self.block.as_mut(),
                handle,
                &mut stat,
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(Metadata::from(stat))
    }

    // #TEST
    pub fn open_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        mode: FileMode,
    ) -> Result<File, FilesystemError> {
        let str = CString::new(path.as_ref().as_str()).unwrap();
        let mut handle = c_wut::FSFileHandle::default();

        let status = unsafe {
            c_wut::FSOpenFile(
                self.client.as_mut(),
                self.block.as_mut(),
                str.as_ptr(),
                mode.as_c_str().as_ptr(),
                &mut handle,
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(File {
            handle,
            path: path.as_ref().to_path_buf(),
        })
    }

    // #TEST
    pub fn read_file(&mut self, file: &File) -> Result<Vec<u8>, FilesystemError> {
        let metadata = self.metadata_file(file)?;
        let size = metadata.len() as usize;
        let mut buffer: Vec<u8> = Vec::with_capacity(size);

        let status = unsafe {
            c_wut::FSReadFile(
                self.client.as_mut(),
                self.block.as_mut(),
                buffer.as_mut_ptr(),
                size as u32,
                1,
                file.as_handle(),
                0,
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(buffer)
    }

    // #TEST
    pub fn close_file(&mut self, file: &File) -> Result<(), FilesystemError> {
        let status = unsafe {
            c_wut::FSCloseFile(
                self.client.as_mut(),
                self.block.as_mut(),
                file.as_handle(),
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(())
    }

    //endregion

    // region: Directory

    // #TEST
    pub fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<(), FilesystemError> {
        let str = CString::new(path.as_ref().as_str()).unwrap();

        let status = unsafe {
            c_wut::FSMakeDir(
                self.client.as_mut(),
                self.block.as_mut(),
                str.as_ptr(),
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(())
    }

    // #TEST
    pub fn open_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<ReadDir, FilesystemError> {
        let str = CString::new(path.as_ref().as_str()).unwrap();
        let mut handle = c_wut::FSDirectoryHandle::default();

        let status = unsafe {
            c_wut::FSOpenDir(
                self.client.as_mut(),
                self.block.as_mut(),
                str.as_ptr(),
                &mut handle,
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(ReadDir {
            handle,
            path: path.as_ref().to_path_buf(),
        })
    }

    // #TEST
    pub fn read_dir(&mut self, dir: &ReadDir) -> Result<DirEntry, FilesystemError> {
        let mut entry = c_wut::FSDirectoryEntry::default();

        let status = unsafe {
            c_wut::FSReadDir(
                self.client.as_mut(),
                self.block.as_mut(),
                dir.as_handle(),
                &mut entry,
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        let name = String::from_utf8_lossy(unsafe {
            alloc::slice::from_raw_parts(entry.name.as_ptr() as *const u8, entry.name.len())
        });

        Ok(DirEntry {
            metadata: Metadata::from(entry.info),
            path: dir.path().join(name),
        })
    }

    // #TEST
    pub fn close_dir(&mut self, dir: &ReadDir) -> Result<(), FilesystemError> {
        let status = unsafe {
            c_wut::FSCloseDir(
                self.client.as_mut(),
                self.block.as_mut(),
                dir.as_handle(),
                self.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(())
    }

    //endregion
}

impl<'a> Drop for FsHandler<'_> {
    fn drop(&mut self) {
        unsafe { c_wut::FSDelClient(self.client.as_mut(), self.error_mask) };
    }
}

// endregion

flags! {
    enum MetadataFlags: u32 {
        /// The retrieved file entry is a (link to a) directory.
        Directory = c_wut::FS_STAT_DIRECTORY,
        /// The retrieved file entry also has a quota set.
        Quota = c_wut::FS_STAT_QUOTA,
        /// The retrieved file entry is a (link to a) file.
        File = c_wut::FS_STAT_FILE,
        /// The retrieved file entry also is encrypted and can't be opened (see vWii files for example).
        Encrypted = c_wut::FS_STAT_ENCRYPTED_FILE,
        /// The retrieved file entry also is a link to a different file on the filesystem.
        ///
        /// Note: It's currently not known how one can read the linked-to file entry.
        Link = c_wut::FS_STAT_LINK
    }

    pub enum Mode: u8 {
        Read,
        Write,
        Execute
    }
}

// region: FileType

pub struct FileType(FlagSet<MetadataFlags>);

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.0.contains(MetadataFlags::Directory)
    }

    pub fn is_file(&self) -> bool {
        self.0.contains(MetadataFlags::File)
    }

    pub fn is_symlink(&self) -> bool {
        self.0.contains(MetadataFlags::Link)
    }
}

// endregion

// region: Permissions

#[derive(Debug, Default)]
pub struct Permissions {
    owner: FlagSet<Mode>,
    group: FlagSet<Mode>,
    other: FlagSet<Mode>,
}

impl From<c_wut::FSMode> for Permissions {
    fn from(value: c_wut::FSMode) -> Self {
        let mut p = Permissions::default();

        if (value & c_wut::FS_MODE_READ_OWNER) != 0 {
            p.owner |= Mode::Read;
        }

        if (value & c_wut::FS_MODE_WRITE_OWNER) != 0 {
            p.owner |= Mode::Write;
        }

        if (value & c_wut::FS_MODE_EXEC_OWNER) != 0 {
            p.owner |= Mode::Execute;
        }

        if (value & c_wut::FS_MODE_READ_GROUP) != 0 {
            p.group |= Mode::Read;
        }

        if (value & c_wut::FS_MODE_WRITE_GROUP) != 0 {
            p.group |= Mode::Write;
        }

        if (value & c_wut::FS_MODE_EXEC_GROUP) != 0 {
            p.group |= Mode::Execute;
        }

        if (value & c_wut::FS_MODE_READ_OTHER) != 0 {
            p.other |= Mode::Read;
        }

        if (value & c_wut::FS_MODE_WRITE_OTHER) != 0 {
            p.other |= Mode::Write;
        }

        if (value & c_wut::FS_MODE_EXEC_OTHER) != 0 {
            p.other |= Mode::Execute;
        }

        p
    }
}

impl Into<c_wut::FSMode> for Permissions {
    fn into(self) -> c_wut::FSMode {
        let mut m = c_wut::FSMode::default();

        if self.owner.contains(Mode::Read) {
            m &= c_wut::FS_MODE_READ_OWNER;
        }

        if self.owner.contains(Mode::Write) {
            m &= c_wut::FS_MODE_WRITE_OWNER;
        }

        if self.owner.contains(Mode::Execute) {
            m &= c_wut::FS_MODE_EXEC_OWNER;
        }

        if self.group.contains(Mode::Read) {
            m &= c_wut::FS_MODE_READ_GROUP;
        }

        if self.group.contains(Mode::Write) {
            m &= c_wut::FS_MODE_WRITE_GROUP;
        }

        if self.group.contains(Mode::Execute) {
            m &= c_wut::FS_MODE_EXEC_GROUP;
        }

        if self.other.contains(Mode::Read) {
            m &= c_wut::FS_MODE_READ_OTHER;
        }

        if self.other.contains(Mode::Write) {
            m &= c_wut::FS_MODE_WRITE_OTHER;
        }

        if self.other.contains(Mode::Execute) {
            m &= c_wut::FS_MODE_EXEC_OTHER;
        }

        m
    }
}

// endregion

// region: Metdata

pub struct Metadata(c_wut::FSStat);

impl Metadata {
    pub fn created(&self) -> Result<SystemTime, FilesystemError> {
        Ok(SystemTime::from(self.0.created as i64))
    }

    pub fn modified(&self) -> Result<SystemTime, FilesystemError> {
        Ok(SystemTime::from(self.0.modified as i64))
    }

    pub fn file_type(&self) -> FileType {
        FileType(FlagSet::<MetadataFlags>::new_truncated(self.0.flags))
    }

    pub fn is_dir(&self) -> bool {
        self.file_type().is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.file_type().is_file()
    }

    pub fn is_symlink(&self) -> bool {
        self.file_type().is_symlink()
    }

    pub fn len(&self) -> u64 {
        self.0.size as u64
    }

    pub fn permissions(&self) -> Permissions {
        Permissions::from(self.0.mode)
    }
}

impl From<c_wut::FSStat> for Metadata {
    fn from(value: c_wut::FSStat) -> Self {
        Self(value)
    }
}

// endregion

// region: File

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

pub struct File {
    handle: c_wut::FSFileHandle,
    path: PathBuf,
}

impl File {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl AsHandle for File {
    type Handle = c_wut::FSFileHandle;

    fn as_handle(&self) -> Self::Handle {
        self.handle
    }
}

// endregion

// region: ReadDir

pub struct ReadDir {
    handle: c_wut::FSDirectoryHandle,
    path: PathBuf,
}

impl ReadDir {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl AsHandle for ReadDir {
    type Handle = c_wut::FSDirectoryHandle;
    fn as_handle(&self) -> Self::Handle {
        self.handle
    }
}

impl Iterator for ReadDir {
    type Item = Result<DirEntry, FilesystemError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut fs = match FsHandler::new() {
            Ok(fs) => fs,
            Err(e) => return Some(Err(e)),
        };

        match fs.read_dir(self) {
            Ok(entry) => Some(Ok(entry)),
            Err(FilesystemError::AllRead) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl Drop for ReadDir {
    fn drop(&mut self) {
        let mut fs = FsHandler::new().unwrap();
        let _ = fs.close_dir(self).unwrap();
    }
}

// endregion

// region: DirEntry

pub struct DirEntry {
    metadata: Metadata,
    path: PathBuf,
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

// endregion

/*
std functions
*/

pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<u64, FilesystemError> {
    todo!()
}

pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<(), FilesystemError> {
    let mut fs = FsHandler::new()?;
    fs.create_dir(path)
}

pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<(), FilesystemError> {
    let mut fs = FsHandler::new()?;
    let path = path.as_ref().absolute()?;

    let mut sub = PathBuf::new();
    for component in path.components() {
        sub = sub.join(component);
        if !fs.exists(&sub)? {
            fs.create_dir(&sub)?;
        }
    }

    Ok(())
}

pub fn exists<P: AsRef<Path>>(path: P) -> Result<bool, FilesystemError> {
    let mut fs = FsHandler::new()?;
    fs.exists(path)
}

pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata, FilesystemError> {
    let mut fs = FsHandler::new()?;
    fs.metadata_path(path)
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, FilesystemError> {
    let mut fs = FsHandler::new()?;
    let file = fs.open_file(path, FileMode::Read)?;
    fs.read_file(&file)
}

pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir, FilesystemError> {
    let mut fs = FsHandler::new()?;
    fs.open_dir(path)
}

/*

pub struct ReadDir {
    handle: c_wut::FSDirectoryHandle,
    base: PathBuf,
}

impl Iterator for ReadDir {
    type Item = Result<DirEntry, FilesystemError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut io = match FsHandler::new() {
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
        let mut io = FsHandler::new().unwrap();

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
    let mut io = FsHandler::new()?;
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
    let mut io = FsHandler::new()?;
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
    let mut io = FsHandler::new()?;

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
        let mut io = FsHandler::new()?;

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
*/
