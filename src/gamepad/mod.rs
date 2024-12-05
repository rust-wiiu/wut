//! Simple gamepad support
//!
//! Purposefully kept simple and not supporting all features of the controllers.
//! If finer controll and access to controller specific features is required, use the more complex "..." (which I maybe add later)

use crate::bindings as c_wut;
use crate::rrc::{ResourceGuard, Rrc};
use core::fmt::Debug;
use core::panic;
use flagset::{flags, FlagSet};
use thiserror::Error;

pub(crate) static KPAD: Rrc<fn(), fn()> = Rrc::new(
    || unsafe {
        c_wut::KPADInit();
    },
    || unsafe {
        c_wut::KPADShutdown();
    },
);

pub(crate) static VPAD: Rrc<fn(), fn()> = Rrc::new(
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
        let button_mappings = [
            (c_wut::VPAD_BUTTON_A, Button::A),
            (c_wut::VPAD_BUTTON_B, Button::B),
            (c_wut::VPAD_BUTTON_X, Button::X),
            (c_wut::VPAD_BUTTON_Y, Button::Y),
            (c_wut::VPAD_BUTTON_LEFT, Button::Left),
            (c_wut::VPAD_BUTTON_RIGHT, Button::Right),
            (c_wut::VPAD_BUTTON_UP, Button::Up),
            (c_wut::VPAD_BUTTON_DOWN, Button::Down),
            (c_wut::VPAD_BUTTON_L, Button::L),
            (c_wut::VPAD_BUTTON_R, Button::R),
            (c_wut::VPAD_BUTTON_ZL, Button::ZL),
            (c_wut::VPAD_BUTTON_ZR, Button::ZR),
            (c_wut::VPAD_BUTTON_PLUS, Button::Plus),
            (c_wut::VPAD_BUTTON_MINUS, Button::Minus),
            (c_wut::VPAD_BUTTON_HOME, Button::Home),
            (c_wut::VPAD_BUTTON_SYNC, Button::Sync),
            (c_wut::VPAD_BUTTON_STICK_R, Button::RStick),
            (c_wut::VPAD_BUTTON_STICK_L, Button::LStick),
            (c_wut::VPAD_STICK_R_EMULATION_LEFT, Button::RStickLeft),
            (c_wut::VPAD_STICK_R_EMULATION_RIGHT, Button::RStickRight),
            (c_wut::VPAD_STICK_R_EMULATION_UP, Button::RStickUp),
            (c_wut::VPAD_STICK_R_EMULATION_DOWN, Button::RStickDown),
            (c_wut::VPAD_STICK_L_EMULATION_LEFT, Button::LStickLeft),
            (c_wut::VPAD_STICK_L_EMULATION_RIGHT, Button::LStickRight),
            (c_wut::VPAD_STICK_L_EMULATION_UP, Button::LStickUp),
            (c_wut::VPAD_STICK_L_EMULATION_DOWN, Button::LStickDown),
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
        let button_mappings = [
            // Buttons
            (c_wut::WPAD_BUTTON_LEFT, Button::Left),
            (c_wut::WPAD_BUTTON_RIGHT, Button::Right),
            (c_wut::WPAD_BUTTON_UP, Button::Up),
            (c_wut::WPAD_BUTTON_DOWN, Button::Down),
            (c_wut::WPAD_BUTTON_A, Button::A),
            (c_wut::WPAD_BUTTON_B, Button::B),
            (c_wut::WPAD_BUTTON_PLUS, Button::Plus),
            (c_wut::WPAD_BUTTON_MINUS, Button::Minus),
            (c_wut::WPAD_BUTTON_HOME, Button::Home),
            (c_wut::WPAD_BUTTON_1, Button::One),
            (c_wut::WPAD_BUTTON_2, Button::Two),
            (c_wut::WPAD_BUTTON_Z, Button::Z),
            (c_wut::WPAD_BUTTON_C, Button::C),
            // Nunchuck
            (c_wut::WPAD_NUNCHUK_STICK_EMULATION_LEFT, Button::LStickLeft),
            (
                c_wut::WPAD_NUNCHUK_STICK_EMULATION_RIGHT,
                Button::LStickRight,
            ),
            (c_wut::WPAD_NUNCHUK_STICK_EMULATION_UP, Button::LStickUp),
            (c_wut::WPAD_NUNCHUK_STICK_EMULATION_DOWN, Button::LStickDown),
            (c_wut::WPAD_NUNCHUK_BUTTON_Z, Button::Z),
            (c_wut::WPAD_NUNCHUK_BUTTON_C, Button::C),
            // Classic controller
            (c_wut::WPAD_CLASSIC_BUTTON_UP, Button::Up),
            (c_wut::WPAD_CLASSIC_BUTTON_DOWN, Button::Down),
            (c_wut::WPAD_CLASSIC_BUTTON_LEFT, Button::Left),
            (c_wut::WPAD_CLASSIC_BUTTON_RIGHT, Button::Right),
            (c_wut::WPAD_CLASSIC_BUTTON_A, Button::A),
            (c_wut::WPAD_CLASSIC_BUTTON_B, Button::B),
            (c_wut::WPAD_CLASSIC_BUTTON_X, Button::X),
            (c_wut::WPAD_CLASSIC_BUTTON_Y, Button::Y),
            (c_wut::WPAD_CLASSIC_BUTTON_L, Button::L),
            (c_wut::WPAD_CLASSIC_BUTTON_R, Button::R),
            (c_wut::WPAD_CLASSIC_BUTTON_ZL, Button::Left),
            (c_wut::WPAD_CLASSIC_BUTTON_ZR, Button::Right),
            (c_wut::WPAD_CLASSIC_BUTTON_PLUS, Button::Plus),
            (c_wut::WPAD_CLASSIC_BUTTON_MINUS, Button::Minus),
            (c_wut::WPAD_CLASSIC_BUTTON_HOME, Button::Home),
            (
                c_wut::WPAD_CLASSIC_STICK_L_EMULATION_LEFT,
                Button::LStickLeft,
            ),
            (
                c_wut::WPAD_CLASSIC_STICK_L_EMULATION_RIGHT,
                Button::LStickRight,
            ),
            (c_wut::WPAD_CLASSIC_STICK_L_EMULATION_UP, Button::LStickUp),
            (
                c_wut::WPAD_CLASSIC_STICK_L_EMULATION_DOWN,
                Button::LStickDown,
            ),
            (
                c_wut::WPAD_CLASSIC_STICK_R_EMULATION_LEFT,
                Button::RStickLeft,
            ),
            (
                c_wut::WPAD_CLASSIC_STICK_R_EMULATION_RIGHT,
                Button::RStickRight,
            ),
            (c_wut::WPAD_CLASSIC_STICK_R_EMULATION_UP, Button::RStickUp),
            (
                c_wut::WPAD_CLASSIC_STICK_R_EMULATION_DOWN,
                Button::RStickDown,
            ),
            // Pro controller
            (c_wut::WPAD_PRO_BUTTON_UP, Button::Up),
            (c_wut::WPAD_PRO_BUTTON_DOWN, Button::Down),
            (c_wut::WPAD_PRO_BUTTON_LEFT, Button::Left),
            (c_wut::WPAD_PRO_BUTTON_RIGHT, Button::Right),
            (c_wut::WPAD_PRO_BUTTON_A, Button::A),
            (c_wut::WPAD_PRO_BUTTON_B, Button::B),
            (c_wut::WPAD_PRO_BUTTON_X, Button::X),
            (c_wut::WPAD_PRO_BUTTON_Y, Button::Y),
            (c_wut::WPAD_PRO_TRIGGER_L, Button::L),
            (c_wut::WPAD_PRO_TRIGGER_R, Button::R),
            (c_wut::WPAD_PRO_TRIGGER_ZL, Button::ZL),
            (c_wut::WPAD_PRO_TRIGGER_ZR, Button::ZR),
            (c_wut::WPAD_PRO_BUTTON_PLUS, Button::Plus),
            (c_wut::WPAD_PRO_BUTTON_MINUS, Button::Minus),
            (c_wut::WPAD_PRO_BUTTON_HOME, Button::Home),
            (c_wut::WPAD_PRO_BUTTON_STICK_L, Button::LStick),
            (c_wut::WPAD_PRO_BUTTON_STICK_R, Button::RStick),
            (c_wut::WPAD_PRO_STICK_L_EMULATION_LEFT, Button::LStickLeft),
            (c_wut::WPAD_PRO_STICK_L_EMULATION_RIGHT, Button::LStickRight),
            (c_wut::WPAD_PRO_STICK_L_EMULATION_UP, Button::LStickUp),
            (c_wut::WPAD_PRO_STICK_L_EMULATION_DOWN, Button::LStickDown),
            (c_wut::WPAD_PRO_STICK_R_EMULATION_LEFT, Button::RStickLeft),
            (c_wut::WPAD_PRO_STICK_R_EMULATION_RIGHT, Button::RStickRight),
            (c_wut::WPAD_PRO_STICK_R_EMULATION_UP, Button::RStickUp),
            (c_wut::WPAD_PRO_STICK_R_EMULATION_DOWN, Button::RStickDown),
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

    fn from_nunchuck(buttons: u32) -> FlagSet<Button> {
        let button_mappings = [
            (c_wut::WPAD_NUNCHUK_STICK_EMULATION_LEFT, Button::LStickLeft),
            (
                c_wut::WPAD_NUNCHUK_STICK_EMULATION_RIGHT,
                Button::LStickRight,
            ),
            (c_wut::WPAD_NUNCHUK_STICK_EMULATION_UP, Button::LStickUp),
            (c_wut::WPAD_NUNCHUK_STICK_EMULATION_DOWN, Button::LStickDown),
            (c_wut::WPAD_NUNCHUK_BUTTON_Z, Button::Z),
            (c_wut::WPAD_NUNCHUK_BUTTON_C, Button::C),
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
        let button_mappings = [
            (c_wut::WPAD_CLASSIC_BUTTON_UP, Button::Up),
            (c_wut::WPAD_CLASSIC_BUTTON_DOWN, Button::Down),
            (c_wut::WPAD_CLASSIC_BUTTON_LEFT, Button::Left),
            (c_wut::WPAD_CLASSIC_BUTTON_RIGHT, Button::Right),
            (c_wut::WPAD_CLASSIC_BUTTON_A, Button::A),
            (c_wut::WPAD_CLASSIC_BUTTON_B, Button::B),
            (c_wut::WPAD_CLASSIC_BUTTON_X, Button::X),
            (c_wut::WPAD_CLASSIC_BUTTON_Y, Button::Y),
            (c_wut::WPAD_CLASSIC_BUTTON_L, Button::L),
            (c_wut::WPAD_CLASSIC_BUTTON_R, Button::R),
            (c_wut::WPAD_CLASSIC_BUTTON_ZL, Button::Left),
            (c_wut::WPAD_CLASSIC_BUTTON_ZR, Button::Right),
            (c_wut::WPAD_CLASSIC_BUTTON_PLUS, Button::Plus),
            (c_wut::WPAD_CLASSIC_BUTTON_MINUS, Button::Minus),
            (c_wut::WPAD_CLASSIC_BUTTON_HOME, Button::Home),
            (
                c_wut::WPAD_CLASSIC_STICK_L_EMULATION_LEFT,
                Button::LStickLeft,
            ),
            (
                c_wut::WPAD_CLASSIC_STICK_L_EMULATION_RIGHT,
                Button::LStickRight,
            ),
            (c_wut::WPAD_CLASSIC_STICK_L_EMULATION_UP, Button::LStickUp),
            (
                c_wut::WPAD_CLASSIC_STICK_L_EMULATION_DOWN,
                Button::LStickDown,
            ),
            (
                c_wut::WPAD_CLASSIC_STICK_R_EMULATION_LEFT,
                Button::RStickLeft,
            ),
            (
                c_wut::WPAD_CLASSIC_STICK_R_EMULATION_RIGHT,
                Button::RStickRight,
            ),
            (c_wut::WPAD_CLASSIC_STICK_R_EMULATION_UP, Button::RStickUp),
            (
                c_wut::WPAD_CLASSIC_STICK_R_EMULATION_DOWN,
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
        let button_mappings = [
            (c_wut::WPAD_PRO_BUTTON_UP, Button::Up),
            (c_wut::WPAD_PRO_BUTTON_DOWN, Button::Down),
            (c_wut::WPAD_PRO_BUTTON_LEFT, Button::Left),
            (c_wut::WPAD_PRO_BUTTON_RIGHT, Button::Right),
            (c_wut::WPAD_PRO_BUTTON_A, Button::A),
            (c_wut::WPAD_PRO_BUTTON_B, Button::B),
            (c_wut::WPAD_PRO_BUTTON_X, Button::X),
            (c_wut::WPAD_PRO_BUTTON_Y, Button::Y),
            (c_wut::WPAD_PRO_TRIGGER_L, Button::L),
            (c_wut::WPAD_PRO_TRIGGER_R, Button::R),
            (c_wut::WPAD_PRO_TRIGGER_ZL, Button::ZL),
            (c_wut::WPAD_PRO_TRIGGER_ZR, Button::ZR),
            (c_wut::WPAD_PRO_BUTTON_PLUS, Button::Plus),
            (c_wut::WPAD_PRO_BUTTON_MINUS, Button::Minus),
            (c_wut::WPAD_PRO_BUTTON_HOME, Button::Home),
            (c_wut::WPAD_PRO_BUTTON_STICK_L, Button::LStick),
            (c_wut::WPAD_PRO_BUTTON_STICK_R, Button::RStick),
            (c_wut::WPAD_PRO_STICK_L_EMULATION_LEFT, Button::LStickLeft),
            (c_wut::WPAD_PRO_STICK_L_EMULATION_RIGHT, Button::LStickRight),
            (c_wut::WPAD_PRO_STICK_L_EMULATION_UP, Button::LStickUp),
            (c_wut::WPAD_PRO_STICK_L_EMULATION_DOWN, Button::LStickDown),
            (c_wut::WPAD_PRO_STICK_R_EMULATION_LEFT, Button::RStickLeft),
            (c_wut::WPAD_PRO_STICK_R_EMULATION_RIGHT, Button::RStickRight),
            (c_wut::WPAD_PRO_STICK_R_EMULATION_UP, Button::RStickUp),
            (c_wut::WPAD_PRO_STICK_R_EMULATION_DOWN, Button::RStickDown),
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

#[derive(Debug, Clone, Copy, Default)]
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
    _resource: ResourceGuard<'static>,
}

impl Debug for Gamepad {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Gamepad {{ port: {:?} }}", self.port)
    }
}

#[derive(Debug)]
pub struct GamepadState {
    pub hold: FlagSet<Button>,
    pub trigger: FlagSet<Button>,
    pub release: FlagSet<Button>,
    pub left_stick: Option<Joystick>,
    pub right_stick: Option<Joystick>,
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
        match value {
            c_wut::VPAD_READ_NO_SAMPLES | c_wut::KPAD_ERROR_NO_SAMPLES => Self::NoSamples,
            c_wut::VPAD_READ_INVALID_CONTROLLER | c_wut::KPAD_ERROR_INVALID_CONTROLLER => {
                Self::InvalidController
            }
            c_wut::VPAD_READ_BUSY | c_wut::KPAD_ERROR_BUSY => Self::Busy,
            c_wut::VPAD_READ_UNINITIALIZED
            | c_wut::KPAD_ERROR_WPAD_UNINIT
            | c_wut::KPAD_ERROR_UNINITIALIZED => Self::Uninitialized,
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
                let mut status = c_wut::VPADStatus::default();
                let mut error = c_wut::VPAD_READ_SUCCESS;

                if unsafe { c_wut::VPADRead(self.port.into(), &mut status, 1, &mut error) } == 0
                    && error != c_wut::VPAD_READ_SUCCESS
                {
                    Err(GamepadError::from(error))
                } else {
                    Ok(GamepadState {
                        hold: Button::from_vpad(status.hold),
                        trigger: Button::from_vpad(status.trigger),
                        release: Button::from_vpad(status.release),
                        left_stick: Some(status.leftStick.into()),
                        right_stick: Some(status.rightStick.into()),
                    })
                }
            }
            _ => {
                let mut status = c_wut::KPADStatus::default();
                let mut error = c_wut::KPAD_ERROR_OK;

                if unsafe { c_wut::KPADReadEx(self.port.into(), &mut status, 1, &mut error) } == 0
                    && error != c_wut::KPAD_ERROR_OK
                {
                    Err(GamepadError::from(error))
                } else {
                    let mut s = GamepadState {
                        hold: Button::from_kpad(status.hold),
                        trigger: Button::from_kpad(status.trigger),
                        release: Button::from_kpad(status.release),
                        left_stick: None,
                        right_stick: None,
                    };

                    match status.extensionType as u32 {
                        c_wut::WPAD_EXT_NUNCHUK => unsafe {
                            s.hold |= Button::from_nunchuck(status.__bindgen_anon_1.nunchuk.hold);
                            s.trigger |=
                                Button::from_nunchuck(status.__bindgen_anon_1.nunchuk.trigger);
                            s.release |=
                                Button::from_nunchuck(status.__bindgen_anon_1.nunchuk.release);
                            s.left_stick = Some(status.__bindgen_anon_1.nunchuk.stick.into())
                        },
                        c_wut::WPAD_EXT_CLASSIC => unsafe {
                            s.hold |= Button::from_classic(status.__bindgen_anon_1.classic.hold);
                            s.trigger |=
                                Button::from_classic(status.__bindgen_anon_1.classic.trigger);
                            s.release |=
                                Button::from_classic(status.__bindgen_anon_1.classic.release);
                            s.left_stick = Some(status.__bindgen_anon_1.classic.leftStick.into());
                            s.right_stick = Some(status.__bindgen_anon_1.classic.rightStick.into());
                        },
                        c_wut::WPAD_EXT_PRO_CONTROLLER => unsafe {
                            s.hold |= Button::from_pro(status.__bindgen_anon_1.pro.hold);
                            s.trigger |= Button::from_pro(status.__bindgen_anon_1.pro.trigger);
                            s.release |= Button::from_pro(status.__bindgen_anon_1.pro.release);
                            s.left_stick = Some(status.__bindgen_anon_1.pro.leftStick.into());
                            s.right_stick = Some(status.__bindgen_anon_1.pro.rightStick.into());
                        },
                        _ => (),
                    }

                    Ok(s)
                }
            }
        }
    }
}

pub fn gamepads() /*-> Iter<Gamepad>*/
{
    todo!()
}

pub fn max_gamepads() -> u8 {
    todo!()
}
