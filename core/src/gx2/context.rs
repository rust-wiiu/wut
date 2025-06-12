use crate::rrc::RrcGuard;
use core::marker::PhantomData;
use wut_sys as sys;

pub struct RenderContext {
    _resource: RrcGuard,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            _resource: super::GFX.acquire(),
        }
    }

    pub fn ready(&self) -> Context<Ready> {
        Context::new(self)
    }
}

// region: Context State

pub trait State {
    fn on_ready();
    fn on_finish();
}
pub struct Ready;
pub struct Tv;
pub struct Drc;
pub trait TvOrDrc: State {}

impl State for Ready {
    fn on_ready() {
        unsafe {
            sys::WHBGfxBeginRender();
        }
    }

    fn on_finish() {}
}
impl State for Tv {
    fn on_ready() {
        unsafe {
            sys::WHBGfxBeginRenderTV();
        }
    }

    fn on_finish() {
        unsafe {
            sys::WHBGfxFinishRenderTV();
        }
    }
}
impl State for Drc {
    fn on_ready() {
        unsafe {
            sys::WHBGfxBeginRenderDRC();
        }
    }

    fn on_finish() {
        unsafe {
            sys::WHBGfxFinishRenderDRC();
        }
    }
}

impl TvOrDrc for Tv {}
impl TvOrDrc for Drc {}

// endregion
pub struct Context<'a, S: State> {
    _context: &'a RenderContext,
    _marker: PhantomData<S>,
}

impl<'a, S: State> Context<'a, S> {
    fn transition<T: State>(self) -> Context<'a, T> {
        let c = Context {
            _context: self._context,
            _marker: PhantomData,
        };
        drop(self);
        T::on_ready();
        c
    }
}

impl<'a> Context<'a, Ready> {
    fn new(context: &'a RenderContext) -> Self {
        Context {
            _context: context,
            _marker: PhantomData,
        }
    }

    pub fn drc(self) -> Context<'a, Drc> {
        Context::transition(self)
    }

    pub fn tv(self) -> Context<'a, Tv> {
        Context::transition(self)
    }
}

impl<'a> Context<'a, Drc> {
    pub fn tv(self) -> Context<'a, Tv> {
        Context::transition(self)
    }

    pub fn finish(self) {
        drop(self);
        unsafe {
            sys::WHBGfxFinishRender();
        }
    }
}

impl<'a> Context<'a, Tv> {
    pub fn drc(self) -> Context<'a, Drc> {
        Context::transition(self)
    }

    pub fn finish(self) {
        drop(self);
        unsafe {
            sys::WHBGfxFinishRender();
        }
    }
}

impl<S: State> Drop for Context<'_, S> {
    fn drop(&mut self) {
        S::on_finish();
    }
}

// fn test() {
//     let context = RenderContext::new();

//     loop {
//         // do something
//         let context = context.ready();

//         // do something
//         // call func like (&context);
//         let context = context.tv();

//         // do something
//         // call func like foo(&context);
//         let context = context.drc();

//         // do something
//         context.finish();

//         // do something
//     }
// }
