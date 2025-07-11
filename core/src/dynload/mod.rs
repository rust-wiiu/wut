//! Runtime loading of dynamic libraries (RPLs)
//!
//! This module provides functionality for **loading dynamic/shared libraries (RPL files)** at runtime. RPLs are the Wii U equivalent of `.dll` files on Windows or `.so` files on Linux. They're used to share code and static data between applications.
//!
//! Inspired by the [libloading](https://docs.rs/libloading/latest/libloading/) crate.
//!
//! # Note
//!
//! System RPLs are listed [here](https://wut.devkitpro.org/modules.html). These are typically available via WUT and **don't need to be loaded manually**.
//!
//! # Example:
//!
//! ```
//! use dynload::Module;
//!
//! let m = Module::new("coreinit.rpl").unwrap();
//!
//! let s = m.function::<unsafe fn() -> u64>("OSGetTitleID").unwrap();
//!
//! assert_eq!(unsafe { s() }, wut::title::current_title());
//! ```

use alloc::{boxed::Box, ffi::CString, string::String, vec::Vec};
use core::{ffi, fmt::Debug, marker::PhantomData, ops::Deref};
use thiserror::Error;
use wut_sys as sys;

pub mod allocators;

#[derive(Debug, Error)]
pub enum DynamicLoadingError {
    #[error("Internal 0-byte in name")]
    InvalidString(#[from] alloc::ffi::NulError),
    #[error("Module cannot be loaded due to local memory limitations")]
    OutOfMemory,
    #[error("Provided notification pointer is invalid")]
    InvalidNotifyPointer,
    #[error("Provided module name pointer is invalid")]
    InvalidModuleNamePointer,
    #[error("Provided module name is invalid")]
    InvalidModuleName,
    #[error("Provided acquire pointer is invalid")]
    InvalidAcquirePointer,
    #[error("Provided module name is empty")]
    EmptyModuleName,
    #[error("Underlying allocator is invalid")]
    InvalidAllocationPointer,
    #[error("Module cannot be loaded due to system memory limitations")]
    OutOfSystemMemory,
    #[error("Thread local storage allocator is currently locked")]
    TlsAllocatorLocked,
    #[error("The requested name cannot be found as a module")]
    ModuleNotFound,
    #[error("The requested name cannot be found as a symbol")]
    SymbolNotFound,
    #[error("Pointer returned by module is null")]
    SymbolNullPointer,
    #[error("Unknown error: {0:#x?}")]
    Unknown(u32),
}

impl TryFrom<u32> for DynamicLoadingError {
    type Error = Self;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use sys::OSDynLoad_Error as E;
        match value {
            0 => Ok(Self::Unknown(0)),
            E::OS_DYNLOAD_OUT_OF_MEMORY => Err(Self::OutOfMemory),
            E::OS_DYNLOAD_INVALID_NOTIFY_PTR => Err(Self::InvalidNotifyPointer),
            E::OS_DYNLOAD_INVALID_MODULE_NAME_PTR => Err(Self::InvalidModuleNamePointer),
            E::OS_DYNLOAD_INVALID_MODULE_NAME => Err(Self::InvalidModuleName),
            E::OS_DYNLOAD_INVALID_ACQUIRE_PTR => Err(Self::InvalidAcquirePointer),
            E::OS_DYNLOAD_EMPTY_MODULE_NAME => Err(Self::EmptyModuleName),
            E::OS_DYNLOAD_INVALID_ALLOCATOR_PTR => Err(Self::InvalidAllocationPointer),
            E::OS_DYNLOAD_OUT_OF_SYSTEM_MEMORY => Err(Self::OutOfSystemMemory),
            E::OS_DYNLOAD_TLS_ALLOCATOR_LOCKED => Err(Self::TlsAllocatorLocked),
            E::OS_DYNLOAD_MODULE_NOT_FOUND => Err(Self::ModuleNotFound),
            0xBAD1001D => Err(Self::SymbolNotFound),
            v => Err(Self::Unknown(v)),
        }
    }
}

/// A dynamically loaded module.
///
/// Modules are loaded by name and can be used to access functions and data exported by the module.
pub struct Module(sys::OSDynLoad_Module);

