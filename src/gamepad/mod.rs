//! Simple gamepad support
//!
//! Purposefully kept simple and not supporting all features of the controllers.
//! If finer controll and access to controller specific features is required, use the more complex "..." (which I maybe add later)

use crate::{
    bindings as c_wut,
    rrc::{Rrc, RrcGuard},
};
use alloc::vec;
use core::{fmt::Debug, panic};
use flagset::{flags, FlagSet};
use thiserror::Error;

pub(crate) static KPAD: Rrc = Rrc::new(
    || unsafe {
        c_wut::KPADInit();
    },
    || unsafe {
        c_wut::KPADShutdown();
    },
);

pub(crate) static VPAD: Rrc = Rrc::new(
    || unsafe {
        c_wut::VPADInit();
    },
    || unsafe {
        c_wut::VPADShutdown();
    },
);

flags! {
    pub enum Button: u32 {
        A,
        B,
        X,
        Y,
        Left,
        Right,
        Up,
        Down,
        L,
        R,
        ZL,
        ZR,
        Plus,
        Minus,
        Home,
        Sync,
        RStick,
        LStick,
        RStickLeft,
        RStickRight,
        RStickUp,
        RStickDown,
        LStickLeft,
        LStickRight,
        LStickUp,
        LStickDown,
        One,
        Two,
        Z,
        C,
    }
}

impl Button {
    fn from_vpad(buttons: u32) -> FlagSet<Button> {
        use c_wut::VPADButtons as B;
        let button_mappings = [
            (B::VPAD_BUTTON_A, Button::A),
            (B::VPAD_BUTTON_B, Button::B),
            (B::VPAD_BUTTON_X, Button::X),
            (B::VPAD_BUTTON_Y, Button::Y),
            (B::VPAD_BUTTON_LEFT, Button::Left),
            (B::VPAD_BUTTON_RIGHT, Button::Right),
            (B::VPAD_BUTTON_UP, Button::Up),
            (B::VPAD_BUTTON_DOWN, Button::Down),
            (B::VPAD_BUTTON_L, Button::L),
            (B::VPAD_BUTTON_R, Button::R),
            (B::VPAD_BUTTON_ZL, Button::ZL),
            (B::VPAD_BUTTON_ZR, Button::ZR),
            (B::VPAD_BUTTON_PLUS, Button::Plus),
            (B::VPAD_BUTTON_MINUS, Button::Minus),
            (B::VPAD_BUTTON_HOME, Button::Home),
            (B::VPAD_BUTTON_SYNC, Button::Sync),
            (B::VPAD_BUTTON_STICK_R, Button::RStick),
            (B::VPAD_BUTTON_STICK_L, Button::LStick),
            (B::VPAD_STICK_R_EMULATION_LEFT, Button::RStickLeft),
            (B::VPAD_STICK_R_EMULATION_RIGHT, Button::RStickRight),
            (B::VPAD_STICK_R_EMULATION_UP, Button::RStickUp),
            (B::VPAD_STICK_R_EMULATION_DOWN, Button::RStickDown),
            (B::VPAD_STICK_L_EMULATION_LEFT, Button::LStickLeft),
            (B::VPAD_STICK_L_EMULATION_RIGHT, Button::LStickRight),
            (B::VPAD_STICK_L_EMULATION_UP, Button::LStickUp),
            (B::VPAD_STICK_L_EMULATION_DOWN, Button::LStickDown),
        ];

        button_mappings
            .iter()
            .fold(Default::default(), |mut b, &(flag, button)| {
                if buttons & flag != 0 {
                    b |= button;
                }
                b
            })
    }

