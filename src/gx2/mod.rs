pub mod color;
mod dialog_utils;
pub mod error_view;
pub mod keyboard;
pub mod render_context;
pub mod screen;

pub use dialog_utils::{Controller, Language, Region};
pub use error_view::ErrorView;
pub use keyboard::Keyboard;
pub use render_context::RenderContext;

use crate::{bindings as c_wut, rrc::Rrc};

pub(crate) static GFX: Rrc = Rrc::new(
    || unsafe {
        c_wut::WHBGfxInit();
    },
    || unsafe {
        c_wut::WHBGfxShutdown();
    },
);

// pub(crate) static GX2: Rrc = Rrc::new(
//     || unsafe {
//         c_wut::GX2Init(attributes);
//     },
//     || unsafe {
//         c_wut::WHBGfxShutdown();
//     },
// );

pub trait Renderable {
    fn render_tv(&self, context: &render_context::Context<render_context::Tv>);
    fn render_drc(&self, context: &render_context::Context<render_context::Drc>);
}