impl Module {
    /// Load or get a module by name. The name must end with `.rpl`.
    ///
    /// # Example
    ///
    /// ```
    /// let module = Module::new("coreinit.rpl")?;
    /// ```
    pub fn new(name: &str) -> Result<Self, DynamicLoadingError> {
        let mut module = core::ptr::null_mut();
        let name = CString::new(name)?;

        let status = unsafe { sys::OSDynLoad_Acquire(name.as_ptr(), &mut module) };
        DynamicLoadingError::try_from(status)?;

        Ok(Self(module))
    }

    pub unsafe fn into_raw(self) -> sys::OSDynLoad_Module {
        self.0
    }

    pub unsafe fn from_raw(raw: sys::OSDynLoad_Module) -> Result<Self, DynamicLoadingError> {
        Ok(Self(raw))
    }

    /// Find a symbol name exported by a module.
    ///
    /// # Errors
    ///
    /// This function will return an error if symbol does not exist.
    fn get<'lib, T>(
        &self,
        name: &str,
        export: sys::OSDynLoad_ExportType::Type,
    ) -> Result<Symbol<'lib, T>, DynamicLoadingError> {
        let mut pointer = core::ptr::null_mut();
        let name = CString::new(name)?;

        let status =
            unsafe { sys::OSDynLoad_FindExport(self.0, export, name.as_ptr(), &mut pointer) };
        DynamicLoadingError::try_from(status)?;

        Symbol::<T>::new(pointer)
    }

    /// Get a function symbol from the module.
    ///
    /// The `name` must match an exported function of the module. The type *must* start with `unsafe extern "C"`!
    ///
    /// # Example
    ///
    /// ```
    /// let module = Module::new("coreinit.rpl")?;
    /// let get_title = module.function::<unsafe fn() -> u64>("OSGetTitleID")?;
    /// let title = unsafe { get_title() };
    /// ```
    #[inline]
    pub fn function<'lib, T>(&self, name: &str) -> Result<Symbol<'lib, T>, DynamicLoadingError> {
        self.get(name, sys::OSDynLoad_ExportType::OS_DYNLOAD_EXPORT_FUNC)
    }

    /// Get a data symbol from the module.
    ///
    /// The `name` must match an exported function of the module. The type *must* start with `*const`!
    ///
    /// # Example
    ///
    /// ```
    /// let module = Module::new("???.rpl")?;
    /// let data = module.function::<*const u32>("???")?;
    /// let value = unsafe { *(*data) };
    /// ```
    #[inline]
    pub fn data<'lib, T>(&self, name: &str) -> Result<Symbol<'lib, T>, DynamicLoadingError> {
        self.get(name, sys::OSDynLoad_ExportType::OS_DYNLOAD_EXPORT_DATA)
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            sys::OSDynLoad_Release(self.0);
        }
    }
}

unsafe impl Send for Module {}
unsafe impl Sync for Module {}

/// A symbol loaded by [Module] via [Module::function] or [Module::data].
///
/// Function symbols can be called like other unsafe functions `unsafe { f() }`. Data symbols can be accessed with a double dereference `unsafe { *(*v) }`.
pub struct Symbol<'lib, T: 'lib> {
    pointer: *const ffi::c_void,
    _marker: PhantomData<&'lib T>,
}

impl<'lib, T> Symbol<'lib, T> {
    /// Create a new symbol from an arbitrary pointer.
    ///
    /// # Errors
    ///
    /// This function will return an error if pointer is null.
    ///
    /// # Safety
    ///
    /// Pointer is assumed to point to a valid function.
    fn new(pointer: *const ffi::c_void) -> Result<Self, DynamicLoadingError> {
        if pointer.is_null() {
            Err(DynamicLoadingError::SymbolNullPointer)
        } else {
            Ok(Self {
                pointer: pointer,
                _marker: PhantomData,
            })
        }
    }

    /// Get the raw pointer to memory location of symbol.
    pub unsafe fn into_raw(self) -> *const ffi::c_void {
        self.pointer
    }
}

impl<T> Deref for Symbol<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.pointer as *const *const _ as *const T) }
    }
}
unsafe impl<'lib, T: Send> Send for Symbol<'lib, T> {}
unsafe impl<'lib, T: Sync> Sync for Symbol<'lib, T> {}

/// Gets the number of currently loaded RPLs.
///
/// Always returns 0 on release versions of CafeOS. Requires OSGetSecurityLevel() > 0 (?).
pub fn loaded_rpls() -> usize {
    unsafe { sys::OSDynLoad_GetNumberOfRPLs() as usize }
}

