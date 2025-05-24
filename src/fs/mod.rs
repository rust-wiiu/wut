//! Filesystem manipulation operations.
//!
//! This module contains basic methods to manipulate the contents of the local filesystem.

mod walkdir;
pub use walkdir::walkdir;

use crate::{
    bindings as c_wut,
    path::{Path, PathBuf},
    rrc::{Rrc, RrcGuard},
    time::DateTime,
};
use alloc::{
    boxed::Box,
    ffi::{CString, NulError},
    string::{String, ToString},
    vec::Vec,
};
use core::{cell::RefCell, ffi, fmt, str::Utf8Error};
use flagset::{flags, FlagSet};
use thiserror::Error;

pub(crate) static FS: Rrc = Rrc::new(
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
    AllRead, // not sure if this also applies to files or just to directories (then maybe change the name to ALlEntriesRead or so)
    #[error("System library call returned unexpected null pointer")]
    NulError(#[from] NulError),
    #[error("Object already exists at path")]
    AlreadyExists,
    #[error("Object was requested as a file but is none")]
    NotAFile,
    #[error("Invalid permissions for action on object")]
    InvalidPermissions,
    #[error("Invalid combination of requested file mode")]
    InvalidModeCombination {
        read: bool,
        write: bool,
        append: bool,
        create: bool,
        truncate: bool,
    },
}

impl TryFrom<i32> for FilesystemError {
    type Error = FilesystemError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use c_wut::FSStatus as S;
        if value > 0 {
            return Ok(Self::Unknown(value));
        }

        match value {
            S::FS_STATUS_OK => Ok(Self::Unknown(value)),
            S::FS_STATUS_END => Err(Self::AllRead),
            S::FS_STATUS_NOT_FOUND => Err(Self::NotFound),
            S::FS_STATUS_EXISTS => Err(Self::AlreadyExists),
            S::FS_STATUS_NOT_FILE => Err(Self::NotAFile),
            S::FS_STATUS_PERMISSION_ERROR => Err(Self::InvalidPermissions),
            _ => Err(Self::Unknown(value)),
        }
    }
}

// region: FsHandler

/// Handler for accessing filesystem
pub struct FsHandler {
    // not sure why Box is required, but it is - trust me
    // ig think it has something to do with copied/moved memory, which the API apperently doesnt like
    pub client: Box<RefCell<c_wut::FSClient>>,
    pub block: Box<RefCell<c_wut::FSCmdBlock>>,
    pub error_mask: c_wut::FSErrorFlag::Type,
    _resource: RrcGuard,
}

impl FsHandler {
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

impl Drop for FsHandler {
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
        use c_wut::FSMode as M;
        let mut p = Permissions::default();

        if (value & M::FS_MODE_READ_OWNER) != 0 {
            p.owner |= Mode::Read;
        }

        if (value & M::FS_MODE_WRITE_OWNER) != 0 {
            p.owner |= Mode::Write;
        }

        if (value & M::FS_MODE_EXEC_OWNER) != 0 {
            p.owner |= Mode::Execute;
        }

        if (value & M::FS_MODE_READ_GROUP) != 0 {
            p.group |= Mode::Read;
        }

        if (value & M::FS_MODE_WRITE_GROUP) != 0 {
            p.group |= Mode::Write;
        }

        if (value & M::FS_MODE_EXEC_GROUP) != 0 {
            p.group |= Mode::Execute;
        }

        if (value & M::FS_MODE_READ_OTHER) != 0 {
            p.other |= Mode::Read;
        }

        if (value & M::FS_MODE_WRITE_OTHER) != 0 {
            p.other |= Mode::Write;
        }

        if (value & M::FS_MODE_EXEC_OTHER) != 0 {
            p.other |= Mode::Execute;
        }

        p
    }
}

