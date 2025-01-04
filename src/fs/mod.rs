//! Filesystem

mod walkdir;
pub use walkdir::walkdir;

use crate::{
    bindings as c_wut,
    io::_print,
    path::{Path, PathBuf},
    rrc::{ResourceGuard, Rrc},
    time::SystemTime,
};
use alloc::{
    boxed::Box,
    ffi::{CString, NulError},
    string::{String, ToString},
    vec::Vec,
};
use core::{cell::RefCell, ffi::c_void, fmt, str::Utf8Error};
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
    #[error("Pointer was null")]
    NulError(#[from] NulError),
    #[error("Object already exists at path")]
    AlreadyExists,
    #[error("Object was requested as a file but is none")]
    NotAFile,
}

impl TryFrom<i32> for FilesystemError {
    type Error = FilesystemError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use c_wut::FSStatus::*;
        if value > 0 {
            return Ok(Self::Unknown(value));
        }

        match value {
            FS_STATUS_OK => Ok(Self::Unknown(value)),
            FS_STATUS_END => Err(Self::AllRead),
            FS_STATUS_NOT_FOUND => Err(Self::NotFound),
            FS_STATUS_EXISTS => Err(Self::AlreadyExists),
            FS_STATUS_NOT_FILE => Err(Self::NotAFile),
            _ => Err(Self::Unknown(value)),
        }
    }
}

// region: FsHandler

pub struct FsHandler<'a> {
    // not sure why Box is required, but it is - trust me
    // ig think it has something to do with copied/moved memory, which the API apperently doesnt like
    client: Box<RefCell<c_wut::FSClient>>,
    block: Box<RefCell<c_wut::FSCmdBlock>>,
    error_mask: c_wut::FSErrorFlag::Type,
    _resource: ResourceGuard<'a>,
}

impl<'a> FsHandler<'a> {
    pub fn new() -> Result<Self, FilesystemError> {
        let fs = Self {
            client: Box::new(RefCell::new(c_wut::FSClient::default())),
            block: Box::new(RefCell::new(c_wut::FSCmdBlock::default())),
            error_mask: c_wut::FSErrorFlag::FS_ERROR_FLAG_ALL,
            _resource: FS.acquire(),
        };

        let status = unsafe { c_wut::FSAddClient(&mut *fs.client.borrow_mut(), fs.error_mask) };

        FilesystemError::try_from(status)?;

        unsafe {
            c_wut::FSInitCmdBlock(&mut *fs.block.borrow_mut());
        }

        Ok(fs)
    }
}

impl<'a> Drop for FsHandler<'_> {
    fn drop(&mut self) {
        unsafe { c_wut::FSDelClient(&mut *self.client.borrow_mut(), self.error_mask) };
    }
}

// endregion

flags! {
    enum MetadataFlags: c_wut::FSStatFlags::Type {
        /// The retrieved file entry is a (link to a) directory.
        Directory = c_wut::FSStatFlags::FS_STAT_DIRECTORY,
        /// The retrieved file entry also has a quota set.
        Quota = c_wut::FSStatFlags::FS_STAT_QUOTA,
        /// The retrieved file entry is a (link to a) file.
        File = c_wut::FSStatFlags::FS_STAT_FILE,
        /// The retrieved file entry also is encrypted and can't be opened (see vWii files for example).
        Encrypted = c_wut::FSStatFlags::FS_STAT_ENCRYPTED_FILE,
        /// The retrieved file entry also is a link to a different file on the filesystem.
        ///
        /// Note: It's currently not known how one can read the linked-to file entry.
        Link = c_wut::FSStatFlags::FS_STAT_LINK
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

impl fmt::Debug for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_dir() {
            write!(f, "FileType(Dir)")
        } else if self.is_file() {
            write!(f, "FileType(Dir)")
        } else if self.is_symlink() {
            write!(f, "FileType(Symlink)")
        } else {
            write!(f, "FileType(Unknown)")
        }
    }
}

// endregion

// region: Permissions

#[derive(Debug, Default)]
pub struct Permissions {
    pub owner: FlagSet<Mode>,
    pub group: FlagSet<Mode>,
    pub other: FlagSet<Mode>,
}

