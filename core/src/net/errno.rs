//! Errno module
//!
//! Map errno to Rust enum

use thiserror::Error;
use wut_sys as sys;

#[derive(Debug, Error)]
#[repr(i32)]
pub enum SystemError {
    #[error("Not owner")]
    PermissionInvalid = sys::EPERM as i32,
    #[error("No such file or directory")]
    NoEntry = sys::ENOENT as i32,
    #[error("No such process")]
    NoSuchProcess = sys::ESRCH as i32,
    #[error("Interrupted system call")]
    Interrupted = sys::EINTR as i32,
    #[error("I/O error")]
    Io = sys::EIO as i32,
    #[error("No such device or address")]
    NoSuchDeviceOrAddress = sys::ENXIO as i32,
    #[error("Argument list too long")]
    ArgumentListTooLong = sys::E2BIG as i32,
    #[error("Exec format error")]
    ExecFormatError = sys::ENOEXEC as i32,
    #[error("Bad file number")]
    BadFileNumber = sys::EBADF as i32,
    #[error("No children")]
    NoChildren = sys::ECHILD as i32,
    #[error("No more processes | Operation would block")]
    NoMoreProcesses = sys::EAGAIN as i32,
    #[error("Not enough space")]
    NotEnoughSpace = sys::ENOMEM as i32,
    #[error("Permission denied")]
    PermissionDeniedAgain = sys::EACCES as i32,
    #[error("Bad address")]
    BadAddress = sys::EFAULT as i32,
    #[error("Device or resource busy")]
    Busy = sys::EBUSY as i32,
    #[error("File exists")]
    Exist = sys::EEXIST as i32,
    #[error("Cross-device link")]
    CrossDeviceLink = sys::EXDEV as i32,
    #[error("No such device")]
    NoDevice = sys::ENODEV as i32,
    #[error("Not a directory")]
    NotDirectory = sys::ENOTDIR as i32,
    #[error("Is a directory")]
    IsDirectory = sys::EISDIR as i32,
    #[error("Invalid argument")]
    InvalidArgument = sys::EINVAL as i32,
    #[error("Too many open files in system")]
    TooManyOpenFiles = sys::ENFILE as i32,
    #[error("File descriptor value too large")]
    FileDescriptorTooLarge = sys::EMFILE as i32,
    #[error("Not a character device")]
    NotCharacterDevice = sys::ENOTTY as i32,
    #[error("Text file busy")]
    TextFileBusy = sys::ETXTBSY as i32,
    #[error("File too large")]
    FileTooLarge = sys::EFBIG as i32,
    #[error("No space left on device")]
    NoSpaceLeft = sys::ENOSPC as i32,
    #[error("Illegal seek")]
    IllegalSeek = sys::ESPIPE as i32,
    #[error("Read-only file system")]
    ReadOnlyFileSystem = sys::EROFS as i32,
    #[error("Too many links")]
    TooManyLinks = sys::EMLINK as i32,
    #[error("Broken pipe")]
    BrokenPipe = sys::EPIPE as i32,
    #[error("Mathematics argument out of domain of function")]
    OutOfDomain = sys::EDOM as i32,
    #[error("Result too large")]
    ResultTooLarge = sys::ERANGE as i32,
    #[error("No message of desired type")]
    NoMessage = sys::ENOMSG as i32,
    #[error("Identifier removed")]
    IdentifierRemoved = sys::EIDRM as i32,
    #[error("Deadlock")]
    Deadlock = sys::EDEADLK as i32,
    #[error("No lock")]
    NoLock = sys::ENOLCK as i32,
    #[error("Not a stream")]
    NotStream = sys::ENOSTR as i32,
    #[error("No data (for no delay io)")]
    NoData = sys::ENODATA as i32,
    #[error("Stream ioctl timeout")]
    StreamIoctlTimeout = sys::ETIME as i32,
    #[error("No stream resources")]
    NoStreamResources = sys::ENOSR as i32,
    #[error("Virtual circuit is gone")]
    NoLink = sys::ENOLINK as i32,
    #[error("Protocol error")]
    ProtocolError = sys::EPROTO as i32,
    #[error("Multihop attempted")]
    MultihopAttempted = sys::EMULTIHOP as i32,
    #[error("Bad message")]
    BadMessage = sys::EBADMSG as i32,
    #[error("Inappropriate file type or format")]
    InappropriateFile = sys::EFTYPE as i32,
    #[error("Function not implemented")]
    FunctionNotImplemented = sys::ENOSYS as i32,
    #[error("Directory not empty")]
    NotEmpty = sys::ENOTEMPTY as i32,
    #[error("File or path name too long")]
    NameTooLong = sys::ENAMETOOLONG as i32,
    #[error("Too many symbolic links")]
    TooManySymbolicLinks = sys::ELOOP as i32,
    #[error("Operation not supported on socket")]
    OperationNotSupportedOnSocket = sys::EOPNOTSUPP as i32,
    #[error("Protocol family not supported")]
    ProtocolFamilyNotSupported = sys::EPFNOSUPPORT as i32,
    #[error("Connection reset by peer")]
    ConnectionReset = sys::ECONNRESET as i32,
    #[error("No buffer space available")]
    NoBufferSpace = sys::ENOBUFS as i32,
    #[error("Address family not supported by protocol family")]
    AddressFamilyNotSupported = sys::EAFNOSUPPORT as i32,
    #[error("Protocol wrong type for socket")]
    ProtocolWrongTypeForSocket = sys::EPROTOTYPE as i32,
    #[error("Socket operation on non-socket")]
    NotSocket = sys::ENOTSOCK as i32,
    #[error("Protocol not available")]
    ProtocolNotAvailable = sys::ENOPROTOOPT as i32,
    #[error("Connection refused")]
    ConnectionRefused = sys::ECONNREFUSED as i32,
    #[error("Address already in use")]
    AddressInUse = sys::EADDRINUSE as i32,
    #[error("Software caused connection abort")]
    ConnectionAborted = sys::ECONNABORTED as i32,
    #[error("Network is unreachable")]
    NetworkUnreachable = sys::ENETUNREACH as i32,
    #[error("Network interface is not configured")]
    NetworkDown = sys::ENETDOWN as i32,
    #[error("Connection timed out")]
    TimedOut = sys::ETIMEDOUT as i32,
    #[error("Host is down")]
    HostDown = sys::EHOSTDOWN as i32,
    #[error("Host is unreachable")]
    HostUnreachable = sys::EHOSTUNREACH as i32,
    #[error("Connection already in progress")]
    InProgress = sys::EINPROGRESS as i32,
    #[error("Socket already connected")]
    AlreadyConnected = sys::EALREADY as i32,
    #[error("Destination address required")]
    DestinationAddressRequired = sys::EDESTADDRREQ as i32,
    #[error("Message too long")]
    MessageTooLong = sys::EMSGSIZE as i32,
    #[error("Unknown protocol")]
    ProtocolNotSupported = sys::EPROTONOSUPPORT as i32,
    #[error("Address not available")]
    AddressNotAvailable = sys::EADDRNOTAVAIL as i32,
    #[error("Connection aborted by network")]
    NetworkReset = sys::ENETRESET as i32,
    #[error("Socket is already connected")]
    IsConnected = sys::EISCONN as i32,
    #[error("Socket is not connected")]
    NotConnected = sys::ENOTCONN as i32,
    #[error("Too many references: cannot splice.")]
    TooManyReferences = sys::ETOOMANYREFS as i32,
    #[error("Disk quota exceeded")]
    DiskQuotaExceeded = sys::EDQUOT as i32,
    #[error("Stale file handle")]
    StaleFileHandle = sys::ESTALE as i32,
    #[error("Not supported")]
    NotSupported = sys::ENOTSUP as i32,
    #[error("Illegal byte sequence")]
    IllegalByteSequence = sys::EILSEQ as i32,
    #[error("Value too large for defined data type")]
    Overflow = sys::EOVERFLOW as i32,
    #[error("Operation canceled")]
    Canceled = sys::ECANCELED as i32,
    #[error("State not recoverable")]
    NotRecoverable = sys::ENOTRECOVERABLE as i32,
    #[error("Previous owner died")]
    OwnerDead = sys::EOWNERDEAD as i32,
    // #[error("Operation would block")]
    // WouldBlock = sys::EWOULDBLOCK as i32,
    #[error("Unkown error")]
    Unknown(i32),
}

