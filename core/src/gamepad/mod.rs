//! Basic unified gamepad interface
//!
//! This module simplifies gamepad input on the Wii U, offering a unified interface for various controller types. It focuses exclusively on **buttons and joysticks**, providing a common subset of input methods across different gamepads. This module does not offer full support for every controller's unique features.

use crate::rrc::{Rrc, RrcGuard};
use alloc::vec;
use core::{fmt::Debug, u16};
use flagset::{FlagSet, flags};
use thiserror::Error;
use wut_math::FloatingMathExt;
use wut_sys as sys;

pub(crate) static KPAD: Rrc = Rrc::new(
    || unsafe {
        sys::KPADInit();
    },
    || unsafe {
        sys::KPADShutdown();
    },
);

pub(crate) static VPAD: Rrc = Rrc::new(
    || unsafe {
        sys::VPADInit();
    },
    || unsafe {
        sys::VPADShutdown();
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
    const VPAD_BUTTON_MAPPING: [(u32, Button); 26] = [
        (sys::VPADButtons::VPAD_BUTTON_A, Button::A),
        (sys::VPADButtons::VPAD_BUTTON_B, Button::B),
        (sys::VPADButtons::VPAD_BUTTON_X, Button::X),
        (sys::VPADButtons::VPAD_BUTTON_Y, Button::Y),
        (sys::VPADButtons::VPAD_BUTTON_LEFT, Button::Left),
        (sys::VPADButtons::VPAD_BUTTON_RIGHT, Button::Right),
        (sys::VPADButtons::VPAD_BUTTON_UP, Button::Up),
        (sys::VPADButtons::VPAD_BUTTON_DOWN, Button::Down),
        (sys::VPADButtons::VPAD_BUTTON_L, Button::L),
        (sys::VPADButtons::VPAD_BUTTON_R, Button::R),
        (sys::VPADButtons::VPAD_BUTTON_ZL, Button::ZL),
        (sys::VPADButtons::VPAD_BUTTON_ZR, Button::ZR),
        (sys::VPADButtons::VPAD_BUTTON_PLUS, Button::Plus),
        (sys::VPADButtons::VPAD_BUTTON_MINUS, Button::Minus),
        (sys::VPADButtons::VPAD_BUTTON_HOME, Button::Home),
        (sys::VPADButtons::VPAD_BUTTON_SYNC, Button::Sync),
        (sys::VPADButtons::VPAD_BUTTON_STICK_R, Button::RStick),
        (sys::VPADButtons::VPAD_BUTTON_STICK_L, Button::LStick),
        (
            sys::VPADButtons::VPAD_STICK_R_EMULATION_LEFT,
            Button::RStickLeft,
        ),
        (
            sys::VPADButtons::VPAD_STICK_R_EMULATION_RIGHT,
            Button::RStickRight,
        ),
        (
            sys::VPADButtons::VPAD_STICK_R_EMULATION_UP,
            Button::RStickUp,
        ),
        (
            sys::VPADButtons::VPAD_STICK_R_EMULATION_DOWN,
            Button::RStickDown,
        ),
        (
            sys::VPADButtons::VPAD_STICK_L_EMULATION_LEFT,
            Button::LStickLeft,
        ),
        (
            sys::VPADButtons::VPAD_STICK_L_EMULATION_RIGHT,
            Button::LStickRight,
        ),
        (
            sys::VPADButtons::VPAD_STICK_L_EMULATION_UP,
            Button::LStickUp,
        ),
        (
            sys::VPADButtons::VPAD_STICK_L_EMULATION_DOWN,
            Button::LStickDown,
        ),
    ];

    const KPAD_BUTTON_MAPPING: [(u32, Button); 13] = [
        (sys::WPADButton::WPAD_BUTTON_LEFT, Button::Left),
        (sys::WPADButton::WPAD_BUTTON_RIGHT, Button::Right),
        (sys::WPADButton::WPAD_BUTTON_UP, Button::Up),
        (sys::WPADButton::WPAD_BUTTON_DOWN, Button::Down),
        (sys::WPADButton::WPAD_BUTTON_A, Button::A),
        (sys::WPADButton::WPAD_BUTTON_B, Button::B),
        (sys::WPADButton::WPAD_BUTTON_PLUS, Button::Plus),
        (sys::WPADButton::WPAD_BUTTON_MINUS, Button::Minus),
        (sys::WPADButton::WPAD_BUTTON_HOME, Button::Home),
        (sys::WPADButton::WPAD_BUTTON_1, Button::One),
        (sys::WPADButton::WPAD_BUTTON_2, Button::Two),
        (sys::WPADButton::WPAD_BUTTON_Z, Button::Z),
        (sys::WPADButton::WPAD_BUTTON_C, Button::C),
    ];

    const NUNCHUK_BUTTON_MAPPING: [(u32, Button); 6] = [
        (
            sys::WPADNunchukButton::WPAD_NUNCHUK_STICK_EMULATION_LEFT,
            Button::LStickLeft,
        ),
        (
            sys::WPADNunchukButton::WPAD_NUNCHUK_STICK_EMULATION_RIGHT,
            Button::LStickRight,
        ),
        (
            sys::WPADNunchukButton::WPAD_NUNCHUK_STICK_EMULATION_UP,
            Button::LStickUp,
        ),
        (
            sys::WPADNunchukButton::WPAD_NUNCHUK_STICK_EMULATION_DOWN,
            Button::LStickDown,
        ),
        (sys::WPADNunchukButton::WPAD_NUNCHUK_BUTTON_Z, Button::Z),
        (sys::WPADNunchukButton::WPAD_NUNCHUK_BUTTON_C, Button::C),
    ];

    const CLASSIC_BUTTON_MAPPING: [(u32, Button); 23] = [
        (sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_UP, Button::Up),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_DOWN,
            Button::Down,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_LEFT,
            Button::Left,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_RIGHT,
            Button::Right,
        ),
        (sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_A, Button::A),
        (sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_B, Button::B),
        (sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_X, Button::X),
        (sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_Y, Button::Y),
        (sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_L, Button::L),
        (sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_R, Button::R),
        (sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_ZL, Button::Left),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_ZR,
            Button::Right,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_PLUS,
            Button::Plus,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_MINUS,
            Button::Minus,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_BUTTON_HOME,
            Button::Home,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_STICK_L_EMULATION_LEFT,
            Button::LStickLeft,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_STICK_L_EMULATION_RIGHT,
            Button::LStickRight,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_STICK_L_EMULATION_UP,
            Button::LStickUp,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_STICK_L_EMULATION_DOWN,
            Button::LStickDown,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_STICK_R_EMULATION_LEFT,
            Button::RStickLeft,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_STICK_R_EMULATION_RIGHT,
            Button::RStickRight,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_STICK_R_EMULATION_UP,
            Button::RStickUp,
        ),
        (
            sys::WPADClassicButton::WPAD_CLASSIC_STICK_R_EMULATION_DOWN,
            Button::RStickDown,
        ),
    ];

    const PRO_BUTTON_MAPPING: [(u32, Button); 25] = [
        (sys::WPADProButton::WPAD_PRO_BUTTON_UP, Button::Up),
        (sys::WPADProButton::WPAD_PRO_BUTTON_DOWN, Button::Down),
        (sys::WPADProButton::WPAD_PRO_BUTTON_LEFT, Button::Left),
        (sys::WPADProButton::WPAD_PRO_BUTTON_RIGHT, Button::Right),
        (sys::WPADProButton::WPAD_PRO_BUTTON_A, Button::A),
        (sys::WPADProButton::WPAD_PRO_BUTTON_B, Button::B),
        (sys::WPADProButton::WPAD_PRO_BUTTON_X, Button::X),
        (sys::WPADProButton::WPAD_PRO_BUTTON_Y, Button::Y),
        (sys::WPADProButton::WPAD_PRO_TRIGGER_L, Button::L),
        (sys::WPADProButton::WPAD_PRO_TRIGGER_R, Button::R),
        (sys::WPADProButton::WPAD_PRO_TRIGGER_ZL, Button::ZL),
        (sys::WPADProButton::WPAD_PRO_TRIGGER_ZR, Button::ZR),
        (sys::WPADProButton::WPAD_PRO_BUTTON_PLUS, Button::Plus),
        (sys::WPADProButton::WPAD_PRO_BUTTON_MINUS, Button::Minus),
        (sys::WPADProButton::WPAD_PRO_BUTTON_HOME, Button::Home),
        (sys::WPADProButton::WPAD_PRO_BUTTON_STICK_L, Button::LStick),
        (sys::WPADProButton::WPAD_PRO_BUTTON_STICK_R, Button::RStick),
        (
            sys::WPADProButton::WPAD_PRO_STICK_L_EMULATION_LEFT,
            Button::LStickLeft,
        ),
        (
            sys::WPADProButton::WPAD_PRO_STICK_L_EMULATION_RIGHT,
            Button::LStickRight,
        ),
        (
            sys::WPADProButton::WPAD_PRO_STICK_L_EMULATION_UP,
            Button::LStickUp,
        ),
        (
            sys::WPADProButton::WPAD_PRO_STICK_L_EMULATION_DOWN,
            Button::LStickDown,
        ),
        (
            sys::WPADProButton::WPAD_PRO_STICK_R_EMULATION_LEFT,
            Button::RStickLeft,
        ),
        (
            sys::WPADProButton::WPAD_PRO_STICK_R_EMULATION_RIGHT,
            Button::RStickRight,
        ),
        (
            sys::WPADProButton::WPAD_PRO_STICK_R_EMULATION_UP,
            Button::RStickUp,
        ),
        (
            sys::WPADProButton::WPAD_PRO_STICK_R_EMULATION_DOWN,
            Button::RStickDown,
        ),
    ];

    /// New input with no buttons pressed
    ///
    /// # Examples
    ///
    /// ```
    /// use wut::gamepad::Button;
    ///
    /// assert_eq!(Button::none().is_empty(), true);
    /// ```
    #[inline]
    pub fn none() -> FlagSet<Button> {
        FlagSet::<Button>::default()
    }

    /// Converts native VPAD inputs to generic gamepad input
    ///
    /// # Examples
    ///
    /// ```
    /// use wut::gamepad::Button;
    ///
    /// assert_eq!(Button::from_vpad(wut::bindings::VPADButtons::VPAD_BUTTON_A), Button::A.into());
    /// ```
    #[inline]
    pub fn from_vpad(buttons: u32) -> FlagSet<Button> {
        Self::VPAD_BUTTON_MAPPING
            .iter()
            .fold(Default::default(), |mut b, &(flag, button)| {
                if buttons & flag != 0 {
                    b |= button;
                }
                b
            })
    }

    /// Converts generic gamepad input into native VPAD inputs
    ///
    /// # Examples
    ///
    /// ```
    /// use wut::gamepad::Button;
    ///
    /// assert_eq!(Button::into_vpad(Button::A), wut::bindings::VPADButtons::VPAD_BUTTON_A);
    /// ```
    #[inline]
    pub fn into_vpad(buttons: impl Into<FlagSet<Button>>) -> u32 {
        let buttons = buttons.into();
        Self::VPAD_BUTTON_MAPPING
            .iter()
            .fold(0, |b, &(flag, button)| {
                if buttons.contains(button) {
                    b | flag
                } else {
                    b
                }
            })
    }

    /// Converts native KPAD / WPAD inputs to generic gamepad input
    ///
    /// # Note
    /// KPAD is the high level API over WPAD
    ///
    /// # Examples
    ///
    /// ```
    /// use wut::gamepad::Button;
    ///
    /// assert_eq!(Button::from_kpad(wut::bindings::WPADButtons::WPAD_BUTTON_A), Button::A.into());
    /// ```
    #[inline]
    pub fn from_kpad(buttons: u32) -> FlagSet<Button> {
        Self::KPAD_BUTTON_MAPPING
            .iter()
            .fold(Default::default(), |mut b, &(flag, button)| {
                if buttons & flag != 0 {
                    b |= button;
                }
                b
            })
    }

    /// Converts generic gamepad input into native KPAD / WPAD inputs
    ///
    /// # Examples
    ///
    /// ```
    /// use wut::gamepad::Button;
    ///
    /// assert_eq!(Button::into_kpad(Button::A), wut::bindings::WPADButtons::WPAD_BUTTON_A);
    /// ```
    #[inline]
    pub fn into_kpad(buttons: impl Into<FlagSet<Button>>) -> u32 {
        let buttons = buttons.into();
        Self::KPAD_BUTTON_MAPPING
            .iter()
            .fold(0, |b, &(flag, button)| {
                if buttons.contains(button) {
                    b | flag
                } else {
                    b
                }
            })
    }

    /// Converts KPAD / WPAD extended Nunchuk inputs to generic gamepad inputs
    ///
    /// # Examples
    ///
    /// ```
    /// use wut::gamepad::Button;
    ///
    /// assert_eq!(Button::from_nunchuk(wut::bindings::WPADNunchukButton::WPAD_NUNCHUK_BUTTON_Z), Button:Z.into());
    /// ```
    #[inline]
    pub fn from_nunchuk(buttons: u32) -> FlagSet<Button> {
        Self::NUNCHUK_BUTTON_MAPPING
            .iter()
            .fold(Default::default(), |mut b, &(flag, button)| {
                if buttons & flag != 0 {
                    b |= button;
                }
                b
            })
    }

    /// Converts generic gamepad inputs to KPAD / WPAD extended Nunchuk inputs
    ///
    /// # Examples
    ///
    /// ```
    /// use wut::gamepad::Button;
    ///
    /// assert_eq!(Button::into_nunchuk(Button:Z), wut::bindings::WPADNunchukButton::WPAD_NUNCHUK_BUTTON_Z);
    /// ```
    #[inline]
    pub fn into_nunchuk(buttons: impl Into<FlagSet<Button>>) -> u32 {
        let buttons = buttons.into();
        Self::NUNCHUK_BUTTON_MAPPING
            .iter()
            .fold(0, |b, &(flag, button)| {
                if buttons.contains(button) {
                    b | flag
                } else {
                    b
                }
            })
    }

    /// Converts KPAD / WPAD extended Classic Controller inputs to generic gamepad inputs
    ///
    /// # Examples
    ///
    /// ```
    /// use wut::gamepad::Button;
    ///
    /// assert_eq!(Button::from_classic(wut::bindings::WPADClassicButton::WPAD_CLASSIC_BUTTON_ZL), Button::ZL.into());
    /// ```
    #[inline]
    pub fn from_classic(buttons: u32) -> FlagSet<Button> {
        Self::CLASSIC_BUTTON_MAPPING
            .iter()
            .fold(Default::default(), |mut b, &(flag, button)| {
                if buttons & flag != 0 {
                    b |= button;
                }
                b
            })
    }

    /// Converts generic gamepad inputs to KPAD / WPAD extended Classic Controller inputs
    ///
    /// # Examples
    ///
    /// ```
    /// use wut::gamepad::Button;
    ///
    /// assert_eq!(Button::into_classic(Button::ZL), wut::bindings::WPADClassicButton::WPAD_CLASSIC_BUTTON_ZL);
    /// ```
    #[inline]
    pub fn into_classic(buttons: impl Into<FlagSet<Button>>) -> u32 {
        let buttons = buttons.into();
        Self::CLASSIC_BUTTON_MAPPING
            .iter()
            .fold(0, |b, &(flag, button)| {
                if buttons.contains(button) {
                    b | flag
                } else {
                    b
                }
            })
    }

    /// Converts KPAD / WPAD "extended" Pro Controller inputs to generic gamepad inputs
    ///
    /// # Examples
    ///
    /// ```
    /// use wut::gamepad::Button;
    ///
    /// assert_eq!(Button::from_pro(wut::bindings::WPADProButton::WPAD_PRO_BUTTON_X), Button::X.into());
    /// ```
    #[inline]
    pub fn from_pro(buttons: u32) -> FlagSet<Button> {
        Self::PRO_BUTTON_MAPPING
            .iter()
            .fold(Default::default(), |mut b, &(flag, button)| {
                if buttons & flag != 0 {
                    b |= button;
                }
                b
            })
    }

    /// Converts generic gamepad inputs to KPAD / WPAD "extended" Pro Controller inputs
    ///
    /// # Examples
    ///
    /// ```
    /// use wut::gamepad::Button;
    ///
    /// assert_eq!(Button::into_pro(Button::X), wut::bindings::WPADProButton::WPAD_PRO_BUTTON_X);
    /// ```
    #[inline]
    pub fn into_pro(buttons: impl Into<FlagSet<Button>>) -> u32 {
        let buttons = buttons.into();
        Self::PRO_BUTTON_MAPPING
            .iter()
            .fold(0, |b, &(flag, button)| {
                if buttons.contains(button) {
                    b | flag
                } else {
                    b
                }
            })
    }
}

