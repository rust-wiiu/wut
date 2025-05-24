//! Switch to system overlays.
//!
//! Provides functions to open various Wii U system overlays (foreground applications) such as the Home Menu, Browser, Controller Sync, eManual, etc.
//!
//! Overlays are applications that can be opened while another main application is running. They pause the main application and return focus once closed.
//!
//! # Examples
//!
//! ```rust
//! // Open the Home Menu overlay
//! wut::foreground::home_menu();
//!
//! // Open the Controller Sync overlay
//! wut::foreground::controller_sync();
//! ```

use crate::bindings as c_wut;

pub mod browser;

/// Opens the Home Menu (`Home`-Button).
#[inline]
pub fn home_menu() {
    unsafe {
        c_wut::_SYSDirectlySwitchTo(c_wut::SysAppPFID::SYSAPP_PFID_HOME_MENU);
    }
}

/// Opens a prompt to sync a contoller.
#[inline]
pub fn controller_sync() {
    unsafe {
        c_wut::SYSSwitchToSyncControllerOnHBM();
    }
}

/// Opens the game manual for the current game.
#[inline]
pub fn e_manual() {
    unsafe {
        c_wut::SYSSwitchToEManual();
    }
}

/// Opens the Download Manager.
#[inline]
pub fn download_manager() {
    unsafe {
        c_wut::_SYSSwitchTo(c_wut::SysAppPFID::SYSAPP_PFID_DOWNLOAD_MANAGEMENT);
    }
}

/// Opens the friendlist.
#[inline]
pub fn friendlist() {
    unsafe {
        c_wut::_SYSSwitchTo(c_wut::SysAppPFID::SYSAPP_PFID_FRIENDLIST);
    }
}

/// Opens TVii.
#[inline]
pub fn tvii() {
    unsafe {
        c_wut::_SYSSwitchTo(c_wut::SysAppPFID::SYSAPP_PFID_TVII);
    }
}

/// Opens the Miiverse.
#[inline]
pub fn miiverse() {
    unsafe {
        c_wut::_SYSSwitchTo(c_wut::SysAppPFID::SYSAPP_PFID_MIIVERSE);
    }
}

/// Opens the eShop.
#[inline]
pub fn e_shop() {
    unsafe {
        c_wut::SYSSwitchToEShop(core::ptr::null_mut());
    }
}
