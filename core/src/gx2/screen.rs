use super::{
    color::Color,
    context::{Context, TvOrDrc},
};
use wut_sys as sys;

pub fn fill<S: TvOrDrc>(_context: &Context<S>, color: Color) {
    let (r, g, b, a) = color.into();
    unsafe {
        sys::WHBGfxClearColor(r, g, b, a);
    }
}