impl From<c_wut::FSMode::Type> for Permissions {
    fn from(value: c_wut::FSMode::Type) -> Self {
        use c_wut::FSMode::*;
        let mut p = Permissions::default();

        if (value & FS_MODE_READ_OWNER) != 0 {
            p.owner |= Mode::Read;
        }

        if (value & FS_MODE_WRITE_OWNER) != 0 {
            p.owner |= Mode::Write;
        }

        if (value & FS_MODE_EXEC_OWNER) != 0 {
            p.owner |= Mode::Execute;
        }

        if (value & FS_MODE_READ_GROUP) != 0 {
            p.group |= Mode::Read;
        }

        if (value & FS_MODE_WRITE_GROUP) != 0 {
            p.group |= Mode::Write;
        }

        if (value & FS_MODE_EXEC_GROUP) != 0 {
            p.group |= Mode::Execute;
        }

        if (value & FS_MODE_READ_OTHER) != 0 {
            p.other |= Mode::Read;
        }

        if (value & FS_MODE_WRITE_OTHER) != 0 {
            p.other |= Mode::Write;
        }

        if (value & FS_MODE_EXEC_OTHER) != 0 {
            p.other |= Mode::Execute;
        }

        p
    }
}

impl Into<c_wut::FSMode::Type> for Permissions {
    fn into(self) -> c_wut::FSMode::Type {
        use c_wut::FSMode::*;
        let mut m = Type::default();

        if self.owner.contains(Mode::Read) {
            m &= FS_MODE_READ_OWNER;
        }

        if self.owner.contains(Mode::Write) {
            m &= FS_MODE_WRITE_OWNER;
        }

        if self.owner.contains(Mode::Execute) {
            m &= FS_MODE_EXEC_OWNER;
        }

        if self.group.contains(Mode::Read) {
            m &= FS_MODE_READ_GROUP;
        }

        if self.group.contains(Mode::Write) {
            m &= FS_MODE_WRITE_GROUP;
        }

        if self.group.contains(Mode::Execute) {
            m &= FS_MODE_EXEC_GROUP;
        }

        if self.other.contains(Mode::Read) {
            m &= FS_MODE_READ_OTHER;
        }

        if self.other.contains(Mode::Write) {
            m &= FS_MODE_WRITE_OTHER;
        }

        if self.other.contains(Mode::Execute) {
            m &= FS_MODE_EXEC_OTHER;
        }

        m
    }
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for mode in [&self.owner, &self.group, &self.other] {
            s.push(if mode.contains(Mode::Read) { 'r' } else { '-' });
            s.push(if mode.contains(Mode::Write) { 'w' } else { '-' });
            s.push(if mode.contains(Mode::Execute) {
                'x'
            } else {
                '-'
            });
        }

        write!(f, "{s}")
    }
}

// endregion

// region: Metdata

#[derive(Clone, Copy)]
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

    pub fn owner(&self) -> u32 {
        self.0.owner
    }

    pub fn group(&self) -> u32 {
        self.0.group
    }
}

impl From<c_wut::FSStat> for Metadata {
    fn from(value: c_wut::FSStat) -> Self {
        Self(value)
    }
}

// endregion

// region: OpenOptions

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

    pub fn open<'a, P: AsRef<Path>>(&self, path: P) -> Result<File<'a>, FilesystemError> {
        let fs = FsHandler::new()?;
        let str = CString::new(path.as_ref().as_str())?;
        // let mode = CString::new(self.file_mode())?;
        let mode = c"r";
        let mut handle = c_wut::FSFileHandle::default();

        let status = unsafe {
            c_wut::FSOpenFile(
                &mut *fs.client.borrow_mut(),
                &mut *fs.block.borrow_mut(),
                str.as_ptr(),
                mode.as_ptr(),
                &mut handle,
                fs.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(File {
            fs,
            handle,
            path: path.as_ref().to_path_buf(),
        })
    }

    fn file_mode(&self) -> String {
        let mut mode = String::new();

        if self.read && self.write {
            mode.push_str("r+");
        } else if self.read {
            mode.push_str("r");
        } else if self.write {
            if self.truncate {
                mode.push_str("w");
            } else if self.append {
                mode.push_str("a");
            }
        }

        if self.write && self.create && !self.truncate && !self.append {
            // `w+` is appropriate for creating and truncating files
            mode.push_str("+");
        }

        mode
    }
}

// endregion

// region: File

// pub enum FileMode {
//     /// Open for reading. The file must exist.
//     Read,
//     /// Open for writing. Creates an empty file or truncates an existing file.
//     Write,
//     /// Open for appending. Writes data at the end of the file. Creates the file if it does not exist.
//     Append,
//     /// Open for reading and writing. The file must exist.
//     ReadWrite,
//     /// Open for reading and writing. Creates an empty file or truncates an existing file.
//     ReadWriteCreate,
//     /// Open for reading and appending. The file is created if it does not exist.
//     ReadAppendCreate,
// }

// impl FileMode {
//     pub fn as_c_str(&self) -> &CStr {
//         match self {
//             FileMode::Read => c"r",
//             FileMode::Write => c"w",
//             FileMode::Append => c"a",
//             FileMode::ReadWrite => c"r+",
//             FileMode::ReadWriteCreate => c"w+",
//             FileMode::ReadAppendCreate => c"a+",
//         }
//     }
// }

pub struct File<'a> {
    fs: FsHandler<'a>,
    handle: c_wut::FSFileHandle,
    path: PathBuf,
}

