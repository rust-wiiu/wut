//! On-screen Keyboard
//!
//! Render a digital keyboard on the screen. Must be used with a Gx2 [RenderContext][crate::gx2::render_context::RenderContext].
//!
//! # Usage:
//!
//! ```
//! use wut::gx2::{
//!     keyboard::Config, Controller, Keyboard, Language, Region, RenderContext,
//!     Renderable,
//! };
//!
//! let keyboard = Keyboard::new(Region::Europe).unwrap();
//!
//! let config = Config::builder()
//!     .language(Language::English)
//!     .controller(Controller::DrcGamepad)
//!     .hint("Hint")
//!     .initial("Initial");
//!
//! let render = keyboard.appear(config).unwrap();
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
    utils::{from_utf16, into_utf16},
};
use alloc::{string::String, vec, vec::Vec};
use core::marker::PhantomData;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyboardError {
    #[error("")]
    CannotCreate,
}

// region: Mode

pub enum Mode {
    Full,
    Numpad,
    Utf8,
    NNID,
}

impl Into<c_wut::nn_swkbd_KeyboardMode::Type> for Mode {
    fn into(self) -> c_wut::nn_swkbd_KeyboardMode::Type {
        use c_wut::nn_swkbd_KeyboardMode as T;
        match self {
            Self::Full => T::Full,
            Self::Numpad => T::Numpad,
            Self::Utf8 => T::Utf8,
            Self::NNID => T::NNID,
        }
    }
}

// endregion

// region: PasswordMode

pub enum PasswordMode {
    Clear,
    Hide,
    Fade,
}

impl Into<c_wut::nn_swkbd_PasswordMode::Type> for PasswordMode {
    fn into(self) -> c_wut::nn_swkbd_PasswordMode::Type {
        use c_wut::nn_swkbd_PasswordMode as T;
        match self {
            Self::Clear => T::Clear,
            Self::Hide => T::Hide,
            Self::Fade => T::Fade,
        }
    }
}

// endregion

// region: Config

#[derive(Debug, Clone)]
pub struct Config {
    arg: c_wut::nn_swkbd_AppearArg,
    ok_string: Vec<u16>,
    initial_text: Vec<u16>,
    hint_text: Vec<u16>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            arg: c_wut::nn_swkbd_AppearArg::default(),
            ok_string: vec![0],
            initial_text: vec![0],
            hint_text: vec![0],
        }
    }
}

impl Config {
    pub fn builder() -> Self {
        Self::default()
    }

    // pub fn build(self) -> Self {
    //     self
    // }

    pub fn get_raw(&self) -> &c_wut::nn_swkbd_AppearArg {
        &self.arg
    }

    pub fn get_raw_mut(&mut self) -> &c_wut::nn_swkbd_AppearArg {
        &mut self.arg
    }

    pub fn language(mut self, language: Language) -> Self {
        self.arg.keyboardArg.configArg.languageType = language.into();
        self
    }

    pub fn controller(mut self, controller: Controller) -> Self {
        self.arg.keyboardArg.configArg.controllerType = controller.into();
        self
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.arg.keyboardArg.configArg.keyboardMode = mode.into();
        self
    }

    pub fn show_suggestions(mut self, enabled: bool) -> Self {
        self.arg.keyboardArg.configArg.showWordSuggestions = enabled;
        self
    }

    pub fn new_line_enabled(mut self, enabled: bool) -> Self {
        self.arg.keyboardArg.configArg.disableNewLine = !enabled;
        self
    }

    pub fn ok_value(mut self, value: &str) -> Self {
        self.ok_string = into_utf16(value);
        self.arg.keyboardArg.configArg.okString = self.ok_string.as_ptr();
        self
    }

    pub fn numpad_char_left(mut self, character: char) -> Self {
        let mut value = [0; 1];
        character.encode_utf16(&mut value);
        self.arg.keyboardArg.configArg.numpadCharLeft = value[0];
        self
    }

    pub fn numpad_char_right(mut self, character: char) -> Self {
        let mut value = [0; 1];
        character.encode_utf16(&mut value);
        self.arg.keyboardArg.configArg.numpadCharRight = value[0];
        self
    }

    pub fn max_length(mut self, n: u8) -> Self {
        self.arg.inputFormArg.maxTextLength = n as i32;
        self
    }

    pub fn highlight_initial_text(mut self, enabled: bool) -> Self {
        self.arg.inputFormArg.higlightInitialText = enabled;
        self
    }

