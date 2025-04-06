use super::{
    color::Color,
    context::{Context, TvOrDrc},
};
use crate::bindings as c_wut;

pub fn fill<S: TvOrDrc>(_context: &Context<S>, color: Color) {
    let (r, g, b, a) = color.into();
    unsafe {
        c_wut::WHBGfxClearColor(r, g, b, a);
    }
}