pub struct RplInfo(sys::OSDynLoad_NotifyData);

impl RplInfo {
    /// Name of the RPL
    #[inline]
    pub fn name(&self) -> String {
        unsafe { ffi::CStr::from_ptr(self.0.name) }
            .to_string_lossy()
            .into_owned()
    }

    /// ...
    #[inline]
    pub fn text_addr(&self) -> u32 {
        self.0.textAddr
    }

    /// ...
    #[inline]
    pub fn text_offset(&self) -> u32 {
        self.0.textOffset
    }

    /// ...
    #[inline]
    pub fn text_size(&self) -> u32 {
        self.0.textSize
    }

    /// ...
    #[inline]
    pub fn data_addr(&self) -> u32 {
        self.0.dataAddr
    }

    /// ...
    #[inline]
    pub fn data_offset(&self) -> u32 {
        self.0.dataOffset
    }

    /// ...
    #[inline]
    pub fn data_size(&self) -> u32 {
        self.0.dataSize
    }

    /// ...
    #[inline]
    pub fn read_addr(&self) -> u32 {
        self.0.readAddr
    }

    /// ...
    #[inline]
    pub fn read_offset(&self) -> u32 {
        self.0.readOffset
    }

    /// ...
    #[inline]
    pub fn read_size(&self) -> u32 {
        self.0.readSize
    }
}

impl Debug for RplInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RplInfo({})", self.name())
    }
}

pub fn rpls_info(index: u32, count: usize) -> Vec<RplInfo> {
    let mut info = Vec::with_capacity(count);

    unsafe { sys::OSDynLoad_GetRPLInfo(index, count as u32, info.as_mut_ptr()) };

    info.into_iter().map(RplInfo).collect()
}

pub struct RplCallback<F>
where
    F: 'static + Fn(Module, NotifyReason, RplInfo) + Send,
{
    f: F,
}

impl<F: 'static + Fn(Module, NotifyReason, RplInfo) + Send> RplCallback<F> {
    pub fn new(f: F) -> Result<Self, DynamicLoadingError> {
        let cb = Self { f };

        let f: Box<Box<&dyn Fn(Module, NotifyReason, RplInfo)>> = Box::new(Box::new(&cb.f));

        let status = unsafe {
            sys::OSDynLoad_AddNotifyCallback(Some(Self::_notify), Box::into_raw(f) as *mut _)
        };
        DynamicLoadingError::try_from(status)?;

        Ok(cb)
    }

    extern "C" fn _notify(
        module: sys::OSDynLoad_Module,
        user_context: *mut ffi::c_void,
        notify_reason: sys::OSDynLoad_NotifyReason::Type,
        infos: *mut sys::OSDynLoad_NotifyData,
    ) {
        let closure = unsafe {
            Box::from_raw(user_context as *mut Box<&dyn Fn(Module, NotifyReason, RplInfo)>)
        };

        let module = unsafe { Module::from_raw(module) }.unwrap();
        let reason = NotifyReason::from(notify_reason);
        let info = RplInfo(unsafe { *infos });

        closure(module, reason, info);
    }
}

impl<F: 'static + Fn(Module, NotifyReason, RplInfo) + Send> Drop for RplCallback<F> {
    fn drop(&mut self) {
        let f: Box<Box<&dyn Fn(Module, NotifyReason, RplInfo)>> = Box::new(Box::new(&self.f));

        let status = unsafe {
            sys::OSDynLoad_DelNotifyCallback(Some(Self::_notify), Box::into_raw(f) as *mut _)
        };
        let _ = DynamicLoadingError::try_from(status).unwrap();
    }
}

pub enum NotifyReason {
    Loaded,
    Unloaded,
}

impl From<sys::OSDynLoad_NotifyReason::Type> for NotifyReason {
    fn from(value: sys::OSDynLoad_NotifyReason::Type) -> Self {
        use sys::OSDynLoad_NotifyReason as T;
        match value {
            T::OS_DYNLOAD_NOTIFY_LOADED => Self::Loaded,
            T::OS_DYNLOAD_NOTIFY_UNLOADED => Self::Unloaded,
            _ => unreachable!(),
        }
    }
}
