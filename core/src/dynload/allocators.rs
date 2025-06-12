//! Dynamic loading allocators
//!
//! Allows for overwriting of dynload allocators. Unless you know exactly what you are doing, never touch these functions.

use super::DynamicLoadingError;
use wut_sys as sys;

pub unsafe fn get_allocator()
-> Result<(sys::OSDynLoadAllocFn, sys::OSDynLoadFreeFn), DynamicLoadingError> {
    let mut alloc = sys::OSDynLoadAllocFn::default();
    let mut free = sys::OSDynLoadFreeFn::default();

    let status = unsafe { sys::OSDynLoad_GetAllocator(&mut alloc, &mut free) };
    DynamicLoadingError::try_from(status)?;

    Ok((alloc, free))
}

pub unsafe fn set_allocator(
    alloc: sys::OSDynLoadAllocFn,
    free: sys::OSDynLoadFreeFn,
) -> Result<(), DynamicLoadingError> {
    let status = unsafe { sys::OSDynLoad_SetAllocator(alloc, free) };
    DynamicLoadingError::try_from(status)?;

    Ok(())
}

pub unsafe fn get_tls_allocator()
-> Result<(sys::OSDynLoadAllocFn, sys::OSDynLoadFreeFn), DynamicLoadingError> {
    let mut alloc = sys::OSDynLoadAllocFn::default();
    let mut free = sys::OSDynLoadFreeFn::default();

    let status = unsafe { sys::OSDynLoad_GetTLSAllocator(&mut alloc, &mut free) };
    DynamicLoadingError::try_from(status)?;

    Ok((alloc, free))
}

pub unsafe fn set_tls_allocator(
    alloc: sys::OSDynLoadAllocFn,
    free: sys::OSDynLoadFreeFn,
) -> Result<(), DynamicLoadingError> {
    let status = unsafe { sys::OSDynLoad_SetTLSAllocator(alloc, free) };
    DynamicLoadingError::try_from(status)?;

    Ok(())
}