    pub fn show_copy_paste(mut self, enabled: bool) -> Self {
        self.arg.inputFormArg.showCopyPasteButtons = enabled;
        self
    }

    pub fn initial(mut self, value: &str) -> Self {
        self.initial_text = into_utf16(value);
        self.arg.inputFormArg.initialText = self.initial_text.as_ptr();
        self
    }

    pub fn hint(mut self, value: &str) -> Self {
        self.hint_text = into_utf16(value);
        self.arg.inputFormArg.hintText = self.hint_text.as_ptr();
        self
    }

    pub fn password(mut self, mode: PasswordMode) -> Self {
        self.arg.inputFormArg.passwordMode = mode.into();
        self
    }
}

// endregion

pub struct Keyboard {
    _fs: c_wut::FSClient,
    _mem: Vec<u8>,
    _fs_guard: RrcGuard,
    _gfx_guard: RrcGuard,
    _vpad_guard: RrcGuard,
}

impl Keyboard {
    pub fn new(region: Region) -> Result<Self, KeyboardError> {
        let _fs_guard = crate::fs::FS.acquire();
        let _gfx_guard = crate::gx2::GFX.acquire();
        let _vpad_guard = crate::gamepad::VPAD.acquire();

        let mut fs = c_wut::FSClient::default();
        unsafe {
            c_wut::FSAddClient(&mut fs, c_wut::FSErrorFlag::FS_ERROR_FLAG_NONE);
        }

        let mut mem = Vec::with_capacity(unsafe { c_wut::nn_swkbd_GetWorkMemorySize(0) } as usize);

        let create_arg = c_wut::nn_swkbd_CreateArg {
            regionType: region.into(),
            workMemory: mem.as_mut_ptr() as *mut _,
            fsClient: &mut fs,
            ..Default::default()
        };

        if !unsafe { c_wut::nn_swkbd_Create(&create_arg) } {
            Err(KeyboardError::CannotCreate)
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

    pub fn appear(&self, config: Config) -> Result<KeyboardRenderer, ()> {
        let success = unsafe { c_wut::nn_swkbd_AppearInputForm(config.get_raw()) };

        if success {
            unsafe {
                c_wut::nn_swkbd_SetCursorPos(config.initial_text.len() as i32);
            }
            Ok(KeyboardRenderer::new(self))
        } else {
            Err(())
        }
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        unsafe {
            c_wut::nn_swkbd_Destroy();
        }
    }
}

pub struct KeyboardRenderer<'a> {
    _marker: PhantomData<&'a Keyboard>,
}

impl<'a> KeyboardRenderer<'a> {
    fn new(_keyboard: &Keyboard) -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn update(&self) {
        let mut info = ControllerInfo::default();
        info.read_vpad();

        unsafe {
            c_wut::nn_swkbd_Calc(&mut info.as_swkbd());

            if c_wut::nn_swkbd_IsNeedCalcSubThreadFont() {
                c_wut::nn_swkbd_CalcSubThreadFont();
            }

            if c_wut::nn_swkbd_IsNeedCalcSubThreadPredict() {
                c_wut::nn_swkbd_CalcSubThreadPredict();
            }
        }
    }

    pub fn is_ok(&self) -> bool {
        unsafe { c_wut::nn_swkbd_IsDecideOkButton(core::ptr::null_mut()) }
    }

    pub fn is_cancel(&self) -> bool {
        unsafe { c_wut::nn_swkbd_IsDecideCancelButton(core::ptr::null_mut()) }
    }

    pub fn get_value(self) -> String {
        let value = unsafe { c_wut::nn_swkbd_GetInputFormString() };
        from_utf16(value)
    }
}

impl<'a> Drop for KeyboardRenderer<'a> {
    fn drop(&mut self) {
        unsafe {
            c_wut::nn_swkbd_DisappearInputForm();
            c_wut::nn_swkbd_DisappearKeyboard();
        }
    }
}

impl<'a> Renderable for KeyboardRenderer<'a> {
    fn render_tv(
        &self,
        _context: &crate::gx2::render_context::Context<crate::gx2::render_context::Tv>,
    ) {
        unsafe {
            c_wut::nn_swkbd_DrawTV();
        }
    }

    fn render_drc(
        &self,
        _context: &crate::gx2::render_context::Context<crate::gx2::render_context::Drc>,
    ) {
        unsafe {
            c_wut::nn_swkbd_DrawDRC();
        }
    }
}