impl Into<c_wut::FSMode::Type> for Permissions {
    fn into(self) -> c_wut::FSMode::Type {
        use c_wut::FSMode as M;
        let mut m = M::Type::default();

        if self.owner.contains(Mode::Read) {
            m &= M::FS_MODE_READ_OWNER;
        }

        if self.owner.contains(Mode::Write) {
            m &= M::FS_MODE_WRITE_OWNER;
        }

        if self.owner.contains(Mode::Execute) {
            m &= M::FS_MODE_EXEC_OWNER;
        }

        if self.group.contains(Mode::Read) {
            m &= M::FS_MODE_READ_GROUP;
        }

        if self.group.contains(Mode::Write) {
            m &= M::FS_MODE_WRITE_GROUP;
        }

        if self.group.contains(Mode::Execute) {
            m &= M::FS_MODE_EXEC_GROUP;
        }

        if self.other.contains(Mode::Read) {
            m &= M::FS_MODE_READ_OTHER;
        }

        if self.other.contains(Mode::Write) {
            m &= M::FS_MODE_WRITE_OTHER;
        }

        if self.other.contains(Mode::Execute) {
            m &= M::FS_MODE_EXEC_OTHER;
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
    #[inline]
    pub fn created(&self) -> Result<DateTime, FilesystemError> {
        let mut cal = c_wut::OSCalendarTime::default();
        unsafe {
            c_wut::FSTimeToCalendarTime(self.0.created, &mut cal);
        }
        Ok(DateTime::from(cal))
    }

    #[inline]
    pub fn modified(&self) -> Result<DateTime, FilesystemError> {
        let mut cal = c_wut::OSCalendarTime::default();
        unsafe {
            c_wut::FSTimeToCalendarTime(self.0.modified, &mut cal);
        }
        Ok(DateTime::from(cal))
    }

    #[inline]
    pub fn file_type(&self) -> FileType {
        FileType(FlagSet::<MetadataFlags>::new_truncated(self.0.flags))
    }

    #[inline]
    pub fn is_dir(&self) -> bool {
        self.file_type().is_dir()
    }

    #[inline]
    pub fn is_file(&self) -> bool {
        self.file_type().is_file()
    }

    #[inline]
    pub fn is_symlink(&self) -> bool {
        self.file_type().is_symlink()
    }

    #[inline]
    pub fn len(&self) -> u64 {
        self.0.size as u64
    }

    #[inline]
    pub fn permissions(&self) -> Permissions {
        Permissions::from(self.0.mode)
    }

    #[inline]
    pub fn owner(&self) -> u32 {
        self.0.owner
    }

    #[inline]
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

/// Options and flags which can be used to configure how a file is opened.
///
/// This builder exposes the ability to configure how a [`File`] is opened and what operations are permitted on the open file. The [`File::open`] and [`File::create`] methods are aliases for commonly used options using this builder.
///
/// Generally speaking, when using `OpenOptions`, you'll first call [`OpenOptions::new`], then chain calls to methods to set each option, then call [`OpenOptions::open`], passing the path of the file you're trying to open. This will give you a [`Result`] with a [`File`] inside that you can further operate on.
///
/// ## Valid combination
/// - read
/// - create
/// - read, write
/// - (write), append, create
/// - write, create, truncate
/// - read, write, create, truncate
/// - read, (write), append, create
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

    /// Sets the option for read access.
    ///
    /// This option, when true, will indicate that the file should be `read`-able if opened.
    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }

    /// Sets the option for write access.
    ///
    /// This option, when true, will indicate that the file should be `write`-able if opened.
    ///
    /// If the file already exists, any write calls on it will overwrite its contents, without truncating it.
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    /// Sets the option for the append mode.
    ///
    /// This option, when true, means that writes will append to a file instead of overwriting previous contents.
    /// Note that setting `.write(true).append(true)` has the same effect as setting only `.append(true)`.
    ///
    /// ## Note
    ///
    /// This function doesn't create the file if it doesn't exist. Use the [`OpenOptions::create`] method to do so.
    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        // if append {
        //     self.write = true;
        // }
        self
    }

