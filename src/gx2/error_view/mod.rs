//! Error View
//!
//! Render a error message on the screen. Must be used with a Gx2 [RenderContext][super::RenderContext].
//!
//! # Usage:
//!
//! ```
//! use wut::gx2::{
//!     error_view::{Format, Target, Config},
//!     ErrorView, Language, Region, RenderContext, Renderable
//! };
//!
//! let error_view = ErrorView::new(Region::Europe, Language::English).unwrap();
//!
//! let config = Config::builder()
//!     .messgae("Message")
//!     .title("Title")
//!     .format(Format::OkCancel)
//!     .target(Target::Both);
//!
//! let render = error_view.appear(config);
//!
//! let context = RenderContext::new();
//! while wut::process:running() {
//!     render.update();
//!
//!     if render.is_ok() {
//!         println!("{:?}", render.get_value());
//!         break;
//!     }
//!
//!     let context = context.ready();
//!     let context = context.tv();
//!     render.render_tv(&context);
//!     
//!     let context = context.drc();
//!     render.render_drc(&context);
//!
//!     context.finish();
//! }
//! ```

use crate::{
    bindings as c_wut,
    gx2::{
        dialog_utils::{Controller, ControllerInfo, Language, Region},
        Renderable,
    },
    rrc::RrcGuard,
    utils::into_utf16,
};
use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
// use thiserror::Error;

// region: Format

/// Type of Error Dialog
pub enum Format {
    /// Default error message.
    Code,
    /// Custom title and message without and interactivity.
    Message,
    /// Like [Message][Format::Message] but with a button.
    Ok,
    /// Like [Message][Format::Message] but with two buttons.
    OkCancel,
}

impl Into<c_wut::nn_erreula_ErrorType::Type> for Format {
    fn into(self) -> c_wut::nn_erreula_ErrorType::Type {
        use c_wut::nn_erreula_ErrorType as T;
        match self {
            Self::Code => T::Code,
            Self::Message => T::Message,
            Self::Ok => T::Message1Button,
            Self::OkCancel => T::Message2Button,
        }
    }
}

// endregion

// region: Target

pub enum Target {
    Tv,
    Drc,
    Both,
}

impl Into<c_wut::nn_erreula_RenderTarget::Type> for Target {
    fn into(self) -> c_wut::nn_erreula_RenderTarget::Type {
        use c_wut::nn_erreula_RenderTarget as T;
        match self {
            Self::Tv => T::Tv,
            Self::Drc => T::Drc,
            Self::Both => T::Both,
        }
    }
}

// endregion

// region: Config

pub struct Config {
    arg: c_wut::nn_erreula_AppearArg,
    message: Vec<u16>,
    ok_label: Vec<u16>,
    cancel_label: Vec<u16>,
    title: Vec<u16>,
    sound: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            arg: c_wut::nn_erreula_AppearArg::default(),
            message: vec![0],
            ok_label: vec![0],
            cancel_label: vec![0],
            title: vec![0],
            sound: false,
        }
    }
}

impl Config {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn get_raw(&self) -> &c_wut::nn_erreula_AppearArg {
        &self.arg
    }

    pub fn get_raw_mut(&mut self) -> &c_wut::nn_erreula_AppearArg {
        &mut self.arg
    }

    pub fn format(mut self, format: Format) -> Self {
        self.arg.errorArg.errorType = format.into();
        self
    }

    pub fn target(mut self, target: Target) -> Self {
        self.arg.errorArg.renderTarget = target.into();
        self
    }

    pub fn controller(mut self, controller: Controller) -> Self {
        self.arg.errorArg.controllerType = controller.into();
        self
    }

    pub fn error_code(mut self, code: i32) -> Self {
        self.arg.errorArg.errorCode = code;
        self
    }

    pub fn message(mut self, text: &str) -> Self {
        self.message = into_utf16(text);
        self.arg.errorArg.errorMessage = self.message.as_ptr();
        self
    }

