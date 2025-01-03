//! Simple graphics library
//!
//! [...]  is much more straightforward than GX2, which makes it appealing for situations that do not require complex graphics. It can draw text and pixels (one at a time!) to both the Gamepad and TV.
//!
//! There is only one framebuffer per screen to write to so multiple instances of Screen<...> will write to the same framebuffer.

mod color;
mod position;

use crate::{
    bindings as c_wut,
    rrc::{ResourceGuard, Rrc},
};
use alloc::{ffi::CString, string::String};
pub use color::{Color, ColorParseError};
use core::{ffi, marker::PhantomData, ptr};
use position::Position;
pub use position::{TextAlign, TextPosition};

pub(crate) static OSSCREEN: Rrc<fn(), fn()> = Rrc::new(
    || unsafe {
        use c_wut::ProcUICallbackType::*;

        c_wut::OSScreenInit();
        let _ = _alloc_framebuffer(ptr::null_mut());
        c_wut::ProcUIRegisterCallback(
            PROCUI_CALLBACK_ACQUIRE,
            Some(_alloc_framebuffer),
            ptr::null_mut(),
            100,
        );
        c_wut::ProcUIRegisterCallback(
            PROCUI_CALLBACK_ACQUIRE,
            Some(_dealloc_framebuffer),
            ptr::null_mut(),
            100,
        );
    },
    || unsafe {
        c_wut::OSScreenShutdown();
    },
);

const FRAMEBUFFER_HEAP_TAG: u32 = 0x8E8B30C2;
static mut FRAMEBUFFER_TV: (*mut ffi::c_void, u32) = (ptr::null_mut(), 0);
static mut FRAMEBUFFER_DRC: (*mut ffi::c_void, u32) = (ptr::null_mut(), 0);

unsafe extern "C" fn _alloc_framebuffer(_: *mut ffi::c_void) -> u32 {
    use c_wut::MEMBaseHeapType::*;
    use c_wut::OSScreenID::*;

    let heap = c_wut::MEMGetBaseHeapHandle(MEM_BASE_HEAP_MEM1);
    let _ = c_wut::MEMRecordStateForFrmHeap(heap, FRAMEBUFFER_HEAP_TAG);

    if FRAMEBUFFER_TV.0.is_null() {
        let size = c_wut::OSScreenGetBufferSizeEx(SCREEN_TV);
        FRAMEBUFFER_TV = (c_wut::MEMAllocFromFrmHeapEx(heap, size, 0x100), size);
    }

    if FRAMEBUFFER_DRC.0.is_null() {
        let size = c_wut::OSScreenGetBufferSizeEx(SCREEN_DRC);
        FRAMEBUFFER_DRC = (c_wut::MEMAllocFromFrmHeapEx(heap, size, 0x100), size);
    }

    c_wut::OSScreenSetBufferEx(SCREEN_TV, FRAMEBUFFER_TV.0);
    c_wut::OSScreenSetBufferEx(SCREEN_DRC, FRAMEBUFFER_DRC.0);

    0
}

unsafe extern "C" fn _dealloc_framebuffer(_: *mut ffi::c_void) -> u32 {
    use c_wut::MEMBaseHeapType::*;

    let heap = c_wut::MEMGetBaseHeapHandle(MEM_BASE_HEAP_MEM1);
    let _ = c_wut::MEMFreeByStateToFrmHeap(heap, FRAMEBUFFER_HEAP_TAG);

    FRAMEBUFFER_TV = (ptr::null_mut(), 0);
    FRAMEBUFFER_DRC = (ptr::null_mut(), 0);

    0
}

pub struct TV;
// impl TV {
//     pub const ROWS: u32 = 30;
//     pub const COLS: u32 = 80;
// }
pub struct DRC;

pub trait DisplayType {
    fn id() -> u32;

    fn width() -> u32;

    fn height() -> u32;

    fn resolution() -> (u32, u32);

    fn rows() -> u32;

    fn columns() -> u32;
}

impl DisplayType for TV {
    fn id() -> c_wut::OSScreenID::Type {
        c_wut::OSScreenID::SCREEN_TV
    }

    fn width() -> u32 {
        TV::resolution().0
    }

    fn height() -> u32 {
        TV::resolution().1
    }

    fn resolution() -> (u32, u32) {
        use c_wut::AVMTvResolution::*;
        // it is (theoretically?) possible that no resolution is found?!
        let mut resolution = Type::default();

        unsafe {
            c_wut::AVMGetTVScanMode(&mut resolution);

            if resolution == 0 {
                if c_wut::AVMReadSystemVideoResConfig(&mut resolution) != 0 {
                    panic!("No resolution was returned by the system");
                }
            }
        }
        match resolution {
            AVM_TV_RESOLUTION_576I | AVM_TV_RESOLUTION_576P => (720, 576),
            AVM_TV_RESOLUTION_480I | AVM_TV_RESOLUTION_480I_PAL60 | AVM_TV_RESOLUTION_480P => {
                (720, 480)
            }
            AVM_TV_RESOLUTION_720P | AVM_TV_RESOLUTION_720P_3D | AVM_TV_RESOLUTION_720P_50HZ => {
                (1280, 720)
            }
            AVM_TV_RESOLUTION_1080I
            | AVM_TV_RESOLUTION_1080I_50HZ
            | AVM_TV_RESOLUTION_1080P
            | AVM_TV_RESOLUTION_1080P_50HZ => (1920, 1080),
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
}

impl DisplayType for DRC {
    fn id() -> c_wut::OSScreenID::Type {
        c_wut::OSScreenID::SCREEN_DRC
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
}

pub struct Screen<'a, Display: DisplayType> {
    display: PhantomData<Display>,
    _resource: ResourceGuard<'a>,
}

impl<Display: DisplayType> Screen<'_, Display> {
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
            c_wut::OSScreenEnableEx(Display::id(), 1);
        }
    }

    pub fn disable(&self) {
        unsafe {
            c_wut::OSScreenEnableEx(Display::id(), 0);
        }
    }

    pub fn fill(&self, color: Color) {
        unsafe {
            c_wut::OSScreenClearBufferEx(Display::id(), color.into());
        }
    }

    pub fn update(&self) {
        unsafe {
            c_wut::OSScreenFlipBuffersEx(Display::id());
        }
    }

    // pub fn text(&self, text: &str, position: impl Into<TextPosition>) {
    //     let text = String::from(text);
    //     let position: TextPosition = position.into();

    //     for (line, column, row) in position.format(&text) {
    //         crate::println!("\"{}\" - {} x {}", line, column, row);
    //         unsafe {
    //             c_wut::OSScreenPutFontEx(
    //                 Display::id(),
    //                 column,
    //                 row,
    //                 CString::new(line).unwrap().as_c_str().as_ptr(),
    //             );
    //         }
    //     }
    // }

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

            c_wut::OSScreenPutFontEx(
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
            c_wut::OSScreenPutPixelEx(
                Display::id(),
                x.into(Display::width()),
                y.into(Display::height()),
                color.into(),
            );
        }
    }
}

pub fn tv<'a>() -> Screen<'a, TV> {
    Screen {
        display: PhantomData,
        _resource: OSSCREEN.acquire(),
    }
}

pub fn drc<'a>() -> Screen<'a, DRC> {
    Screen {
        display: PhantomData,
        _resource: OSSCREEN.acquire(),
    }
}