    /// Sets the option for truncating a previous file.
    ///
    /// If a file is successfully opened with this option set it will truncate the file to 0 length if it already exists.
    ///
    /// The file must be opened with write access for truncate to work.
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        // if truncate {
        //     self.write = true;
        // }
        self
    }

    /// Sets the option to create a new file, or open it if it already exists.
    ///
    /// In order for the file to be created, [`OpenOptions::write`] or [`OpenOptions::append`] access must be used.
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<File, FilesystemError> {
        let fs = FsHandler::new()?;
        let str = CString::new(path.as_ref().as_str())?;
        let mode = self.file_mode()?;
        let mut handle = c_wut::FSFileHandle::default();

        crate::println!("mode: {:?}", mode);

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

    fn file_mode(&self) -> Result<&ffi::CStr, FilesystemError> {
        match (
            self.read,
            self.write,
            self.append,
            self.create,
            self.truncate,
        ) {
            // based on: https://www.tutorialspoint.com/c_standard_library/c_function_fopen.htm
            (true, false, false, false, false) => Ok(c"r"),
            (false, true, false, true, true) | (false, false, false, true, false) => Ok(c"w"),
            (false, _, true, true, false) => Ok(c"a"),
            (true, true, _, false, false) => Ok(c"r+"),
            (true, true, _, true, true) => Ok(c"w+"),
            (true, _, true, true, false) => Ok(c"a+"),
            (read, write, append, create, truncate) => {
                Err(FilesystemError::InvalidModeCombination {
                    read,
                    write,
                    append,
                    create,
                    truncate,
                })
            }
        }
    }
}

// endregion

// region: File

#[derive(Debug, Clone, Copy)]
pub enum SeekFrom {
    Start(u32),
    End(i32),
    Current(i32),
}

pub struct File {
    fs: FsHandler,
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