// region: FsBuffer

struct FsBuffer {
    data: *mut u8,
    len: u32,
}

impl FsBuffer {
    fn new<T: Into<u32>>(len: T) -> Self {
        let len = len.into();
        Self {
            data: unsafe { c_wut::MEMAllocFromDefaultHeapEx.unwrap()(len, 0x40) } as *mut _,
            len,
        }
    }

    fn as_slice(&self) -> &[u8] {
        unsafe { alloc::slice::from_raw_parts(self.data as *const _, self.len as usize) }
    }
}

impl Drop for FsBuffer {
    fn drop(&mut self) {
        unsafe {
            c_wut::MEMFreeToDefaultHeap.unwrap()(self.data as *mut _);
        }
    }
}

// endregion

impl<'a> File<'a> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, FilesystemError> {
        OpenOptions::new().read(true).open(path)
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, FilesystemError> {
        OpenOptions::new().create(true).open(path)
    }

    // #TODO

    pub fn metadata(&self) -> Result<Metadata, FilesystemError> {
        let mut stat = c_wut::FSStat::default();

        let status = unsafe {
            c_wut::FSGetStatFile(
                &mut *self.fs.client.borrow_mut(),
                &mut *self.fs.block.borrow_mut(),
                self.handle,
                &mut stat,
                self.fs.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(Metadata::from(stat))
    }

    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, FilesystemError> {
        let meta = self.metadata().unwrap();

        let buffer = FsBuffer::new(meta.len() as u32);
        let read = unsafe {
            c_wut::FSReadFile(
                &mut *self.fs.client.borrow_mut(),
                &mut *self.fs.block.borrow_mut(),
                buffer.data,
                1,
                buffer.len,
                self.handle,
                0,
                self.fs.error_mask,
            )
        };
        FilesystemError::try_from(read)?;

        *buf = buffer.as_slice().to_vec();
        Ok(read as usize)
    }

    pub fn read_to_string(&mut self, buf: &mut String) -> Result<usize, FilesystemError> {
        let mut buffer = Vec::new();
        let size = self.read_to_end(&mut buffer)?;
        buf.push_str(&String::from_utf8_lossy(&buffer));
        Ok(size)
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), FilesystemError> {
        let status = unsafe {
            c_wut::FSWriteFile(
                &mut *self.fs.client.borrow_mut(),
                &mut *self.fs.block.borrow_mut(),
                buf.as_ptr() as *mut u8,
                buf.len() as u32,
                1,
                self.handle,
                0,
                self.fs.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        Ok(())
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl Drop for File<'_> {
    fn drop(&mut self) {
        let status = unsafe {
            c_wut::FSCloseFile(
                &mut *self.fs.client.borrow_mut(),
                &mut *self.fs.block.borrow_mut(),
                self.handle,
                self.fs.error_mask,
            )
        };
        FilesystemError::try_from(status).unwrap();
    }
}

impl fmt::Debug for File<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File({})", self.path)
    }
}

// endregion

// region: ReadDir

pub struct ReadDir<'a> {
    fs: FsHandler<'a>,
    handle: c_wut::FSDirectoryHandle,
    path: PathBuf,
}

impl Iterator for ReadDir<'_> {
    type Item = Result<DirEntry, FilesystemError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = c_wut::FSDirectoryEntry::default();

        let status = unsafe {
            c_wut::FSReadDir(
                &mut *self.fs.client.borrow_mut(),
                &mut *self.fs.block.borrow_mut(),
                self.handle,
                &mut entry,
                self.fs.error_mask,
            )
        };

        match FilesystemError::try_from(status) {
            Err(FilesystemError::AllRead) => None,
            Err(e) => Some(Err(e)),
            Ok(_) => {
                let name = PathBuf::try_from(entry.name.as_ptr()).unwrap();
                Some(Ok(DirEntry {
                    metadata: Metadata::from(entry.info),
                    path: self.path.join(name),
                }))
            }
        }
    }
}

impl Drop for ReadDir<'_> {
    fn drop(&mut self) {
        let status = unsafe {
            c_wut::FSCloseDir(
                self.fs.client.get_mut(),
                self.fs.block.get_mut(),
                self.handle,
                self.fs.error_mask,
            )
        };
        FilesystemError::try_from(status).unwrap();
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

    pub fn metadata(&self) -> Metadata {
        self.metadata
    }

    pub fn file_name(&self) -> String {
        match self.path.file_name() {
            Some(n) => n.to_string(),
            None => "".to_string(),
        }
    }

    pub fn file_type(&self) -> FileType {
        self.metadata.file_type()
    }
}