    fn from_kpad(buttons: u32) -> FlagSet<Button> {
        use c_wut::{
            WPADButton as Wpad, WPADClassicButton as Classic, WPADNunchukButton as Nunchuk,
            WPADProButton as Pro,
        };
        let button_mappings = [
            // Buttons
            (Wpad::WPAD_BUTTON_LEFT, Button::Left),
            (Wpad::WPAD_BUTTON_RIGHT, Button::Right),
            (Wpad::WPAD_BUTTON_UP, Button::Up),
            (Wpad::WPAD_BUTTON_DOWN, Button::Down),
            (Wpad::WPAD_BUTTON_A, Button::A),
            (Wpad::WPAD_BUTTON_B, Button::B),
            (Wpad::WPAD_BUTTON_PLUS, Button::Plus),
            (Wpad::WPAD_BUTTON_MINUS, Button::Minus),
            (Wpad::WPAD_BUTTON_HOME, Button::Home),
            (Wpad::WPAD_BUTTON_1, Button::One),
            (Wpad::WPAD_BUTTON_2, Button::Two),
            (Wpad::WPAD_BUTTON_Z, Button::Z),
            (Wpad::WPAD_BUTTON_C, Button::C),
            // Nunchuk
            (
                Nunchuk::WPAD_NUNCHUK_STICK_EMULATION_LEFT,
                Button::LStickLeft,
            ),
            (
                Nunchuk::WPAD_NUNCHUK_STICK_EMULATION_RIGHT,
                Button::LStickRight,
            ),
            (Nunchuk::WPAD_NUNCHUK_STICK_EMULATION_UP, Button::LStickUp),
            (
                Nunchuk::WPAD_NUNCHUK_STICK_EMULATION_DOWN,
                Button::LStickDown,
            ),
            (Nunchuk::WPAD_NUNCHUK_BUTTON_Z, Button::Z),
            (Nunchuk::WPAD_NUNCHUK_BUTTON_C, Button::C),
            // Classic controller
            (Classic::WPAD_CLASSIC_BUTTON_UP, Button::Up),
            (Classic::WPAD_CLASSIC_BUTTON_DOWN, Button::Down),
            (Classic::WPAD_CLASSIC_BUTTON_LEFT, Button::Left),
            (Classic::WPAD_CLASSIC_BUTTON_RIGHT, Button::Right),
            (Classic::WPAD_CLASSIC_BUTTON_A, Button::A),
            (Classic::WPAD_CLASSIC_BUTTON_B, Button::B),
            (Classic::WPAD_CLASSIC_BUTTON_X, Button::X),
            (Classic::WPAD_CLASSIC_BUTTON_Y, Button::Y),
            (Classic::WPAD_CLASSIC_BUTTON_L, Button::L),
            (Classic::WPAD_CLASSIC_BUTTON_R, Button::R),
            (Classic::WPAD_CLASSIC_BUTTON_ZL, Button::Left),
            (Classic::WPAD_CLASSIC_BUTTON_ZR, Button::Right),
            (Classic::WPAD_CLASSIC_BUTTON_PLUS, Button::Plus),
            (Classic::WPAD_CLASSIC_BUTTON_MINUS, Button::Minus),
            (Classic::WPAD_CLASSIC_BUTTON_HOME, Button::Home),
            (
                Classic::WPAD_CLASSIC_STICK_L_EMULATION_LEFT,
                Button::LStickLeft,
            ),
            (
                Classic::WPAD_CLASSIC_STICK_L_EMULATION_RIGHT,
                Button::LStickRight,
            ),
            (Classic::WPAD_CLASSIC_STICK_L_EMULATION_UP, Button::LStickUp),
            (
                Classic::WPAD_CLASSIC_STICK_L_EMULATION_DOWN,
                Button::LStickDown,
            ),
            (
                Classic::WPAD_CLASSIC_STICK_R_EMULATION_LEFT,
                Button::RStickLeft,
            ),
            (
                Classic::WPAD_CLASSIC_STICK_R_EMULATION_RIGHT,
                Button::RStickRight,
            ),
            (Classic::WPAD_CLASSIC_STICK_R_EMULATION_UP, Button::RStickUp),
            (
                Classic::WPAD_CLASSIC_STICK_R_EMULATION_DOWN,
                Button::RStickDown,
            ),
            // Pro controller
            (Pro::WPAD_PRO_BUTTON_UP, Button::Up),
            (Pro::WPAD_PRO_BUTTON_DOWN, Button::Down),
            (Pro::WPAD_PRO_BUTTON_LEFT, Button::Left),
            (Pro::WPAD_PRO_BUTTON_RIGHT, Button::Right),
            (Pro::WPAD_PRO_BUTTON_A, Button::A),
            (Pro::WPAD_PRO_BUTTON_B, Button::B),
            (Pro::WPAD_PRO_BUTTON_X, Button::X),
            (Pro::WPAD_PRO_BUTTON_Y, Button::Y),
            (Pro::WPAD_PRO_TRIGGER_L, Button::L),
            (Pro::WPAD_PRO_TRIGGER_R, Button::R),
            (Pro::WPAD_PRO_TRIGGER_ZL, Button::ZL),
            (Pro::WPAD_PRO_TRIGGER_ZR, Button::ZR),
            (Pro::WPAD_PRO_BUTTON_PLUS, Button::Plus),
            (Pro::WPAD_PRO_BUTTON_MINUS, Button::Minus),
            (Pro::WPAD_PRO_BUTTON_HOME, Button::Home),
            (Pro::WPAD_PRO_BUTTON_STICK_L, Button::LStick),
            (Pro::WPAD_PRO_BUTTON_STICK_R, Button::RStick),
            (Pro::WPAD_PRO_STICK_L_EMULATION_LEFT, Button::LStickLeft),
            (Pro::WPAD_PRO_STICK_L_EMULATION_RIGHT, Button::LStickRight),
            (Pro::WPAD_PRO_STICK_L_EMULATION_UP, Button::LStickUp),
            (Pro::WPAD_PRO_STICK_L_EMULATION_DOWN, Button::LStickDown),
            (Pro::WPAD_PRO_STICK_R_EMULATION_LEFT, Button::RStickLeft),
            (Pro::WPAD_PRO_STICK_R_EMULATION_RIGHT, Button::RStickRight),
            (Pro::WPAD_PRO_STICK_R_EMULATION_UP, Button::RStickUp),
            (Pro::WPAD_PRO_STICK_R_EMULATION_DOWN, Button::RStickDown),
        ];

        button_mappings
            .iter()
            .fold(Default::default(), |mut b, &(flag, button)| {
                if buttons & flag != 0 {
                    b |= button;
                }
                b
            })
    }

