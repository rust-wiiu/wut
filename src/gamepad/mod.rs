//! Simple gamepad support
//!
//! Purposefully kept simple and not supporting all features of the controllers.
//! If finer controll and access to controller specific features is required, use the more complex "...", which I maybe add later.

use crate::bindings as c_wut;
use crate::sync::Rrc;
use flagset::{flags, FlagSet};
use thiserror::Error;

pub(crate) static WPAD: Rrc<fn(), fn()> = Rrc::new(
    || unsafe {
        c_wut::WPADInit();
    },
    || unsafe {
        c_wut::WPADShutdown();
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

#[derive(Debug)]
pub struct Joystick {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug)]
pub struct Gamepad {
    pub port: Port,
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
    #[error("TODO")]
    Todo,
}

impl Gamepad {
    pub fn new(port: Port) -> Result<Self, GamepadError> {
        match port {
            Port::DRC => {
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn poll(&self) -> Result<GamepadState, GamepadError> {
        match self.port {
            Port::DRC => {
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn name(&self) -> &'static str {
        todo!()
    }
}

pub fn gamepads() /*-> Iter<Gamepad>*/
{
    todo!()
}

pub fn max_gamepads() -> u8 {
    todo!()
}