impl fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DirEntry({})", self.path)
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
    let fs = FsHandler::new()?;
    let str = CString::new(path.as_ref().as_str()).unwrap();

    let status = unsafe {
        c_wut::FSMakeDir(
            &mut *fs.client.borrow_mut(),
            &mut *fs.block.borrow_mut(),
            str.as_ptr(),
            fs.error_mask,
        )
    };
    FilesystemError::try_from(status)?;

    Ok(())
}

// #TEST
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<(), FilesystemError> {
    let path = path.as_ref().absolute()?;

    let mut sub = PathBuf::new();
    for component in path.components() {
        sub = sub.join(component);
        if !exists(&sub)? {
            create_dir(&sub)?;
        }
    }

    Ok(())
}

pub fn exists<P: AsRef<Path>>(path: P) -> Result<bool, FilesystemError> {
    let _ = metadata(path)?;
    Ok(true)
}

pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata, FilesystemError> {
    let fs = FsHandler::new()?;
    let str = CString::new(path.as_ref().as_str()).unwrap();
    let mut stat = c_wut::FSStat::default();

    let status = unsafe {
        c_wut::FSGetStat(
            &mut *fs.client.borrow_mut(),
            &mut *fs.block.borrow_mut(),
            str.as_ptr(),
            &mut stat,
            fs.error_mask,
        )
    };
    FilesystemError::try_from(status)?;

    Ok(Metadata::from(stat))
}

// #TEST
pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, FilesystemError> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}

pub fn read_dir<'a, P: AsRef<Path>>(path: P) -> Result<ReadDir<'a>, FilesystemError> {
    let str = CString::new(path.as_ref().as_str())?;

    let fs = FsHandler::new()?;
    let mut handle = c_wut::FSDirectoryHandle::default();

    let status = unsafe {
        c_wut::FSOpenDir(
            &mut *fs.client.borrow_mut(),
            &mut *fs.block.borrow_mut(),
            str.as_ptr(),
            &mut handle,
            fs.error_mask,
        )
    };
    FilesystemError::try_from(status)?;

    Ok(ReadDir {
        fs,
        handle,
        path: path.as_ref().to_path_buf(),
    })
}

// #TEST
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, FilesystemError> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// #TEST
pub fn remove<P: AsRef<Path>>(path: P) -> Result<(), FilesystemError> {
    let fs = FsHandler::new()?;
    let str = CString::new(path.as_ref().as_str())?;

    let status = unsafe {
        c_wut::FSRemove(
            &mut *fs.client.borrow_mut(),
            &mut *fs.block.borrow_mut(),
            str.as_ptr(),
            fs.error_mask,
        )
    };
    FilesystemError::try_from(status)?;

    Ok(())
}

// #TEST
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<(), FilesystemError> {
    let mut path = path.as_ref().absolute()?;

    while path.parent().is_some() {
        remove(&path)?;
        path = path.parent().unwrap().to_path_buf();
    }

    Ok(())
}

// pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<(), FilesystemError> {
//     let mut fs = FsHandler::new()?;
//     fs.remove(path)
// }

// #TEST
pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), FilesystemError> {
    let fs = FsHandler::new()?;
    let from = CString::new(from.as_ref().as_str())?;
    let to = CString::new(to.as_ref().as_str())?;

    let status = unsafe {
        c_wut::FSRename(
            &mut *fs.client.borrow_mut(),
            &mut *fs.block.borrow_mut(),
            from.as_ptr(),
            to.as_ptr(),
            fs.error_mask,
        )
    };
    FilesystemError::try_from(status)?;

    Ok(())
}

// #TEST
pub fn set_permissions<P: AsRef<Path>>(path: P, perm: Permissions) -> Result<(), FilesystemError> {
    let fs = FsHandler::new()?;
    let str = CString::new(path.as_ref().as_str()).unwrap();
    let mode = perm.into();

    let status = unsafe {
        c_wut::FSChangeMode(
            &mut *fs.client.borrow_mut(),
            &mut *fs.block.borrow_mut(),
            str.as_ptr(),
            mode,
            c_wut::FSMode::Type::MAX,
            fs.error_mask,
        )
    };
    FilesystemError::try_from(status)?;

    Ok(())
}

// #TEST
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<(), FilesystemError> {
    let mut file = File::create(path)?;
    let contents = contents.as_ref().to_vec();
    let _ = file.write_all(&contents)?;
    Ok(())
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

    pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<File, FilesystemError> {
        todo!()
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
