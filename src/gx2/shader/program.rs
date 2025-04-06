use crate::path::Path;
use alloc::{borrow::Cow, vec::Vec};

#[derive(Clone)]
pub struct Program<'a>(Cow<'a, [u8]>);

impl<'a> Program<'a> {
    #[inline]
    pub const fn from(data: &'static [u8]) -> Self {
        Self(Cow::Borrowed(data))
    }

    #[inline]
    pub fn new(data: Vec<u8>) -> Self {
        Self(Cow::Owned(data))
    }

    #[inline]
    pub fn load(path: &Path) -> Self {
        todo!()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn as_raw(&self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub unsafe fn as_inner(&self) -> *mut ::core::ffi::c_void {
        self.as_raw() as *mut _
    }

    #[inline]
    pub fn get(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut [u8] {
        self.0.to_mut()
    }
}
