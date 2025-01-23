// logger

use crate::bindings as c_wut;
use alloc::ffi::{CString, NulError};
use core::sync::atomic::{AtomicU8, Ordering};
use flagset::{flags, FlagSet};
use thiserror::Error;
pub use Channel::{Cafe, Console, Module, Udp};

pub(crate) static mut LOGGER: AtomicU8 = AtomicU8::new(0);

flags! {
    pub enum Channel: u8 {
        /// Default Wii U logging system
        Cafe,
        /// Write to screen. Requires exclusive access over screen.
        Console,
        /// Write to WUMS Logging Module.
        Module,
        /// Write to UDP on port ...
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

    unsafe {
        let logger = FlagSet::new_truncated(LOGGER.load(Ordering::Relaxed));
        let new = channels ^ logger;

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

        LOGGER.store(FlagSet::bits(channels), Ordering::Relaxed);
    }

    Ok(())
}

pub fn deinit() {
    unsafe {
        let channels = FlagSet::new_truncated(LOGGER.swap(0, Ordering::Relaxed));

        if channels.contains(Channel::Cafe) {
            c_wut::WHBLogCafeDeinit();
        }

        if channels.contains(Channel::Console) {
            c_wut::WHBLogConsoleFree();
        }

        if channels.contains(Channel::Module) {
            c_wut::WHBLogModuleDeinit();
        }
        if channels.contains(Channel::Udp) {
            c_wut::WHBLogUdpDeinit();
        }
    }
}

pub fn print(text: &str) -> Result<(), LoggerError> {
    let text = CString::new(text)?;

    unsafe {
        let logger: FlagSet<Channel> = FlagSet::new_unchecked(LOGGER.load(Ordering::Relaxed));

        if logger.is_empty() {
            Err(LoggerError::Uninitialized)
        } else {
            c_wut::WHBLogPrint(text.as_ptr());

            if logger.contains(Channel::Console) {
                c_wut::WHBLogConsoleDraw();
            }

            Ok(())
        }
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        // extern crate alloc;
        use alloc::fmt::format;

        let _ = $crate::logger::print(&format(format_args!($($arg)*))).expect("println! failed");
    }};
}