    fn from_nunchuk(buttons: u32) -> FlagSet<Button> {
        use c_wut::WPADNunchukButton as Nunchuk;
        let button_mappings = [
            (
                Nunchuk::WPAD_NUNCHUK_STICK_EMULATION_LEFT,
                Button::LStickLeft,
            ),
            (
                Nunchuk::WPAD_NUNCHUK_STICK_EMULATION_RIGHT,
                Button::LStickRight,
            ),
            (Nunchuk::WPAD_NUNCHUK_STICK_EMULATION_UP, Button::LStickUp),
            (
                Nunchuk::WPAD_NUNCHUK_STICK_EMULATION_DOWN,
                Button::LStickDown,
            ),
            (Nunchuk::WPAD_NUNCHUK_BUTTON_Z, Button::Z),
            (Nunchuk::WPAD_NUNCHUK_BUTTON_C, Button::C),
        ];

        button_mappings
            .iter()
            .fold(Default::default(), |mut b, &(flag, button)| {
                if buttons & flag != 0 {
                    b |= button;
                }
                b
            })
    }

    fn from_classic(buttons: u32) -> FlagSet<Button> {
        use c_wut::WPADClassicButton as Classic;
        let button_mappings = [
            (Classic::WPAD_CLASSIC_BUTTON_UP, Button::Up),
            (Classic::WPAD_CLASSIC_BUTTON_DOWN, Button::Down),
            (Classic::WPAD_CLASSIC_BUTTON_LEFT, Button::Left),
            (Classic::WPAD_CLASSIC_BUTTON_RIGHT, Button::Right),
            (Classic::WPAD_CLASSIC_BUTTON_A, Button::A),
            (Classic::WPAD_CLASSIC_BUTTON_B, Button::B),
            (Classic::WPAD_CLASSIC_BUTTON_X, Button::X),
            (Classic::WPAD_CLASSIC_BUTTON_Y, Button::Y),
            (Classic::WPAD_CLASSIC_BUTTON_L, Button::L),
            (Classic::WPAD_CLASSIC_BUTTON_R, Button::R),
            (Classic::WPAD_CLASSIC_BUTTON_ZL, Button::Left),
            (Classic::WPAD_CLASSIC_BUTTON_ZR, Button::Right),
            (Classic::WPAD_CLASSIC_BUTTON_PLUS, Button::Plus),
            (Classic::WPAD_CLASSIC_BUTTON_MINUS, Button::Minus),
            (Classic::WPAD_CLASSIC_BUTTON_HOME, Button::Home),
            (
                Classic::WPAD_CLASSIC_STICK_L_EMULATION_LEFT,
                Button::LStickLeft,
            ),
            (
                Classic::WPAD_CLASSIC_STICK_L_EMULATION_RIGHT,
                Button::LStickRight,
            ),
            (Classic::WPAD_CLASSIC_STICK_L_EMULATION_UP, Button::LStickUp),
            (
                Classic::WPAD_CLASSIC_STICK_L_EMULATION_DOWN,
                Button::LStickDown,
            ),
            (
                Classic::WPAD_CLASSIC_STICK_R_EMULATION_LEFT,
                Button::RStickLeft,
            ),
            (
                Classic::WPAD_CLASSIC_STICK_R_EMULATION_RIGHT,
                Button::RStickRight,
            ),
            (Classic::WPAD_CLASSIC_STICK_R_EMULATION_UP, Button::RStickUp),
            (
                Classic::WPAD_CLASSIC_STICK_R_EMULATION_DOWN,
                Button::RStickDown,
            ),
        ];

        button_mappings
            .iter()
            .fold(Default::default(), |mut b, &(flag, button)| {
                if buttons & flag != 0 {
                    b |= button;
                }
                b
            })
    }