// pub trait DeviceConverter {
//     fn into_vpad(self) -> u32;
// }

// impl DeviceConverter for Button {
//     fn into_vpad(self) -> u32 {
//         sys::VPADButtons::VPAD_BUTTON_A
//     }
// }

// impl DeviceConverter for FlagSet<Button> {
//     fn into_vpad(self) -> u32 {
//         sys::VPADButtons::VPAD_BUTTON_A
//     }
// }

/// Represents a joystick with x and y coordinates.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Joystick {
    pub x: f32,
    pub y: f32,
}

impl Joystick {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        debug_assert!(x.abs() <= 1.0);
        debug_assert!(y.abs() <= 1.0);
        Self { x, y }
    }

    /// Calculates the absolute distance from the center point.
    ///
    /// # Returns
    ///
    /// A floating point value representing the distance from the center.
    #[inline]
    pub fn abs(&self) -> f32 {
        self.x.hypot(self.y)
    }

    /// Calculates the angle on the perimeter. `0` represents the joystick being held straight up.
    ///
    /// # Returns
    ///
    /// `None` if the joystick isn't moved, otherwise an angle in the range 0-65535.
    #[inline]
    pub fn angle(&self) -> Option<u16> {
        if self.x == 0.0 && self.y == 0.0 {
            None
        } else {
            let r = self.y.atan2(-self.x);
            let mut d = r.to_degrees();
            if d < 0.0 {
                d += 360.0
            }
            d = (90.0 - d + 360.0) % 360.0;
            let a = d * (u16::MAX as f32 / 360.0);
            Some(a as u16)
        }
    }
}

