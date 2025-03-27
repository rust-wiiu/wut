// logger

use crate::{bindings as c_wut, sync::RwLock};
use alloc::ffi::{CString, NulError};
use flagset::{flags, FlagSet};
use thiserror::Error;
pub use Channel::{Cafe, Console, Module, Udp};

#[derive(Debug)]
struct Logger {
    channels: FlagSet<Channel>,
    counter: u32,
}

static LOGGER: RwLock<Logger> = RwLock::new(Logger {
    channels: unsafe { FlagSet::new_unchecked(0) },
    counter: 0,
});

flags! {
    pub enum Channel: u8 {
        /// Default Wii U logging system
        Cafe,
        /// Write to screen. Requires exclusive access over screen.
        Console,
        /// Write to WUMS Logging Module.
        Module,
        /// Broadcast to UDP on port 4405
        Udp
    }
}

#[derive(Debug, Error)]
pub enum LoggerError {
    #[error("No logging channel is initialized")]
    Uninitialized,
    #[error("Initializing Cafe logging failed")]
    CafeFailed,
    #[error("Initializing Console logging failed")]
    ConsoleFailed,
    #[error("Initializing WUMS Module logging failed")]
    ModuleFailed,
    #[error("Initializing UDP logging failed")]
    UdpFailed,
    #[error("Provided string cannot contain internal 0-bytes")]
    ContainsZeroBytes(#[from] NulError),
}

pub fn init(channels: impl Into<FlagSet<Channel>>) -> Result<(), LoggerError> {
    let channels: FlagSet<Channel> = channels.into();

    let mut logger = LOGGER.write();
    let new = (channels ^ logger.channels) & channels;

    unsafe {
        if new.contains(Channel::Cafe) && c_wut::WHBLogCafeInit() == 0 {
            return Err(LoggerError::CafeFailed);
        }
        if new.contains(Channel::Console) {
            if c_wut::WHBLogConsoleInit() != 0 {
                return Err(LoggerError::ConsoleFailed);
            } else {
                c_wut::WHBLogConsoleSetColor(0xFF000000);
            }
        }
        if new.contains(Channel::Module) && c_wut::WHBLogModuleInit() == 0 {
            return Err(LoggerError::ModuleFailed);
        }
        if new.contains(Channel::Udp) && c_wut::WHBLogUdpInit() == 0 {
            return Err(LoggerError::UdpFailed);
        }
    }

    logger.channels = channels;
    logger.counter += 1;

    Ok(())
}

#[inline]
pub fn cafe() -> Result<(), LoggerError> {
    init(Cafe)
}

#[inline]
pub fn console() -> Result<(), LoggerError> {
    init(Console)
}

#[inline]
pub fn module() -> Result<(), LoggerError> {
    init(Module)
}

#[inline]
pub fn udp() -> Result<(), LoggerError> {
    init(Udp)
}

pub fn deinit() {
    let mut logger = LOGGER.write();

    logger.counter -= 1;

    if logger.counter > 0 {
        return;
    }

    unsafe {
        if logger.channels.contains(Channel::Cafe) {
            c_wut::WHBLogCafeDeinit();
        }

        if logger.channels.contains(Channel::Console) {
            c_wut::WHBLogConsoleFree();
        }

        if logger.channels.contains(Channel::Module) {
            c_wut::WHBLogModuleDeinit();
        }
        if logger.channels.contains(Channel::Udp) {
            c_wut::WHBLogUdpDeinit();
        }
    }

    logger.channels = FlagSet::new(0).unwrap();
    logger.counter = 0;
}

pub fn print(text: &str) -> Result<(), LoggerError> {
    let logger = LOGGER.read();

    if logger.channels.is_empty() {
        Err(LoggerError::Uninitialized)
    } else {
        let text = CString::new(text)?;

        unsafe {
            c_wut::WHBLogPrint(text.as_ptr());

            if logger.channels.contains(Channel::Console) {
                c_wut::WHBLogConsoleDraw();
            }
            if logger.channels.contains(Channel::Cafe) {
                c_wut::OSReport(text.as_ptr());
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        extern crate alloc;
        use alloc::fmt::format;

        let _ = $crate::logger::print(&format(format_args!($($arg)*))).expect("println! failed");
    }};
}
