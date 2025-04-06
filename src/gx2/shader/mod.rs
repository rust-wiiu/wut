pub mod attribute;
pub mod buffer;
pub mod program;

pub(crate) use super::GFX;

pub use attribute::Attribute;
pub use buffer::Buffer;
pub use program::Program;

use crate::{bindings as c, rrc::RrcGuard};
use attribute::Attributes;

pub struct Shader<A: Attributes> {
    _group: c::WHBGfxShaderGroup,
    _resource: RrcGuard,
    pub attributes: A,
}

impl<A: Attributes> Shader<A> {
    pub fn new(index: u32, program: &program::Program) -> Result<Self, ()> {
        let mut _group = c::WHBGfxShaderGroup::default();
        let _resource = super::GFX.acquire();

        if unsafe { c::WHBGfxLoadGFDShaderGroup(&mut _group, index, program.as_inner()) } == 0 {
            return Err(());
        }

        let attributes = Attributes::new(&mut _group)?;

        if unsafe { c::WHBGfxInitFetchShader(&mut _group) } == 0 {
            return Err(());
        }

        Ok(Self {
            _group,
            _resource,
            attributes,
        })
    }

    pub fn render(&mut self) {
        unsafe {
            c::GX2SetFetchShader(&mut self._group.fetchShader);
            c::GX2SetVertexShader(self._group.vertexShader);
            c::GX2SetPixelShader(self._group.pixelShader);
            c::GX2DrawEx(c::GX2PrimitiveMode::GX2_PRIMITIVE_MODE_TRIANGLES, 3, 0, 1);
        }
    }
}
