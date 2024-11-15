// generic

use crate::gamepad;
use flagset::{flags, FlagSet};

pub struct Generic {
    pub hold: FlagSet<gamepad::Button>,
    pub trigger: FlagSet<gamepad::Button>,
    pub release: FlagSet<gamepad::Button>,
    pub left_stick: gamepad::Stick,
    pub right_stick: gamepad::Stick,
}