    fn from_pro(buttons: u32) -> FlagSet<Button> {
        use c_wut::WPADProButton as Pro;
        let button_mappings = [
            (Pro::WPAD_PRO_BUTTON_UP, Button::Up),
            (Pro::WPAD_PRO_BUTTON_DOWN, Button::Down),
            (Pro::WPAD_PRO_BUTTON_LEFT, Button::Left),
            (Pro::WPAD_PRO_BUTTON_RIGHT, Button::Right),
            (Pro::WPAD_PRO_BUTTON_A, Button::A),
            (Pro::WPAD_PRO_BUTTON_B, Button::B),
            (Pro::WPAD_PRO_BUTTON_X, Button::X),
            (Pro::WPAD_PRO_BUTTON_Y, Button::Y),
            (Pro::WPAD_PRO_TRIGGER_L, Button::L),
            (Pro::WPAD_PRO_TRIGGER_R, Button::R),
            (Pro::WPAD_PRO_TRIGGER_ZL, Button::ZL),
            (Pro::WPAD_PRO_TRIGGER_ZR, Button::ZR),
            (Pro::WPAD_PRO_BUTTON_PLUS, Button::Plus),
            (Pro::WPAD_PRO_BUTTON_MINUS, Button::Minus),
            (Pro::WPAD_PRO_BUTTON_HOME, Button::Home),
            (Pro::WPAD_PRO_BUTTON_STICK_L, Button::LStick),
            (Pro::WPAD_PRO_BUTTON_STICK_R, Button::RStick),
            (Pro::WPAD_PRO_STICK_L_EMULATION_LEFT, Button::LStickLeft),
            (Pro::WPAD_PRO_STICK_L_EMULATION_RIGHT, Button::LStickRight),
            (Pro::WPAD_PRO_STICK_L_EMULATION_UP, Button::LStickUp),
            (Pro::WPAD_PRO_STICK_L_EMULATION_DOWN, Button::LStickDown),
            (Pro::WPAD_PRO_STICK_R_EMULATION_LEFT, Button::RStickLeft),
            (Pro::WPAD_PRO_STICK_R_EMULATION_RIGHT, Button::RStickRight),
            (Pro::WPAD_PRO_STICK_R_EMULATION_UP, Button::RStickUp),
            (Pro::WPAD_PRO_STICK_R_EMULATION_DOWN, Button::RStickDown),
        ];

        button_mappings
            .iter()
            .fold(Default::default(), |mut b, &(flag, button)| {
                if buttons & flag != 0 {
                    b |= button;
                }
                b
            })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Joystick {
    pub x: f32,
    pub y: f32,
}

impl Into<Joystick> for c_wut::VPADVec2D {
    fn into(self) -> Joystick {
        Joystick {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<Joystick> for c_wut::KPADVec2D {
    fn into(self) -> Joystick {
        Joystick {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Port {
    DRC,
    Port0,
    Port1,
    Port2,
    Port3,
    Port4,
    Port5,
    Port6,
}

impl Port {
    pub fn iter() -> core::slice::Iter<'static, Port> {
        static P: [Port; 8] = [
            Port::DRC,
            Port::Port0,
            Port::Port1,
            Port::Port2,
            Port::Port3,
            Port::Port4,
            Port::Port5,
            Port::Port6,
        ];
        P.iter()
    }
}

impl Into<u32> for Port {
    fn into(self) -> u32 {
        match self {
            Port::DRC => 0,
            Port::Port0 => 0,
            Port::Port1 => 1,
            Port::Port2 => 2,
            Port::Port3 => 3,
            Port::Port4 => 4,
            Port::Port5 => 5,
            Port::Port6 => 6,
        }
    }
}

pub struct Gamepad {
    pub port: Port,
    _resource: RrcGuard,
}

impl Debug for Gamepad {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Gamepad {{ port: {:?} }}", self.port)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamepadState {
    pub hold: FlagSet<Button>,
    pub trigger: FlagSet<Button>,
    pub release: FlagSet<Button>,
    pub left_stick: Option<Joystick>,
    pub right_stick: Option<Joystick>,
}

impl GamepadState {
    pub const fn empty() -> Self {
        Self {
            hold: unsafe { FlagSet::new_unchecked(0) },
            trigger: unsafe { FlagSet::new_unchecked(0) },
            release: unsafe { FlagSet::new_unchecked(0) },
            left_stick: None,
            right_stick: None,
        }
    }
}

impl From<c_wut::VPADStatus> for GamepadState {
    fn from(value: c_wut::VPADStatus) -> Self {
        GamepadState {
            hold: Button::from_vpad(value.hold),
            trigger: Button::from_vpad(value.trigger),
            release: Button::from_vpad(value.release),
            left_stick: Some(value.leftStick.into()),
            right_stick: Some(value.rightStick.into()),
        }
    }
}

impl From<c_wut::KPADStatus> for GamepadState {
    fn from(value: c_wut::KPADStatus) -> Self {
        use c_wut::WPADExtensionType as Ext;

        let mut s = GamepadState {
            hold: Button::from_kpad(value.hold),
            trigger: Button::from_kpad(value.trigger),
            release: Button::from_kpad(value.release),
            left_stick: None,
            right_stick: None,
        };

        match value.extensionType as u32 {
            Ext::WPAD_EXT_NUNCHUK => unsafe {
                s.hold |= Button::from_nunchuk(value.__bindgen_anon_1.nunchuk.hold);
                s.trigger |= Button::from_nunchuk(value.__bindgen_anon_1.nunchuk.trigger);
                s.release |= Button::from_nunchuk(value.__bindgen_anon_1.nunchuk.release);
                s.left_stick = Some(value.__bindgen_anon_1.nunchuk.stick.into())
            },
            Ext::WPAD_EXT_CLASSIC => unsafe {
                s.hold |= Button::from_classic(value.__bindgen_anon_1.classic.hold);
                s.trigger |= Button::from_classic(value.__bindgen_anon_1.classic.trigger);
                s.release |= Button::from_classic(value.__bindgen_anon_1.classic.release);
                s.left_stick = Some(value.__bindgen_anon_1.classic.leftStick.into());
                s.right_stick = Some(value.__bindgen_anon_1.classic.rightStick.into());
            },
            Ext::WPAD_EXT_PRO_CONTROLLER => unsafe {
                s.hold |= Button::from_pro(value.__bindgen_anon_1.pro.hold);
                s.trigger |= Button::from_pro(value.__bindgen_anon_1.pro.trigger);
                s.release |= Button::from_pro(value.__bindgen_anon_1.pro.release);
                s.left_stick = Some(value.__bindgen_anon_1.pro.leftStick.into());
                s.right_stick = Some(value.__bindgen_anon_1.pro.rightStick.into());
            },
            _ => (),
        }
        s
    }
}

#[derive(Debug, Error)]
pub enum GamepadError {
    #[error("There was no sample new data available to write.")]
    NoSamples,
    #[error("The requested controller or channel was invalid.")]
    InvalidController,
    #[error("VPAD channel is busy, perhaps being accessed by another thread.")]
    Busy,
    #[error("VPAD is uninitialized, need to call VPADInit().")]
    Uninitialized,
}

impl From<i32> for GamepadError {
    #[allow(unreachable_patterns)] // to make it really clear what is matched.
    fn from(value: i32) -> Self {
        use c_wut::KPADError as KPAD;
        use c_wut::VPADReadError as VPAD;
        match value {
            VPAD::VPAD_READ_NO_SAMPLES | KPAD::KPAD_ERROR_NO_SAMPLES => Self::NoSamples,
            VPAD::VPAD_READ_INVALID_CONTROLLER | KPAD::KPAD_ERROR_INVALID_CONTROLLER => {
                Self::InvalidController
            }
            VPAD::VPAD_READ_BUSY | KPAD::KPAD_ERROR_BUSY => Self::Busy,
            VPAD::VPAD_READ_UNINITIALIZED
            | KPAD::KPAD_ERROR_WPAD_UNINIT
            | KPAD::KPAD_ERROR_UNINITIALIZED => Self::Uninitialized,
            _ => panic!("Unknown error code: {}", value),
        }
    }
}

impl Gamepad {
    pub fn new(port: Port) -> Self {
        match port {
            Port::DRC => Self {
                port: port,
                _resource: VPAD.acquire(),
            },
            _ => Self {
                port: port,
                _resource: KPAD.acquire(),
            },
        }
    }

    pub fn poll(&self) -> Result<GamepadState, GamepadError> {
        match self.port {
            Port::DRC => {
                use c_wut::VPADReadError as E;

                let mut status = c_wut::VPADStatus::default();
                let mut error = E::VPAD_READ_SUCCESS;

                if unsafe { c_wut::VPADRead(self.port.into(), &mut status, 1, &mut error) } == 0
                    && error != E::VPAD_READ_SUCCESS
                {
                    Err(GamepadError::from(error))
                } else {
                    Ok(GamepadState::from(status))
                }
            }
            _ => {
                use c_wut::KPADError as E;

                let mut status = c_wut::KPADStatus::default();
                let mut error = E::KPAD_ERROR_OK;

                if unsafe { c_wut::KPADReadEx(self.port.into(), &mut status, 1, &mut error) } == 0
                    && error != E::KPAD_ERROR_OK
                {
                    Err(GamepadError::from(error))
                } else {
                    Ok(GamepadState::from(status))
                }
            }
        }
    }
}

pub fn gamepads() -> alloc::vec::IntoIter<Gamepad> {
    let mut pads: vec::Vec<Gamepad> = vec![];
    for port in Port::iter() {
        let pad = Gamepad::new(*port);
        if pad.poll().is_ok() {
            pads.push(pad);
        }
    }
    pads.into_iter()
}

pub fn max_gamepads() -> u8 {
    let _kpad = KPAD.acquire();
    unsafe { c_wut::KPADGetGameMaxControllers() }
        .try_into()
        .expect("Max # of gamecontroller can NEVER exceed `u8::MAX`")
}
