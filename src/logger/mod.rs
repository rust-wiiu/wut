//! Logging interface over various channels.
//!
//! This module provides a unified logging interface for the Wii U, allowing for logging to multiple channels such as the Cafe logging system, console output, WUMS logging module, and UDP broadcasting.

use crate::{
    bindings as c_wut,
    sync::{ConstMutex, LazyLock, Mutex, MutexError},
};
use alloc::ffi::{CString, NulError};
use flagset::{flags, FlagSet};
use thiserror::Error;
pub use Channel::{Cafe, Console, Module, Udp};

#[derive(Debug)]
struct Logger {
    channels: FlagSet<Channel>,
    counter: u32,
}

static LOGGER: ConstMutex<Logger> = LazyLock::new(|| {
    Mutex::new(Logger {
        channels: FlagSet::new_truncated(0),
        counter: 0,
    })
});

flags! {
    /// Logging channels for the Wii U.
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

/// Errors that can occur during logging operations.
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
    #[error("Underlying mutex returned error")]
    MutexError(#[from] MutexError),
}

/// Initialize the logger with specified channels.
///
/// # Arguments
///
/// `channels` - A set of [channels][Channel] to initialize the logger for.
///
/// # Errors
///
/// Returns an error if the logger is already initialized for a channel, or if any of the initialization functions fail.
pub fn init(channels: impl Into<FlagSet<Channel>>) -> Result<(), LoggerError> {
    let channels: FlagSet<Channel> = channels.into();

    let mut logger = LOGGER.lock()?;

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

/// Initialize the logger for Cafe logging system.
#[inline]
pub fn cafe() -> Result<(), LoggerError> {
    init(Cafe)
}

/// Initialize the logger for console (on screen) output.
#[inline]
pub fn console() -> Result<(), LoggerError> {
    init(Console)
}

/// Initialize the logger for WUMS logging module.
#[inline]
pub fn module() -> Result<(), LoggerError> {
    init(Module)
}

/// Initialize the logger for UDP broadcasting on port 4405.
#[inline]
pub fn udp() -> Result<(), LoggerError> {
    init(Udp)
}

/// Deinitialize the logger.
pub fn deinit() {
    let mut logger = LOGGER.lock().unwrap();

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

/// Print content to the logger.
///
/// # Errors
///
/// Returns an error if the logger is not initialized or if the provided string contains internal null bytes.
pub fn print(text: &str) -> Result<(), LoggerError> {
    let logger = LOGGER.lock()?;

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

/// Prints to the logger output
///
/// # Panics
///
/// Panics if no logger is current initialized or if logger is currently locked elsewhere.
///
/// Writing to the logger can cause an error, which will lead this macro to panic.
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        extern crate alloc;
        use alloc::fmt::format;

        let _ = $crate::logger::print(&format(format_args!($($arg)*))).unwrap();
    }};
}
