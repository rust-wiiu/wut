//! Dynamic loading allocators
//! 
//! Allows for overwriting of dynload allocators. Unless you know exactly what you are doing, never touch these functions.

use crate::bindings as c_wut;

use super::DynamicLoadingError;

pub unsafe fn get_allocator(
) -> Result<(c_wut::OSDynLoadAllocFn, c_wut::OSDynLoadFreeFn), DynamicLoadingError> {
    let mut alloc = c_wut::OSDynLoadAllocFn::default();
    let mut free = c_wut::OSDynLoadFreeFn::default();

    let status = unsafe { c_wut::OSDynLoad_GetAllocator(&mut alloc, &mut free) };
    DynamicLoadingError::try_from(status)?;

    Ok((alloc, free))
}

pub unsafe fn set_allocator(
    alloc: c_wut::OSDynLoadAllocFn,
    free: c_wut::OSDynLoadFreeFn,
) -> Result<(), DynamicLoadingError> {
    let status = unsafe { c_wut::OSDynLoad_SetAllocator(alloc, free) };
    DynamicLoadingError::try_from(status)?;

    Ok(())
}

pub unsafe fn get_tls_allocator(
) -> Result<(c_wut::OSDynLoadAllocFn, c_wut::OSDynLoadFreeFn), DynamicLoadingError> {
    let mut alloc = c_wut::OSDynLoadAllocFn::default();
    let mut free = c_wut::OSDynLoadFreeFn::default();

    let status = unsafe { c_wut::OSDynLoad_GetTLSAllocator(&mut alloc, &mut free) };
    DynamicLoadingError::try_from(status)?;

    Ok((alloc, free))
}

pub unsafe fn set_tls_allocator(
    alloc: c_wut::OSDynLoadAllocFn,
    free: c_wut::OSDynLoadFreeFn,
) -> Result<(), DynamicLoadingError> {
    let status = unsafe { c_wut::OSDynLoad_SetTLSAllocator(alloc, free) };
    DynamicLoadingError::try_from(status)?;

    Ok(())
}