    fn from(data: &[u8]) -> Self {
        let b = Self::new(data.len() as u32);
        unsafe {
            core::ptr::copy(data.as_ptr(), b.data, b.len as usize);
        }
        b
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

impl File {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, FilesystemError> {
        OpenOptions::new().read(true).open(path)
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, FilesystemError> {
        OpenOptions::new().create(true).open(path)
    }

    pub fn options() -> OpenOptions {
        OpenOptions::new()
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

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

    /// Reads all bytes from seeker until EOF in this source, appending them onto `buf`.
    ///
    /// If successful, this function will return the total number of bytes read. The seeker will be moved by the length of `buf` backwards?
    ///
    /// # Errors
    ///
    /// If any read error is encountered then this function immediately returns. No bytes will be written to `buf`.
    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, FilesystemError> {
        let total_len = self.metadata().unwrap().len();
        let current_pos = self.seek_position()?;

        let buffer = FsBuffer::new(total_len as u32 - current_pos);
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

        buf.extend_from_slice(buffer.as_slice());
        Ok(read as usize)
    }

    /// Reads all bytes from seeker until EOF in this source, converting them into a String, and appending them onto `buf`.
    ///
    /// If successful, this function will return the total number of bytes read. The seeker will be moved by the length of `buf` backwards?
    ///
    /// # Errors
    ///
    /// If any read error is encountered then this function immediately returns. No bytes will be written to `buf`.
    pub fn read_to_string(&mut self, buf: &mut String) -> Result<usize, FilesystemError> {
        let mut buffer = Vec::new();
        let size = self.read_to_end(&mut buffer)?;
        buf.push_str(&String::from_utf8_lossy(&buffer));
        Ok(size)
    }

    /// Writes all bytes from seeker onwards in this source.
    ///
    /// If successful, this function will return the total number of bytes written. The seeker will be moved by the length of `buf` backwards?
    ///
    /// # Errors
    ///
    /// If any write error is encountered then this function immediately returns. Bytes may be partially written to this source.
    pub fn write_all(&mut self, buf: &[u8]) -> Result<u32, FilesystemError> {
        crate::println!("{}", buf.len());

        let buffer = FsBuffer::from(buf);
        let written = unsafe {
            c_wut::FSWriteFile(
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
        FilesystemError::try_from(written)?;

        Ok(written as u32)
    }

    /// Seek to an offset, in bytes, in a stream.
    ///
    /// A seek beyond the end of a stream will be clipped to the end of the file.
    ///
    /// If the seek operation completed successfully, this method returns the new position from the start of the stream. That position can be used later with [`SeekFrom::Start`].
    ///
    /// # Errors
    ///
    /// Seeking can fail, for example because it involves file access.
    pub fn seek(&mut self, pos: SeekFrom) -> Result<u32, self::FilesystemError> {
        let mut current = 0;
        let meta = self.metadata()?;

        let status = unsafe {
            c_wut::FSGetPosFile(
                &mut *self.fs.client.borrow_mut(),
                &mut *self.fs.block.borrow_mut(),
                self.handle,
                &mut current,
                self.fs.error_mask,
            )
        };
        FilesystemError::try_from(status)?;

        let new = match pos {
            SeekFrom::Start(v) => v,
            SeekFrom::Current(v) => (current as i32 + v) as u32,
            SeekFrom::End(v) => {
                let len = meta.len();
                (len as i32 + v) as u32
            }
        }
        .clamp(0, meta.len() as u32);

        if new != current {
            let status = unsafe {
                c_wut::FSSetPosFile(
                    &mut *self.fs.client.borrow_mut(),
                    &mut *self.fs.block.borrow_mut(),
                    self.handle,
                    new,
                    self.fs.error_mask,
                )
            };
            FilesystemError::try_from(status)?;
        }

        Ok(new)
    }

    /// Rewind to the beginning of the file.
    ///
    /// This is equivalent to `self.seek(SeekFrom::Start(0))`.
    pub fn rewind(&mut self) -> Result<(), FilesystemError> {
        self.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    /// Returns the current seek position from the start of the file.
    ///
    /// This is equivalent to `self.seek(SeekFrom::Current(0))`.
    pub fn seek_position(&mut self) -> Result<u32, FilesystemError> {
        self.seek(SeekFrom::Current(0))
    }

    pub fn truncate(&mut self) -> Result<(), FilesystemError> {
        let status = unsafe {
            c_wut::FSTruncateFile(
                &mut *self.fs.client.borrow_mut(),
                &mut *self.fs.block.borrow_mut(),
                self.handle,
                self.fs.error_mask,
            )
        };
        FilesystemError::try_from(status)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), FilesystemError> {
        let status = unsafe {
            c_wut::FSFlushFile(
                &mut *self.fs.client.borrow_mut(),
                &mut *self.fs.block.borrow_mut(),
                self.handle,
                self.fs.error_mask,
            )
        };
        FilesystemError::try_from(status)?;
        Ok(())
    }
}

impl Drop for File {
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

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File({})", self.path)
    }
}

// endregion

// region: ReadDir

pub struct ReadDir {
    fs: FsHandler,
    handle: c_wut::FSDirectoryHandle,
    path: PathBuf,
}

impl Iterator for ReadDir {
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

impl Drop for ReadDir {
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

pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(_from: P, _to: Q) -> Result<u64, FilesystemError> {
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

/// Reads the entire contents of a file into a bytes vector.
///
/// This is a convenience function for using [`File::open`] and [`File::read_to_end`] with fewer imports and without an intermediate variable.
///
/// # Examples
///
/// ```no_run
/// use wut::fs;
///
/// fn foo() -> Result<(), fs::FilesystemError> {
///     let content: Vec<u8> = fs::read("file.jpg")?;
/// }
/// ```
pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, FilesystemError> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}

pub fn read_dir<'a, P: AsRef<Path>>(path: P) -> Result<ReadDir, FilesystemError> {
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

pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, FilesystemError> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

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

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<(), FilesystemError> {
    let mut file = File::create(path)?;
    let contents = contents.as_ref().to_vec();
    let _ = file.write_all(&contents)?;
    Ok(())
}
