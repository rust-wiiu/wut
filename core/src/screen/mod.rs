//! Simplified graphics library
//!
//! This module offers a more straightforward alternative to GX2, making it ideal for applications that don't require complex graphics. It can draw text and individual pixels sequentially to both the GamePad and TV.
//!
//! #Note
//!
//! Each screen has only one framebuffer. This means any [Screen] instances will share and write to the same framebuffer for their respective display.

mod position;

pub use crate::gx2::color::Color;
use crate::rrc::{Rrc, RrcGuard};
use alloc::{ffi::CString, string::String};
use core::{ffi, marker::PhantomData, ptr};
use position::Position;
pub use position::{TextAlign, TextPosition};
use wut_sys as sys;

pub(crate) static OSSCREEN: Rrc = Rrc::new(
    || unsafe {
        use sys::ProcUICallbackType as T;

        sys::OSScreenInit();
        let _ = _alloc_framebuffer(ptr::null_mut());
        sys::ProcUIRegisterCallback(
            T::PROCUI_CALLBACK_ACQUIRE,
            Some(_alloc_framebuffer),
            ptr::null_mut(),
            100,
        );
        sys::ProcUIRegisterCallback(
            T::PROCUI_CALLBACK_RELEASE,
            Some(_dealloc_framebuffer),
            ptr::null_mut(),
            100,
        );
    },
    || /*unsafe*/ {
        // _dealloc_framebuffer(ptr::null_mut());
        // sys::OSScreenShutdown();
    },
);

struct Framebuffer {
    ptr: *mut ffi::c_void,
    size: u32,
}

impl Framebuffer {
    const fn new() -> Self {
        Framebuffer {
            ptr: ptr::null_mut(),
            size: 0,
        }
    }
}

impl From<(*mut ffi::c_void, u32)> for Framebuffer {
    fn from(value: (*mut ffi::c_void, u32)) -> Self {
        Framebuffer {
            ptr: value.0,
            size: value.1,
        }
    }
}

const FRAMEBUFFER_HEAP_TAG: u32 = 0x8E8B30C2;
static mut FRAMEBUFFER_TV: Framebuffer = Framebuffer::new();
static mut FRAMEBUFFER_DRC: Framebuffer = Framebuffer::new();

unsafe extern "C" fn _alloc_framebuffer(_: *mut ffi::c_void) -> u32 {
    unsafe {
        let heap = sys::MEMGetBaseHeapHandle(sys::MEMBaseHeapType::MEM_BASE_HEAP_MEM1);
        let _ = sys::MEMRecordStateForFrmHeap(heap, FRAMEBUFFER_HEAP_TAG);

        if FRAMEBUFFER_TV.ptr.is_null() {
            let size = sys::OSScreenGetBufferSizeEx(sys::OSScreenID::SCREEN_TV);
            FRAMEBUFFER_TV =
                Framebuffer::from((sys::MEMAllocFromFrmHeapEx(heap, size, 0x100), size));
        }

        if FRAMEBUFFER_DRC.ptr.is_null() {
            let size = sys::OSScreenGetBufferSizeEx(sys::OSScreenID::SCREEN_DRC);
            FRAMEBUFFER_DRC =
                Framebuffer::from((sys::MEMAllocFromFrmHeapEx(heap, size, 0x100), size));
        }

        sys::OSScreenSetBufferEx(sys::OSScreenID::SCREEN_TV, FRAMEBUFFER_TV.ptr);
        sys::OSScreenSetBufferEx(sys::OSScreenID::SCREEN_DRC, FRAMEBUFFER_DRC.ptr);
    }

    0
}

unsafe extern "C" fn _dealloc_framebuffer(_: *mut ffi::c_void) -> u32 {
    unsafe {
        let heap = sys::MEMGetBaseHeapHandle(sys::MEMBaseHeapType::MEM_BASE_HEAP_MEM1);
        let _ = sys::MEMFreeByStateToFrmHeap(heap, FRAMEBUFFER_HEAP_TAG);

        FRAMEBUFFER_TV = Framebuffer::new();
        FRAMEBUFFER_DRC = Framebuffer::new();
    }

    0
}

pub struct TV;
pub struct DRC;

pub trait DisplayType {
    fn id() -> u32;

    fn width() -> u32;

    fn height() -> u32;

    fn resolution() -> (u32, u32);

    fn rows() -> u32;

    fn columns() -> u32;

    fn update();
}

impl DisplayType for TV {
    fn id() -> sys::OSScreenID::Type {
        sys::OSScreenID::SCREEN_TV
    }

    fn width() -> u32 {
        TV::resolution().0
    }

    fn height() -> u32 {
        TV::resolution().1
    }

