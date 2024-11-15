// input

mod drc;
mod generic;

use flagset::{flags, FlagSet};

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

pub struct Stick {
    pub x: f32,
    pub y: f32,
}
