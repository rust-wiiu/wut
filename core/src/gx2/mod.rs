//! Interface to graphics API
//!
//! GX2 is the graphics API of the Wii U similar to OpenGL.

pub mod color;
mod dialog_utils;
pub mod error_view;
pub mod keyboard;
// pub mod render;
pub mod context;
pub mod screen;
pub mod shader;

pub use context::RenderContext;
pub use dialog_utils::{Controller, Language, Region};
pub use error_view::ErrorView;
pub use keyboard::Keyboard;

use crate::rrc::Rrc;
use wut_sys as sys;

pub(crate) static GFX: Rrc = Rrc::new(
    || unsafe {
        // This should *really* only be called once!
        sys::WHBGfxInit();
    },
    || unsafe {
        sys::WHBGfxShutdown();
    },
);

pub trait Renderable {
    fn render_tv(&self, _context: &context::Context<context::Tv>);
    fn render_drc(&self, _context: &context::Context<context::Drc>);
}