impl SystemError {
    fn errno() -> i32 {
        unsafe { *sys::__errno() }
    }

    pub fn get_last() -> Self {
        match Self::try_from(Self::errno()) {
            Ok(v) => v,
            Err(v) => v,
        }
    }

    pub fn get_last_error() -> Result<(), Self> {
        Err(Self::get_last())
    }
}

impl TryFrom<i32> for SystemError {
    type Error = Self;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            Ok(Self::Unknown(0))
        } else {
            match value as u32 {
                sys::EPERM => Err(Self::PermissionInvalid),
                sys::ENOENT => Err(Self::NoEntry),
                sys::ESRCH => Err(Self::NoSuchProcess),
                sys::EINTR => Err(Self::Interrupted),
                sys::EIO => Err(Self::Io),
                sys::ENXIO => Err(Self::NoSuchDeviceOrAddress),
                sys::E2BIG => Err(Self::ArgumentListTooLong),
                sys::ENOEXEC => Err(Self::ExecFormatError),
                sys::EBADF => Err(Self::BadFileNumber),
                sys::ECHILD => Err(Self::NoChildren),
                sys::EAGAIN => Err(Self::NoMoreProcesses),
                sys::ENOMEM => Err(Self::NotEnoughSpace),
                sys::EACCES => Err(Self::PermissionDeniedAgain),
                sys::EFAULT => Err(Self::BadAddress),
                sys::EBUSY => Err(Self::Busy),
                sys::EEXIST => Err(Self::Exist),
                sys::EXDEV => Err(Self::CrossDeviceLink),
                sys::ENODEV => Err(Self::NoDevice),
                sys::ENOTDIR => Err(Self::NotDirectory),
                sys::EISDIR => Err(Self::IsDirectory),
                sys::EINVAL => Err(Self::InvalidArgument),
                sys::ENFILE => Err(Self::TooManyOpenFiles),
                sys::EMFILE => Err(Self::FileDescriptorTooLarge),
                sys::ENOTTY => Err(Self::NotCharacterDevice),
                sys::ETXTBSY => Err(Self::TextFileBusy),
                sys::EFBIG => Err(Self::FileTooLarge),
                sys::ENOSPC => Err(Self::NoSpaceLeft),
                sys::ESPIPE => Err(Self::IllegalSeek),
                sys::EROFS => Err(Self::ReadOnlyFileSystem),
                sys::EMLINK => Err(Self::TooManyLinks),
                sys::EPIPE => Err(Self::BrokenPipe),
                sys::EDOM => Err(Self::OutOfDomain),
                sys::ERANGE => Err(Self::ResultTooLarge),
                sys::ENOMSG => Err(Self::NoMessage),
                sys::EIDRM => Err(Self::IdentifierRemoved),
                sys::EDEADLK => Err(Self::Deadlock),
                sys::ENOLCK => Err(Self::NoLock),
                sys::ENOSTR => Err(Self::NotStream),
                sys::ENODATA => Err(Self::NoData),
                sys::ETIME => Err(Self::StreamIoctlTimeout),
                sys::ENOSR => Err(Self::NoStreamResources),
                sys::ENOLINK => Err(Self::NoLink),
                sys::EPROTO => Err(Self::ProtocolError),
                sys::EMULTIHOP => Err(Self::MultihopAttempted),
                sys::EBADMSG => Err(Self::BadMessage),
                sys::EFTYPE => Err(Self::InappropriateFile),
                sys::ENOSYS => Err(Self::FunctionNotImplemented),
                sys::ENOTEMPTY => Err(Self::NotEmpty),
                sys::ENAMETOOLONG => Err(Self::NameTooLong),
                sys::ELOOP => Err(Self::TooManySymbolicLinks),
                sys::EOPNOTSUPP => Err(Self::OperationNotSupportedOnSocket),
                sys::EPFNOSUPPORT => Err(Self::ProtocolFamilyNotSupported),
                sys::ECONNRESET => Err(Self::ConnectionReset),
                sys::ENOBUFS => Err(Self::NoBufferSpace),
                sys::EAFNOSUPPORT => Err(Self::AddressFamilyNotSupported),
                sys::EPROTOTYPE => Err(Self::ProtocolWrongTypeForSocket),
                sys::ENOTSOCK => Err(Self::NotSocket),
                sys::ENOPROTOOPT => Err(Self::ProtocolNotAvailable),
                sys::ECONNREFUSED => Err(Self::ConnectionRefused),
                sys::EADDRINUSE => Err(Self::AddressInUse),
                sys::ECONNABORTED => Err(Self::ConnectionAborted),
                sys::ENETUNREACH => Err(Self::NetworkUnreachable),
                sys::ENETDOWN => Err(Self::NetworkDown),
                sys::ETIMEDOUT => Err(Self::TimedOut),
                sys::EHOSTDOWN => Err(Self::HostDown),
                sys::EHOSTUNREACH => Err(Self::HostUnreachable),
                sys::EINPROGRESS => Err(Self::InProgress),
                sys::EALREADY => Err(Self::AlreadyConnected),
                sys::EDESTADDRREQ => Err(Self::DestinationAddressRequired),
                sys::EMSGSIZE => Err(Self::MessageTooLong),
                sys::EPROTONOSUPPORT => Err(Self::ProtocolNotSupported),
                sys::EADDRNOTAVAIL => Err(Self::AddressNotAvailable),
                sys::ENETRESET => Err(Self::NetworkReset),
                sys::EISCONN => Err(Self::IsConnected),
                sys::ENOTCONN => Err(Self::NotConnected),
                sys::ETOOMANYREFS => Err(Self::TooManyReferences),
                sys::EDQUOT => Err(Self::DiskQuotaExceeded),
                sys::ESTALE => Err(Self::StaleFileHandle),
                sys::ENOTSUP => Err(Self::NotSupported),
                sys::EILSEQ => Err(Self::IllegalByteSequence),
                sys::EOVERFLOW => Err(Self::Overflow),
                sys::ECANCELED => Err(Self::Canceled),
                sys::ENOTRECOVERABLE => Err(Self::NotRecoverable),
                sys::EOWNERDEAD => Err(Self::OwnerDead),
                // sys::EWOULDBLOCK => Err(Self::WouldBlock),
                _ => Err(Self::Unknown(value)),
            }
        }
    }
}