    fn resolution() -> (u32, u32) {
        use sys::AVMTvResolution as R;
        // it is (theoretically?) possible that no resolution is found?!
        let mut resolution = R::Type::default();

        unsafe {
            sys::AVMGetTVScanMode(&mut resolution);

            if resolution == 0 {
                if sys::AVMReadSystemVideoResConfig(&mut resolution) != 0 {
                    panic!("No resolution was returned by the system");
                }
            }
        }
        match resolution {
            R::AVM_TV_RESOLUTION_576I | R::AVM_TV_RESOLUTION_576P => (720, 576),
            R::AVM_TV_RESOLUTION_480I
            | R::AVM_TV_RESOLUTION_480I_PAL60
            | R::AVM_TV_RESOLUTION_480P => (720, 480),
            R::AVM_TV_RESOLUTION_720P
            | R::AVM_TV_RESOLUTION_720P_3D
            | R::AVM_TV_RESOLUTION_720P_50HZ => (1280, 720),
            R::AVM_TV_RESOLUTION_1080I
            | R::AVM_TV_RESOLUTION_1080I_50HZ
            | R::AVM_TV_RESOLUTION_1080P
            | R::AVM_TV_RESOLUTION_1080P_50HZ => (1920, 1080),
            0 => {
                crate::println!("Fallback to default resolution (1280, 720)");
                (1280, 720)
            }
            _ => panic!("Returned resolution couldn't be matched"),
        }
    }

    fn rows() -> u32 {
        30
    }

    fn columns() -> u32 {
        80
    }

    fn update() {
        unsafe {
            sys::DCFlushRange(FRAMEBUFFER_TV.ptr, FRAMEBUFFER_TV.size);
            sys::OSScreenFlipBuffersEx(Self::id());
        }
    }
}

impl DisplayType for DRC {
    fn id() -> sys::OSScreenID::Type {
        sys::OSScreenID::SCREEN_DRC
    }

    fn width() -> u32 {
        854
    }

    fn height() -> u32 {
        480
    }

    fn resolution() -> (u32, u32) {
        (DRC::width(), DRC::height())
    }

    fn rows() -> u32 {
        20
    }

    fn columns() -> u32 {
        53
    }

    fn update() {
        unsafe {
            sys::DCFlushRange(FRAMEBUFFER_DRC.ptr, FRAMEBUFFER_DRC.size);
            sys::OSScreenFlipBuffersEx(Self::id());
        }
    }
}

pub struct Screen<Display: DisplayType> {
    display: PhantomData<Display>,
    _resource: RrcGuard,
}

impl<Display: DisplayType> Screen<Display> {
    pub fn width(&self) -> u32 {
        Display::width()
    }

    pub fn height(&self) -> u32 {
        Display::height()
    }

    pub fn resolution(&self) -> (u32, u32) {
        Display::resolution()
    }

    pub fn rows(&self) -> u32 {
        Display::rows()
    }

    pub fn columns(&self) -> u32 {
        Display::columns()
    }

    pub fn enable(&self) {
        unsafe {
            sys::OSScreenEnableEx(Display::id(), 1);
        }
    }

    pub fn disable(&self) {
        unsafe {
            sys::OSScreenEnableEx(Display::id(), 0);
        }
    }

    pub fn fill(&self, color: Color) {
        unsafe {
            sys::OSScreenClearBufferEx(Display::id(), color.into());
        }
    }

    pub fn update(&self) {
        Display::update();
    }

    pub fn text<C: Position, R: Position>(&self, text: &str, col: C, row: R, align: TextAlign) {
        // let text = CString::new(text).unwrap();
        let text = String::from(text);
        let col = col.into(Display::columns());
        let mut row = row.into(Display::rows());

        text.split('\n').for_each(move |line| unsafe {
            let c = match align {
                TextAlign::Left => col,
                TextAlign::Center => col.saturating_sub(line.len() as u32 / 2),
                TextAlign::Right => col.saturating_sub(line.len() as u32),
            };

            sys::OSScreenPutFontEx(
                Display::id(),
                c,
                row,
                CString::new(line).unwrap().as_c_str().as_ptr(),
            );
            row += 1;
        });
    }

    pub fn pixel<X: Position, Y: Position>(&self, x: X, y: Y, color: Color) {
        unsafe {
            sys::OSScreenPutPixelEx(
                Display::id(),
                x.into(Display::width()),
                y.into(Display::height()),
                color.into(),
            );
        }
    }
}

pub fn tv() -> Screen<TV> {
    Screen {
        display: PhantomData,
        _resource: OSSCREEN.acquire(),
    }
}

pub fn drc() -> Screen<DRC> {
    Screen {
        display: PhantomData,
        _resource: OSSCREEN.acquire(),
    }
}