    pub fn ok_button(mut self, text: &str) -> Self {
        self.ok_label = into_utf16(text);
        self.arg.errorArg.button1Label = self.ok_label.as_ptr();
        self
    }

    pub fn cancel_button(mut self, text: &str) -> Self {
        self.cancel_label = into_utf16(text);
        self.arg.errorArg.button2Label = self.cancel_label.as_ptr();
        self
    }

    pub fn title(mut self, text: &str) -> Self {
        self.title = into_utf16(text);
        self.arg.errorArg.errorTitle = self.title.as_ptr();
        self
    }

    pub fn sound_effect(mut self, enabled: bool) -> Self {
        self.sound = enabled;
        self
    }
}

// endregion

pub struct ErrorView {
    _fs: c_wut::FSClient,
    _mem: Vec<u8>,
    _fs_guard: RrcGuard,
    _gfx_guard: RrcGuard,
    _vpad_guard: RrcGuard,
}

impl ErrorView {
    pub fn new(region: Region, language: Language) -> Result<Self, ()> {
        let _fs_guard = crate::fs::FS.acquire();
        let _gfx_guard = crate::gx2::GFX.acquire();
        let _vpad_guard = crate::gamepad::VPAD.acquire();

        let mut fs = c_wut::FSClient::default();
        unsafe {
            c_wut::FSAddClient(&mut fs, c_wut::FSErrorFlag::FS_ERROR_FLAG_NONE);
        }

        let mut mem = Vec::with_capacity(unsafe { c_wut::nn_erreula_GetWorkMemorySize() } as usize);

        let create_arg = c_wut::nn_erreula_CreateArg {
            language: language.into(),
            region: region.into(),
            workMemory: mem.as_mut_ptr() as *mut _,
            fsClient: &mut fs,
        };

        if !unsafe { c_wut::nn_erreula_Create(&create_arg) } {
            Err(())
        } else {
            Ok(Self {
                _fs: fs,
                _mem: mem,
                _fs_guard,
                _gfx_guard,
                _vpad_guard,
            })
        }
    }

    pub fn appear(&self, config: Config) -> ErrorViewRenderer {
        unsafe { c_wut::nn_erreula_AppearErrorViewer(config.get_raw()) }

        unsafe {
            c_wut::nn_erreula_PlayAppearSE(config.sound);
        }

        ErrorViewRenderer::new(self)
    }
}

impl Drop for ErrorView {
    fn drop(&mut self) {
        unsafe {
            c_wut::nn_erreula_Destroy();
        }
    }
}

pub struct ErrorViewRenderer<'a> {
    _marker: PhantomData<&'a ErrorView>,
}

impl<'a> ErrorViewRenderer<'a> {
    fn new(_error_viewer: &ErrorView) -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn update(&self) {
        let mut info = ControllerInfo::default();
        info.read_vpad();

        unsafe {
            c_wut::nn_erreula_Calc(&mut info.as_erreula());
        }
    }

    pub fn is_ok(&self) -> bool {
        unsafe { c_wut::nn_erreula_IsDecideSelectLeftButtonError() }
    }

    pub fn is_cancel(&self) -> bool {
        unsafe { c_wut::nn_erreula_IsDecideSelectRightButtonError() }
    }

    pub fn get_value(self) -> (i32, i32) {
        unsafe {
            (
                c_wut::nn_erreula_GetResultType(),
                c_wut::nn_erreula_GetResultCode(),
            )
        }
    }
}

impl<'a> Drop for ErrorViewRenderer<'a> {
    fn drop(&mut self) {
        unsafe {
            c_wut::nn_erreula_DisappearErrorViewer();
        }
    }
}

impl<'a> Renderable for ErrorViewRenderer<'a> {
    fn render_drc(&self, _context: &super::context::Context<super::context::Drc>) {
        unsafe {
            c_wut::nn_erreula_DrawDRC();
        }
    }

    fn render_tv(&self, _context: &super::context::Context<super::context::Tv>) {
        unsafe {
            c_wut::nn_erreula_DrawTV();
        }
    }
}