impl Into<Joystick> for sys::VPADVec2D {
    fn into(self) -> Joystick {
        Joystick {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<Joystick> for sys::KPADVec2D {
    fn into(self) -> Joystick {
        Joystick {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<sys::VPADVec2D> for Joystick {
    fn into(self) -> sys::VPADVec2D {
        sys::VPADVec2D {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<sys::KPADVec2D> for Joystick {
    fn into(self) -> sys::KPADVec2D {
        sys::KPADVec2D {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<Joystick> for (f32, f32) {
    fn into(self) -> Joystick {
        Joystick {
            x: self.0,
            y: self.1,
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

    #[inline]
    pub fn from_vpad(_value: sys::VPADChan::Type) -> Self {
        return Self::DRC;
    }

    #[inline]
    pub fn from_wpad(value: sys::WPADChan::Type) -> Self {
        use sys::WPADChan as C;
        match value {
            C::WPAD_CHAN_0 => Self::Port0,
            C::WPAD_CHAN_1 => Self::Port1,
            C::WPAD_CHAN_2 => Self::Port2,
            C::WPAD_CHAN_3 => Self::Port3,
            C::WPAD_CHAN_4 => Self::Port4,
            C::WPAD_CHAN_5 => Self::Port5,
            C::WPAD_CHAN_6 => Self::Port6,
            _ => unreachable!(),
        }
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

/// Represent an arbitrary gamepad on the Wii U.
///
/// Since some gamepads have no analog sticks (e.g. Wiimote) they are behind an `Option`. Special features (gyro, speakers, pointers, etc.) cannot be accesses with this.
pub struct Gamepad {
    pub port: Port,
    _resource: RrcGuard,
}

impl Debug for Gamepad {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Gamepad {{ port: {:?} }}", self.port)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct State {
    pub hold: FlagSet<Button>,
    pub trigger: FlagSet<Button>,
    pub release: FlagSet<Button>,
    pub left_stick: Option<Joystick>,
    pub right_stick: Option<Joystick>,
}

impl State {
    pub const fn new() -> Self {
        Self {
            hold: unsafe { FlagSet::new_unchecked(0) },
            trigger: unsafe { FlagSet::new_unchecked(0) },
            release: unsafe { FlagSet::new_unchecked(0) },
            left_stick: None,
            right_stick: None,
        }
    }

    // Convert generic gamepad input into native VPAD input
    #[inline]
    pub fn into_vpad(self) -> sys::VPADStatus {
        sys::VPADStatus {
            hold: Button::into_vpad(self.hold),
            trigger: Button::into_vpad(self.trigger),
            release: Button::into_vpad(self.release),
            leftStick: self.left_stick.unwrap_or_default().into(),
            rightStick: self.right_stick.unwrap_or_default().into(),
            ..Default::default()
        }
    }

    // #[inline]
    // pub fn into_kpad(self) -> sys::KPADStatus {
    //     todo!()
    // }
}

impl core::ops::BitOrAssign for State {
    fn bitor_assign(&mut self, rhs: Self) {
        self.hold |= rhs.hold;
        self.trigger |= rhs.trigger;
        self.release |= rhs.release;
        if let Some(stick) = rhs.left_stick {
            self.left_stick = stick.into();
        }
        if let Some(stick) = rhs.right_stick {
            self.right_stick = stick.into();
        }
    }
}

impl core::ops::BitOrAssign<State> for sys::VPADStatus {
    /// Combine [State] with [VPADStatus][sys::VPADStatus] constructively.
    ///
    /// Buttons are combined; Joysticks are overwritten if they have values. All other fields of controller input are unchanged (gyro, touch, etc.).
    fn bitor_assign(&mut self, rhs: State) {
        self.hold |= Button::into_vpad(rhs.hold);
        self.trigger |= Button::into_vpad(rhs.trigger);
        self.release |= Button::into_vpad(rhs.release);
        if let Some(stick) = rhs.left_stick {
            self.leftStick = stick.into();
        }
        if let Some(stick) = rhs.right_stick {
            self.rightStick = stick.into();
        }
    }
}

impl core::ops::BitAndAssign<State> for sys::VPADStatus {
    /// Combine [State] with [VPADStatus][sys::VPADStatus] destructively.
    ///
    /// Buttons are overwritten; Joysticks are overwritten or set to default values. All other fields of controller input are unchanged (gyro, touch, etc.).
    fn bitand_assign(&mut self, rhs: State) {
        self.hold = Button::into_vpad(rhs.hold);
        self.trigger = Button::into_vpad(rhs.trigger);
        self.release = Button::into_vpad(rhs.release);
        self.leftStick = rhs.left_stick.unwrap_or_default().into();
        self.rightStick = rhs.right_stick.unwrap_or_default().into();
    }
}

impl core::ops::BitOrAssign<State> for sys::KPADStatus {
    /// Combine [State] with [KPADStatus][sys::KPADStatus] constructively.
    ///
    /// Buttons are combined; Joysticks are overwritten if they have values. All other fields of controller input are unchanged (gyro, touch, etc.).
    fn bitor_assign(&mut self, rhs: State) {
        self.hold |= Button::into_kpad(rhs.hold);
        self.trigger |= Button::into_kpad(rhs.trigger);
        self.release |= Button::into_kpad(rhs.release);

        use sys::WPADExtensionType as Ext;
        match self.extensionType as Ext::Type {
            Ext::WPAD_EXT_NUNCHUK => unsafe {
                self.__bindgen_anon_1.nunchuk.hold |= Button::into_nunchuk(rhs.hold);
                self.__bindgen_anon_1.nunchuk.trigger |= Button::into_nunchuk(rhs.trigger);
                self.__bindgen_anon_1.nunchuk.release |= Button::into_nunchuk(rhs.release);
                if let Some(stick) = rhs.left_stick {
                    self.__bindgen_anon_1.nunchuk.stick = stick.into();
                }
            },
            Ext::WPAD_EXT_CLASSIC => unsafe {
                self.__bindgen_anon_1.classic.hold |= Button::into_classic(rhs.hold);
                self.__bindgen_anon_1.classic.trigger |= Button::into_classic(rhs.trigger);
                self.__bindgen_anon_1.classic.release |= Button::into_classic(rhs.release);
                if let Some(stick) = rhs.left_stick {
                    self.__bindgen_anon_1.classic.leftStick = stick.into();
                }
                if let Some(stick) = rhs.right_stick {
                    self.__bindgen_anon_1.classic.rightStick = stick.into();
                }
            },
            Ext::WPAD_EXT_PRO_CONTROLLER => unsafe {
                self.__bindgen_anon_1.pro.hold |= Button::into_pro(rhs.hold);
                self.__bindgen_anon_1.pro.trigger |= Button::into_pro(rhs.trigger);
                self.__bindgen_anon_1.pro.release |= Button::into_pro(rhs.release);
                if let Some(stick) = rhs.left_stick {
                    self.__bindgen_anon_1.pro.leftStick = stick.into();
                }
                if let Some(stick) = rhs.right_stick {
                    self.__bindgen_anon_1.pro.rightStick = stick.into();
                }
            },
            _ => (),
        }
    }
}

impl core::ops::BitAndAssign<State> for sys::KPADStatus {
    /// Combine [State] with [KPADStatus][sys::KPADStatus] destructively.
    ///
    /// Buttons are overwritten; Joysticks are overwritten or set to default values. All other fields of controller input are unchanged (gyro, touch, etc.).
    fn bitand_assign(&mut self, rhs: State) {
        self.hold = Button::into_kpad(rhs.hold);
        self.trigger = Button::into_kpad(rhs.trigger);
        self.release = Button::into_kpad(rhs.release);

        use sys::WPADExtensionType as Ext;
        match self.extensionType as Ext::Type {
            Ext::WPAD_EXT_NUNCHUK => {
                self.__bindgen_anon_1.nunchuk.hold = Button::into_nunchuk(rhs.hold);
                self.__bindgen_anon_1.nunchuk.trigger = Button::into_nunchuk(rhs.trigger);
                self.__bindgen_anon_1.nunchuk.release = Button::into_nunchuk(rhs.release);
                self.__bindgen_anon_1.nunchuk.stick = rhs.left_stick.unwrap_or_default().into();
            }
            Ext::WPAD_EXT_CLASSIC => {
                self.__bindgen_anon_1.classic.hold = Button::into_classic(rhs.hold);
                self.__bindgen_anon_1.classic.trigger = Button::into_classic(rhs.trigger);
                self.__bindgen_anon_1.classic.release = Button::into_classic(rhs.release);
                self.__bindgen_anon_1.classic.leftStick = rhs.left_stick.unwrap_or_default().into();
                self.__bindgen_anon_1.classic.rightStick =
                    rhs.right_stick.unwrap_or_default().into();
            }
            Ext::WPAD_EXT_PRO_CONTROLLER => {
                self.__bindgen_anon_1.pro.hold = Button::into_pro(rhs.hold);
                self.__bindgen_anon_1.pro.trigger = Button::into_pro(rhs.trigger);
                self.__bindgen_anon_1.pro.release = Button::into_pro(rhs.release);
                self.__bindgen_anon_1.pro.leftStick = rhs.left_stick.unwrap_or_default().into();
                self.__bindgen_anon_1.pro.rightStick = rhs.right_stick.unwrap_or_default().into();
            }
            _ => (),
        }
    }
}

impl From<sys::VPADStatus> for State {
    fn from(value: sys::VPADStatus) -> Self {
        State {
            hold: Button::from_vpad(value.hold),
            trigger: Button::from_vpad(value.trigger),
            release: Button::from_vpad(value.release),
            left_stick: Some(value.leftStick.into()),
            right_stick: Some(value.rightStick.into()),
        }
    }
}

impl From<sys::KPADStatus> for State {
    fn from(value: sys::KPADStatus) -> Self {
        use sys::WPADExtensionType as Ext;

        let mut s = State {
            hold: Button::from_kpad(value.hold),
            trigger: Button::from_kpad(value.trigger),
            release: Button::from_kpad(value.release),
            left_stick: None,
            right_stick: None,
        };

        match value.extensionType as Ext::Type {
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

// impl From<i32> for GamepadError {
//     #[allow(unreachable_patterns)] // to make it really clear what is matched.
//     fn from(value: i32) -> Self {
//         use sys::KPADError as KPAD;
//         use sys::VPADReadError as VPAD;
//         match value {
//             VPAD::VPAD_READ_NO_SAMPLES | KPAD::KPAD_ERROR_NO_SAMPLES => Self::NoSamples,
//             VPAD::VPAD_READ_INVALID_CONTROLLER | KPAD::KPAD_ERROR_INVALID_CONTROLLER => {
//                 Self::InvalidController
//             }
//             VPAD::VPAD_READ_BUSY | KPAD::KPAD_ERROR_BUSY => Self::Busy,
//             VPAD::VPAD_READ_UNINITIALIZED
//             | KPAD::KPAD_ERROR_WPAD_UNINIT
//             | KPAD::KPAD_ERROR_UNINITIALIZED => Self::Uninitialized,
//             _ => panic!("Unknown error code: {}", value),
//         }
//     }
// }

impl TryFrom<i32> for GamepadError {
    type Error = Self;
    #[allow(unreachable_patterns)] // to make it really clear what is matched.
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use sys::KPADError as KPAD;
        use sys::VPADReadError as VPAD;
        match value {
            VPAD::VPAD_READ_SUCCESS | KPAD::KPAD_ERROR_OK => Ok(Self::Busy),
            VPAD::VPAD_READ_NO_SAMPLES | KPAD::KPAD_ERROR_NO_SAMPLES => Err(Self::NoSamples),
            VPAD::VPAD_READ_INVALID_CONTROLLER | KPAD::KPAD_ERROR_INVALID_CONTROLLER => {
                Err(Self::InvalidController)
            }
            VPAD::VPAD_READ_BUSY | KPAD::KPAD_ERROR_BUSY => Err(Self::Busy),
            VPAD::VPAD_READ_UNINITIALIZED
            | KPAD::KPAD_ERROR_WPAD_UNINIT
            | KPAD::KPAD_ERROR_UNINITIALIZED => Err(Self::Uninitialized),
            _ => Err(Self::Uninitialized),
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

    pub fn poll(&self) -> Result<State, GamepadError> {
        match self.port {
            Port::DRC => {
                use sys::VPADReadError as E;

                let mut status = sys::VPADStatus::default();
                let mut error = E::VPAD_READ_SUCCESS;

                if unsafe { sys::VPADRead(self.port.into(), &mut status, 1, &mut error) } == 0
                    && error != E::VPAD_READ_SUCCESS
                {
                    Err(GamepadError::try_from(error)?)
                } else {
                    Ok(State::from(status))
                }
            }
            _ => {
                use sys::KPADError as E;

                let mut status = sys::KPADStatus::default();
                let mut error = E::KPAD_ERROR_OK;

                if unsafe { sys::KPADReadEx(self.port.into(), &mut status, 1, &mut error) } == 0
                    && error != E::KPAD_ERROR_OK
                {
                    Err(GamepadError::try_from(error)?)
                } else {
                    Ok(State::from(status))
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
    unsafe { sys::KPADGetGameMaxControllers() }
        .try_into()
        .expect("Max # of gamecontroller can NEVER exceed `u8::MAX`")
}
