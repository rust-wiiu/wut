//! Errno module
//!
//! Map errno to Rust enum

use crate::bindings as c_wut;
use thiserror::Error;

#[derive(Debug, Error)]
#[repr(i32)]
pub enum SystemError {
    #[error("Not owner")]
    PermissionInvalid = c_wut::EPERM as i32,
    #[error("No such file or directory")]
    NoEntry = c_wut::ENOENT as i32,
    #[error("No such process")]
    NoSuchProcess = c_wut::ESRCH as i32,
    #[error("Interrupted system call")]
    Interrupted = c_wut::EINTR as i32,
    #[error("I/O error")]
    Io = c_wut::EIO as i32,
    #[error("No such device or address")]
    NoSuchDeviceOrAddress = c_wut::ENXIO as i32,
    #[error("Argument list too long")]
    ArgumentListTooLong = c_wut::E2BIG as i32,
    #[error("Exec format error")]
    ExecFormatError = c_wut::ENOEXEC as i32,
    #[error("Bad file number")]
    BadFileNumber = c_wut::EBADF as i32,
    #[error("No children")]
    NoChildren = c_wut::ECHILD as i32,
    #[error("No more processes | Operation would block")]
    NoMoreProcesses = c_wut::EAGAIN as i32,
    #[error("Not enough space")]
    NotEnoughSpace = c_wut::ENOMEM as i32,
    #[error("Permission denied")]
    PermissionDeniedAgain = c_wut::EACCES as i32,
    #[error("Bad address")]
    BadAddress = c_wut::EFAULT as i32,
    #[error("Device or resource busy")]
    Busy = c_wut::EBUSY as i32,
    #[error("File exists")]
    Exist = c_wut::EEXIST as i32,
    #[error("Cross-device link")]
    CrossDeviceLink = c_wut::EXDEV as i32,
    #[error("No such device")]
    NoDevice = c_wut::ENODEV as i32,
    #[error("Not a directory")]
    NotDirectory = c_wut::ENOTDIR as i32,
    #[error("Is a directory")]
    IsDirectory = c_wut::EISDIR as i32,
    #[error("Invalid argument")]
    InvalidArgument = c_wut::EINVAL as i32,
    #[error("Too many open files in system")]
    TooManyOpenFiles = c_wut::ENFILE as i32,
    #[error("File descriptor value too large")]
    FileDescriptorTooLarge = c_wut::EMFILE as i32,
    #[error("Not a character device")]
    NotCharacterDevice = c_wut::ENOTTY as i32,
    #[error("Text file busy")]
    TextFileBusy = c_wut::ETXTBSY as i32,
    #[error("File too large")]
    FileTooLarge = c_wut::EFBIG as i32,
    #[error("No space left on device")]
    NoSpaceLeft = c_wut::ENOSPC as i32,
    #[error("Illegal seek")]
    IllegalSeek = c_wut::ESPIPE as i32,
    #[error("Read-only file system")]
    ReadOnlyFileSystem = c_wut::EROFS as i32,
    #[error("Too many links")]
    TooManyLinks = c_wut::EMLINK as i32,
    #[error("Broken pipe")]
    BrokenPipe = c_wut::EPIPE as i32,
    #[error("Mathematics argument out of domain of function")]
    OutOfDomain = c_wut::EDOM as i32,
    #[error("Result too large")]
    ResultTooLarge = c_wut::ERANGE as i32,
    #[error("No message of desired type")]
    NoMessage = c_wut::ENOMSG as i32,
    #[error("Identifier removed")]
    IdentifierRemoved = c_wut::EIDRM as i32,
    #[error("Deadlock")]
    Deadlock = c_wut::EDEADLK as i32,
    #[error("No lock")]
    NoLock = c_wut::ENOLCK as i32,
    #[error("Not a stream")]
    NotStream = c_wut::ENOSTR as i32,
    #[error("No data (for no delay io)")]
    NoData = c_wut::ENODATA as i32,
    #[error("Stream ioctl timeout")]
    StreamIoctlTimeout = c_wut::ETIME as i32,
    #[error("No stream resources")]
    NoStreamResources = c_wut::ENOSR as i32,
    #[error("Virtual circuit is gone")]
    NoLink = c_wut::ENOLINK as i32,
    #[error("Protocol error")]
    ProtocolError = c_wut::EPROTO as i32,
    #[error("Multihop attempted")]
    MultihopAttempted = c_wut::EMULTIHOP as i32,
    #[error("Bad message")]
    BadMessage = c_wut::EBADMSG as i32,
    #[error("Inappropriate file type or format")]
    InappropriateFile = c_wut::EFTYPE as i32,
    #[error("Function not implemented")]
    FunctionNotImplemented = c_wut::ENOSYS as i32,
    #[error("Directory not empty")]
    NotEmpty = c_wut::ENOTEMPTY as i32,
    #[error("File or path name too long")]
    NameTooLong = c_wut::ENAMETOOLONG as i32,
    #[error("Too many symbolic links")]
    TooManySymbolicLinks = c_wut::ELOOP as i32,
    #[error("Operation not supported on socket")]
    OperationNotSupportedOnSocket = c_wut::EOPNOTSUPP as i32,
    #[error("Protocol family not supported")]
    ProtocolFamilyNotSupported = c_wut::EPFNOSUPPORT as i32,
    #[error("Connection reset by peer")]
    ConnectionReset = c_wut::ECONNRESET as i32,
    #[error("No buffer space available")]
    NoBufferSpace = c_wut::ENOBUFS as i32,
    #[error("Address family not supported by protocol family")]
    AddressFamilyNotSupported = c_wut::EAFNOSUPPORT as i32,
    #[error("Protocol wrong type for socket")]
    ProtocolWrongTypeForSocket = c_wut::EPROTOTYPE as i32,
    #[error("Socket operation on non-socket")]
    NotSocket = c_wut::ENOTSOCK as i32,
    #[error("Protocol not available")]
    ProtocolNotAvailable = c_wut::ENOPROTOOPT as i32,
    #[error("Connection refused")]
    ConnectionRefused = c_wut::ECONNREFUSED as i32,
    #[error("Address already in use")]
    AddressInUse = c_wut::EADDRINUSE as i32,
    #[error("Software caused connection abort")]
    ConnectionAborted = c_wut::ECONNABORTED as i32,
    #[error("Network is unreachable")]
    NetworkUnreachable = c_wut::ENETUNREACH as i32,
    #[error("Network interface is not configured")]
    NetworkDown = c_wut::ENETDOWN as i32,
    #[error("Connection timed out")]
    TimedOut = c_wut::ETIMEDOUT as i32,
    #[error("Host is down")]
    HostDown = c_wut::EHOSTDOWN as i32,
    #[error("Host is unreachable")]
    HostUnreachable = c_wut::EHOSTUNREACH as i32,
    #[error("Connection already in progress")]
    InProgress = c_wut::EINPROGRESS as i32,
    #[error("Socket already connected")]
    AlreadyConnected = c_wut::EALREADY as i32,
    #[error("Destination address required")]
    DestinationAddressRequired = c_wut::EDESTADDRREQ as i32,
    #[error("Message too long")]
    MessageTooLong = c_wut::EMSGSIZE as i32,
    #[error("Unknown protocol")]
    ProtocolNotSupported = c_wut::EPROTONOSUPPORT as i32,
    #[error("Address not available")]
    AddressNotAvailable = c_wut::EADDRNOTAVAIL as i32,
    #[error("Connection aborted by network")]
    NetworkReset = c_wut::ENETRESET as i32,
    #[error("Socket is already connected")]
    IsConnected = c_wut::EISCONN as i32,
    #[error("Socket is not connected")]
    NotConnected = c_wut::ENOTCONN as i32,
    #[error("Too many references: cannot splice.")]
    TooManyReferences = c_wut::ETOOMANYREFS as i32,
    #[error("Disk quota exceeded")]
    DiskQuotaExceeded = c_wut::EDQUOT as i32,
    #[error("Stale file handle")]
    StaleFileHandle = c_wut::ESTALE as i32,
    #[error("Not supported")]
    NotSupported = c_wut::ENOTSUP as i32,
    #[error("Illegal byte sequence")]
    IllegalByteSequence = c_wut::EILSEQ as i32,
    #[error("Value too large for defined data type")]
    Overflow = c_wut::EOVERFLOW as i32,
    #[error("Operation canceled")]
    Canceled = c_wut::ECANCELED as i32,
    #[error("State not recoverable")]
    NotRecoverable = c_wut::ENOTRECOVERABLE as i32,
    #[error("Previous owner died")]
    OwnerDead = c_wut::EOWNERDEAD as i32,
    // #[error("Operation would block")]
    // WouldBlock = c_wut::EWOULDBLOCK as i32,
    #[error("Unkown error")]
    Unknown(i32),
}

