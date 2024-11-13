//! Simple graphics library
//!
//! [...]  is much more straightforward than GX2, which makes it appealing for situations that do not require complex graphics. It can draw text and pixels (one at a time!) to both the Gamepad and TV.
//!
//! This clashes with Stdout::Console on the main screen (TV).

mod color;
mod position;

use crate::alloc::string::String;
use crate::bindings as c_wut;
use crate::{GlobalAlloc, Layout, GLOBAL_ALLOCATOR};
use ::alloc::ffi::CString;
use alloc::alloc::{self};
pub use color::{Color, ColorParseError};
use core::{
    ffi,
    marker::PhantomData,
    sync::atomic::{AtomicU8, Ordering},
};
use position::TextPosition;
// use thiserror::Error;

static OSSCREEN_INSTANCE_COUNT: AtomicU8 = AtomicU8::new(0);

pub(crate) fn _screen_init() {
    if OSSCREEN_INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst) == 0 {
        unsafe {
            c_wut::OSScreenInit();
        }
    }
}

pub(crate) fn _screen_deinit(force: bool) {
    if force || OSSCREEN_INSTANCE_COUNT.fetch_sub(1, Ordering::SeqCst) == 1 {
        unsafe {
            c_wut::OSScreenShutdown();
        }
    }
}

pub struct TV;
pub struct DRC;

pub trait DisplayType {
    fn id() -> u32;
}

impl DisplayType for TV {
    fn id() -> u32 {
        c_wut::SCREEN_TV
    }
}

impl DisplayType for DRC {
    fn id() -> u32 {
        c_wut::SCREEN_DRC
    }
}

pub struct Screen<Display> {
    display: PhantomData<Display>,
    buffer: FrameBuffer,
}

impl<Display: DisplayType> Screen<Display> {
    fn screen(&self) -> u32 {
        Display::id()
    }

    pub fn enable(&self) {
        unsafe {
            c_wut::OSScreenEnableEx(self.screen(), 1);
        }
    }

    pub fn disable(&self) {
        unsafe {
            c_wut::OSScreenEnableEx(self.screen(), 0);
        }
    }

    pub fn fill(&self, color: Color) {
        unsafe {
            c_wut::OSScreenClearBufferEx(self.screen(), color.into());
        }
    }

    pub fn draw(&self) {
        self.buffer.flush();
        unsafe {
            c_wut::OSScreenFlipBuffersEx(self.screen());
        }
    }

    pub fn text(&self, text: &str, position: impl Into<TextPosition>) {
        let text = String::from(text);
        let position: TextPosition = position.into();

        for (line, column, row) in position.format(&text) {
            crate::println!("\"{}\" - {} x {}", line, column, row);
            unsafe {
                c_wut::OSScreenPutFontEx(
                    self.screen(),
                    column,
                    row,
                    CString::new(line).unwrap().as_c_str().as_ptr(),
                );
            }
        }
    }

    fn set_buffer(&self) {
        unsafe {
            c_wut::OSScreenSetBufferEx(self.screen(), self.buffer.get());
        }
    }
}

impl<Display> Drop for Screen<Display> {
    fn drop(&mut self) {
        crate::println!("drop screen");
        _screen_deinit(false);
    }
}

impl Screen<TV> {
    pub fn tv() -> Screen<TV> {
        _screen_init();
        let s = Screen {
            display: PhantomData,
            buffer: FrameBuffer::new(TV::id()),
        };
        s.set_buffer();
        s
    }
}

impl Screen<DRC> {
    pub fn drc() -> Screen<DRC> {
        _screen_init();
        let s = Screen {
            display: PhantomData,
            buffer: FrameBuffer::new(DRC::id()),
        };
        s.set_buffer();
        s
    }
}

struct FrameBuffer {
    data: *mut u8,
    layout: alloc::Layout,
}

impl FrameBuffer {
    fn new(screen: c_wut::OSScreenID) -> Self {
        unsafe {
            let size = c_wut::OSScreenGetBufferSizeEx(screen) as usize;
            let layout = Layout::from_size_align(size, 0x100).unwrap();
            let data = GLOBAL_ALLOCATOR.alloc_zeroed(layout);

            Self { data, layout }
        }
    }

    fn get(&self) -> *mut ffi::c_void {
        self.data as *mut ffi::c_void
    }

    fn flush(&self) {
        unsafe {
            c_wut::DCFlushRange(self.get(), self.layout.size() as u32);
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            GLOBAL_ALLOCATOR.dealloc(self.data, self.layout);
        }
    }
}
