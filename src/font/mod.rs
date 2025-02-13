//! Special Icons available on the system font
//!
//! Buttons, Logos, ...

pub mod icons {
    pub const BTN_A: char = '\u{E000}';
    pub const BTN_B: char = '\u{E001}';
    pub const BTN_X: char = '\u{E002}';
    pub const BTN_Y: char = '\u{E003}';
    pub const BTN_L: char = '\u{E004}';
    pub const BTN_R: char = '\u{E005}';
    pub const BTN_DPAD: char = '\u{E006}';

    pub const TARGET: char = '\u{E01D}';

    pub const CAPTURE_STILL: char = '\u{E01E}';
    pub const CAPTURE_VIDEO: char = '\u{E076}';

    pub const SPINNER_0: char = '\u{E020}';
    pub const SPINNER_1: char = '\u{E021}';
    pub const SPINNER_2: char = '\u{E022}';
    pub const SPINNER_3: char = '\u{E023}';
    pub const SPINNER_4: char = '\u{E024}';
    pub const SPINNER_5: char = '\u{E025}';
    pub const SPINNER_6: char = '\u{E026}';
    pub const SPINNER_7: char = '\u{E027}';

    pub const BTN_UP: char = '\u{E079}';
    pub const BTN_DOWN: char = '\u{E07A}';
    pub const BTN_LEFT: char = '\u{E07B}';
    pub const BTN_RIGHT: char = '\u{E07C}';
    pub const BTN_UP_DOWN: char = BTN_UP;
    pub const BTN_DOWN_UP: char = BTN_UP_DOWN;
    pub const BTN_LEFT_RIGHT: char = '\u{E07E}';
    pub const BTN_RIGHT_LEFT: char = BTN_LEFT_RIGHT;

    pub const WIIMOTE_BTN_POWER: char = '\u{E040}';
    pub const WIIMOTE_BTN_DPAD: char = '\u{E041}';
    pub const WIIMOTE_BTN_A: char = '\u{E042}';
    pub const WIIMOTE_BTN_B: char = '\u{E043}';
    pub const WIIMOTE_BTN_HOME: char = '\u{E044}';
    pub const WIIMOTE_BTN_PLUS: char = '\u{E045}';
    pub const WIIMOTE_BTN_MINUS: char = '\u{E046}';
    pub const WIIMOTE_BTN_1: char = '\u{E047}';
    pub const WIIMOTE_BTN_2: char = '\u{E048}';
    pub const WIIMOTE_BTN_UP: char = BTN_UP;
    pub const WIIMOTE_BTN_DOWN: char = BTN_DOWN;
    pub const WIIMOTE_BTN_LEFT: char = BTN_LEFT;
    pub const WIIMOTE_BTN_RIGHT: char = BTN_RIGHT;

    pub const NUNCHUK_STICK: char = '\u{E049}';
    pub const NUNCHUK_BTN_C: char = '\u{E04A}';
    pub const NUNCHUK_BTN_Z: char = '\u{E04B}';

    pub const CLASSIC_BTN_DPAD: char = WIIMOTE_BTN_DPAD;
    pub const CLASSIC_BTN_HOME: char = WIIMOTE_BTN_HOME;
    pub const CLASSIC_BTN_PLUS: char = WIIMOTE_BTN_PLUS;
    pub const CLASSIC_BTN_MINUS: char = WIIMOTE_BTN_MINUS;
    pub const CLASSIC_BTN_A: char = '\u{E04C}';
    pub const CLASSIC_BTN_B: char = '\u{E04D}';
    pub const CLASSIC_BTN_X: char = '\u{E04E}';
    pub const CLASSIC_BTN_Y: char = '\u{E04F}';
    pub const CLASSIC_STICK_L: char = '\u{E050}';
    pub const CLASSIC_STICK_R: char = '\u{E051}';
    pub const CLASSIC_BTN_L: char = '\u{E052}';
    pub const CLASSIC_BTN_R: char = '\u{E053}';
    pub const CLASSIC_BTN_ZL: char = '\u{E054}';
    pub const CLASSIC_BTN_ZR: char = '\u{E055}';
    pub const CLASSIC_BTN_UP: char = BTN_UP;
    pub const CLASSIC_BTN_DOWN: char = BTN_DOWN;
    pub const CLASSIC_BTN_LEFT: char = BTN_LEFT;
    pub const CLASSIC_BTN_RIGHT: char = BTN_RIGHT;

    pub const KBD_RETURN: char = '\u{E056}';
    pub const KBD_SPACE: char = '\u{E057}';

    pub const HAND_POINT: char = '\u{E058}';
    pub const HAND_POINT_1: char = '\u{E059}';
    pub const HAND_POINT_2: char = '\u{E05A}';
    pub const HAND_POINT_3: char = '\u{E05B}';
    pub const HAND_POINT_4: char = '\u{E05C}';

    pub const HAND_FIST: char = '\u{E05D}';
    pub const HAND_FIST_1: char = '\u{E05E}';
    pub const HAND_FIST_2: char = '\u{E05F}';
    pub const HAND_FIST_3: char = '\u{E060}';
    pub const HAND_FIST_4: char = '\u{E061}';

    pub const HAND_OPEN: char = '\u{E062}';
    pub const HAND_OPEN_1: char = '\u{E063}';
    pub const HAND_OPEN_2: char = '\u{E064}';
    pub const HAND_OPEN_3: char = '\u{E065}';
    pub const HAND_OPEN_4: char = '\u{E066}';

    /// Wii logo
    pub const WII: char = '\u{E067}';

    /// Question mark block icon.
    pub const HELP: char = '\u{E06B}';

    /// Close icon.
    pub const CLOSE: char = '\u{E070}';
    pub const CLOSE_ALT: char = '\u{E071}';

    /// Navigation: Back
    pub const BACK: char = '\u{E072}';
    /// Navigation: Home
    pub const HOME: char = '\u{E073}';

    /// Controller image: WiiU Gamepad
    pub const GAMEPAD: char = '\u{E087}';
    /// Controller image: Wiimote
    pub const WIIMOTE: char = '\u{E088}';

    /// 3DS: Circlepad
    pub const CIRCLEPAD: char = '\u{E077}';
    /// 3DS: Power button
    pub const BTN_POWER: char = '\u{E078}';
    /// 3DS: Step counter
    pub const STEPS: char = '\u{E074}';
    /// 3DS: Playcoin
    pub const PLAYCOIN: char = '\u{E075}';

    pub const BTN_TV: char = '\u{E089}';
    pub const ARROW_LEFT_RIGHT: char = '\u{E08C}';
    pub const ARROW_UP_DOWN: char = '\u{E08D}';
    pub const ARROW_CW: char = '\u{E08E}';
    pub const ARROW_CCW: char = '\u{E08F}';
    pub const ARROW_RIGHT: char = '\u{E090}';
    pub const ARROW_LEFT: char = '\u{E091}';
    pub const ARROW_UP: char = '\u{E092}';
    pub const ARROW_DOWN: char = '\u{E093}';
    pub const ARROW_UP_RIGHT: char = '\u{E094}';
    pub const X: char = '\u{E098}';
    pub const NFC: char = '\u{E099}';

    pub const SPACE: char = '\u{3000}';
}