impl SystemError {
    fn errno() -> i32 {
        unsafe { *c_wut::__errno() }
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
                c_wut::EPERM => Err(Self::PermissionInvalid),
                c_wut::ENOENT => Err(Self::NoEntry),
                c_wut::ESRCH => Err(Self::NoSuchProcess),
                c_wut::EINTR => Err(Self::Interrupted),
                c_wut::EIO => Err(Self::Io),
                c_wut::ENXIO => Err(Self::NoSuchDeviceOrAddress),
                c_wut::E2BIG => Err(Self::ArgumentListTooLong),
                c_wut::ENOEXEC => Err(Self::ExecFormatError),
                c_wut::EBADF => Err(Self::BadFileNumber),
                c_wut::ECHILD => Err(Self::NoChildren),
                c_wut::EAGAIN => Err(Self::NoMoreProcesses),
                c_wut::ENOMEM => Err(Self::NotEnoughSpace),
                c_wut::EACCES => Err(Self::PermissionDeniedAgain),
                c_wut::EFAULT => Err(Self::BadAddress),
                c_wut::EBUSY => Err(Self::Busy),
                c_wut::EEXIST => Err(Self::Exist),
                c_wut::EXDEV => Err(Self::CrossDeviceLink),
                c_wut::ENODEV => Err(Self::NoDevice),
                c_wut::ENOTDIR => Err(Self::NotDirectory),
                c_wut::EISDIR => Err(Self::IsDirectory),
                c_wut::EINVAL => Err(Self::InvalidArgument),
                c_wut::ENFILE => Err(Self::TooManyOpenFiles),
                c_wut::EMFILE => Err(Self::FileDescriptorTooLarge),
                c_wut::ENOTTY => Err(Self::NotCharacterDevice),
                c_wut::ETXTBSY => Err(Self::TextFileBusy),
                c_wut::EFBIG => Err(Self::FileTooLarge),
                c_wut::ENOSPC => Err(Self::NoSpaceLeft),
                c_wut::ESPIPE => Err(Self::IllegalSeek),
                c_wut::EROFS => Err(Self::ReadOnlyFileSystem),
                c_wut::EMLINK => Err(Self::TooManyLinks),
                c_wut::EPIPE => Err(Self::BrokenPipe),
                c_wut::EDOM => Err(Self::OutOfDomain),
                c_wut::ERANGE => Err(Self::ResultTooLarge),
                c_wut::ENOMSG => Err(Self::NoMessage),
                c_wut::EIDRM => Err(Self::IdentifierRemoved),
                c_wut::EDEADLK => Err(Self::Deadlock),
                c_wut::ENOLCK => Err(Self::NoLock),
                c_wut::ENOSTR => Err(Self::NotStream),
                c_wut::ENODATA => Err(Self::NoData),
                c_wut::ETIME => Err(Self::StreamIoctlTimeout),
                c_wut::ENOSR => Err(Self::NoStreamResources),
                c_wut::ENOLINK => Err(Self::NoLink),
                c_wut::EPROTO => Err(Self::ProtocolError),
                c_wut::EMULTIHOP => Err(Self::MultihopAttempted),
                c_wut::EBADMSG => Err(Self::BadMessage),
                c_wut::EFTYPE => Err(Self::InappropriateFile),
                c_wut::ENOSYS => Err(Self::FunctionNotImplemented),
                c_wut::ENOTEMPTY => Err(Self::NotEmpty),
                c_wut::ENAMETOOLONG => Err(Self::NameTooLong),
                c_wut::ELOOP => Err(Self::TooManySymbolicLinks),
                c_wut::EOPNOTSUPP => Err(Self::OperationNotSupportedOnSocket),
                c_wut::EPFNOSUPPORT => Err(Self::ProtocolFamilyNotSupported),
                c_wut::ECONNRESET => Err(Self::ConnectionReset),
                c_wut::ENOBUFS => Err(Self::NoBufferSpace),
                c_wut::EAFNOSUPPORT => Err(Self::AddressFamilyNotSupported),
                c_wut::EPROTOTYPE => Err(Self::ProtocolWrongTypeForSocket),
                c_wut::ENOTSOCK => Err(Self::NotSocket),
                c_wut::ENOPROTOOPT => Err(Self::ProtocolNotAvailable),
                c_wut::ECONNREFUSED => Err(Self::ConnectionRefused),
                c_wut::EADDRINUSE => Err(Self::AddressInUse),
                c_wut::ECONNABORTED => Err(Self::ConnectionAborted),
                c_wut::ENETUNREACH => Err(Self::NetworkUnreachable),
                c_wut::ENETDOWN => Err(Self::NetworkDown),
                c_wut::ETIMEDOUT => Err(Self::TimedOut),
                c_wut::EHOSTDOWN => Err(Self::HostDown),
                c_wut::EHOSTUNREACH => Err(Self::HostUnreachable),
                c_wut::EINPROGRESS => Err(Self::InProgress),
                c_wut::EALREADY => Err(Self::AlreadyConnected),
                c_wut::EDESTADDRREQ => Err(Self::DestinationAddressRequired),
                c_wut::EMSGSIZE => Err(Self::MessageTooLong),
                c_wut::EPROTONOSUPPORT => Err(Self::ProtocolNotSupported),
                c_wut::EADDRNOTAVAIL => Err(Self::AddressNotAvailable),
                c_wut::ENETRESET => Err(Self::NetworkReset),
                c_wut::EISCONN => Err(Self::IsConnected),
                c_wut::ENOTCONN => Err(Self::NotConnected),
                c_wut::ETOOMANYREFS => Err(Self::TooManyReferences),
                c_wut::EDQUOT => Err(Self::DiskQuotaExceeded),
                c_wut::ESTALE => Err(Self::StaleFileHandle),
                c_wut::ENOTSUP => Err(Self::NotSupported),
                c_wut::EILSEQ => Err(Self::IllegalByteSequence),
                c_wut::EOVERFLOW => Err(Self::Overflow),
                c_wut::ECANCELED => Err(Self::Canceled),
                c_wut::ENOTRECOVERABLE => Err(Self::NotRecoverable),
                c_wut::EOWNERDEAD => Err(Self::OwnerDead),
                // c_wut::EWOULDBLOCK => Err(Self::WouldBlock),
                _ => Err(Self::Unknown(value)),
            }
        }
    }
}
