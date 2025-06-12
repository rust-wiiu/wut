//! Gx2 shader pipeline
//!
//! # Example
//!
//! ```
//! use wut::{
//!     gx2::{
//!         color::Color,
//!         context::RenderContext,
//!         screen,
//!         shader::{attribute::Float4, buffer::ResourceFlags, Attribute, Buffer, Program, Shader},
//!     },
//!     process,
//! };
//!
//! static PROGRAM: Program = Program::from(include_bytes!("shader.gsh"));
//!
//! #[derive(ShaderAttributes)]
//! struct MyShader {
//!     #[name = "aPosition"]
//!     pos: Attribute<Float4>,
//! }
//!
//! fn main() {
//!     let mut shader: Shader<MyShader> = Shader::new(0, &PROGRAM).unwrap();
//!
//!     let mut buffer = Buffer::from(
//!         &[Float4::from((1.0, -1.0, 0.0, 1.0))],
//!         ResourceFlags::UsageCPU | ResourceFlags::UsageGPURead,
//!     )
//!     .unwrap();
//!
//!     let context = RenderContext::new();
//!
//!     while process::running() {
//!         {
//!             let mut b = buffer.lock().unwrap();
//!             b[0].x += 0.1;
//!         }
//!
//!         let context = context.ready().tv();
//!
//!         screen::fill(&context, Color::blue());
//!
//!         shader.attributes.pos.set_buffer(&mut buffer);
//!
//!         shader.render(&context);
//!
//!         context.finish();
//!     }
//! }
//! ```

pub mod attribute;
pub mod buffer;
pub mod program;

pub(crate) use super::GFX;

pub use attribute::Attribute;
pub use buffer::Buffer;
pub use program::Program;

use super::context::{Context, TvOrDrc};
use crate::rrc::RrcGuard;
use attribute::Attributes;
use wut_sys as sys;

pub struct Shader<A: Attributes> {
    _group: sys::WHBGfxShaderGroup,
    _resource: RrcGuard,
    pub attributes: A,
}

impl<A: Attributes> Shader<A> {
    pub fn new(index: u32, program: &program::Program) -> Result<Self, ()> {
        let mut _group = sys::WHBGfxShaderGroup::default();
        let _resource = super::GFX.acquire();

        if unsafe { sys::WHBGfxLoadGFDShaderGroup(&mut _group, index, program.as_inner()) } == 0 {
            return Err(());
        }

        let attributes = Attributes::new(&mut _group)?;

        if unsafe { sys::WHBGfxInitFetchShader(&mut _group) } == 0 {
            return Err(());
        }

        Ok(Self {
            _group,
            _resource,
            attributes,
        })
    }

    pub fn render<S: TvOrDrc>(&mut self, _context: &Context<S>) {
        unsafe {
            sys::GX2SetFetchShader(&mut self._group.fetchShader);
            sys::GX2SetVertexShader(self._group.vertexShader);
            sys::GX2SetPixelShader(self._group.pixelShader);
            sys::GX2DrawEx(sys::GX2PrimitiveMode::GX2_PRIMITIVE_MODE_TRIANGLES, 3, 0, 1);
        }
    }
}
