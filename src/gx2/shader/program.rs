use crate::path::Path;
use alloc::{borrow::Cow, vec::Vec};

/// A shader program.
///
/// Typically the content of a `*.gsh` shader file.
///
/// # Example
///
/// ```
/// static PROGRAM_1: shader::Program = shader::Program::from(&[0x1, 0x2, 0x3]);
/// static PROGRAM_2: shader::Program = shader::Program::from(include_bytes!("shader.gsh"));
///
/// fn main() {
///     let program_3 = shader::Program::load("/wiiu/content/shader.gsh");
///     let program_4 = shader::Program::new(vec![0x1, 0x2, 0x3]);
///
///     let mut shader = shader::Shader::new(0, &PROGRAM_2).unwrap();
/// }
/// ```
#[derive(Clone)]
pub struct Program<'a>(Cow<'a, [u8]>);

impl<'a> Program<'a> {
    /// Create a new static shader program.
    #[inline]
    pub const fn from(data: &'static [u8]) -> Self {
        Self(Cow::Borrowed(data))
    }

    /// Create a new shader program from bytes.
    #[inline]
    pub fn new(data: Vec<u8>) -> Self {
        Self(Cow::Owned(data))
    }

    /// Creates a new shader program from a file.
    #[inline]
    pub fn load(path: &Path) -> Self {
        todo!()
    }

    /// Returns length of shader code in bytes.
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
