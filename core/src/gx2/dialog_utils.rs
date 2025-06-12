use wut_sys as sys;

pub enum Language {
    Japanese,
    English,
    French,
    German,
    Italian,
    Spanish,
    SimplifiedChinese,
    Korean,
    Dutch,
    Portuguese,
    Russian,
    TraditionalChinese,
}

impl Into<i32> for Language {
    fn into(self) -> i32 {
        // Check at compile time that values are equal
        const _: () = {
            macro_rules! check_lang_types {
                ($($variant:ident),*) => {
                    // These will fail compilation if the values don't match
                    #[allow(unused_comparisons)]
                    const _: bool = {
                        $(
                            sys::nn_erreula_LangType::$variant ==
                            sys::nn_swkbd_LanguageType::$variant
                        )&&*
                    };
                }
            }

            check_lang_types!(
                Japanese,
                English,
                French,
                German,
                Italian,
                Spanish,
                SimplifiedChinese,
                Korean,
                Dutch,
                Portuguese,
                Russian,
                TraditionalChinese
            );
        };

        use sys::nn_erreula_LangType as T;
        match self {
            Self::Japanese => T::Japanese,
            Self::English => T::English,
            Self::French => T::French,
            Self::German => T::German,
            Self::Italian => T::Italian,
            Self::Spanish => T::Spanish,
            Self::SimplifiedChinese => T::SimplifiedChinese,
            Self::Korean => T::Korean,
            Self::Dutch => T::Dutch,
            Self::Portuguese => T::Portuguese,
            Self::Russian => T::Russian,
            Self::TraditionalChinese => T::TraditionalChinese,
        }
    }
}

pub enum Controller {
    Wiimote0,
    Wiimote1,
    Wiimote2,
    Wiimote3,
    DrcGamepad,
}

impl Into<i32> for Controller {
    fn into(self) -> i32 {
        // Check at compile time that values are equal
        const _: () = {
            macro_rules! check_lang_types {
                ($($variant:ident),*) => {
                    // These will fail compilation if the values don't match
                    #[allow(unused_comparisons)]
                    const _: bool = {
                        $(
                            sys::nn_erreula_ControllerType::$variant ==
                            sys::nn_swkbd_ControllerType::$variant
                        )&&*
                    };
                }
            }

            check_lang_types!(WiiRemote0, WiiRemote1, WiiRemote2, WiiRemote3, DrcGamepad);
        };

        use sys::nn_swkbd_ControllerType as T;
        match self {
            Self::Wiimote0 => T::WiiRemote0,
            Self::Wiimote1 => T::WiiRemote1,
            Self::Wiimote2 => T::WiiRemote2,
            Self::Wiimote3 => T::WiiRemote3,
            Self::DrcGamepad => T::DrcGamepad,
        }
    }
}

pub enum Region {
    Japan,
    USA,
    Europe,
    China,
    Korea,
    Taiwan,
}

impl Into<i32> for Region {
    fn into(self) -> i32 {
        // Check at compile time that values are equal
        const _: () = {
            macro_rules! check_lang_types {
                ($($variant:ident),*) => {
                    // These will fail compilation if the values don't match
                    #[allow(unused_comparisons)]
                    const _: bool = {
                        $(
                            sys::nn_erreula_RegionType::$variant ==
                            sys::nn_swkbd_RegionType::$variant
                        )&&*
                    };
                }
            }

            check_lang_types!(Japan, USA, Europe, China, Korea, Taiwan);
        };

        use sys::nn_swkbd_RegionType as T;
        match self {
            Self::Japan => T::Japan,
            Self::USA => T::USA,
            Self::Europe => T::Europe,
            Self::China => T::China,
            Self::Korea => T::Korea,
            Self::Taiwan => T::Taiwan,
        }
    }
}

#[derive(Default)]
pub(crate) struct ControllerInfo {
    pub vpad: sys::VPADStatus,
    pub kpad: [sys::KPADStatus; 4],
}

impl ControllerInfo {
    pub fn read_vpad(&mut self) {
        unsafe {
            sys::VPADRead(
                sys::VPADChan::VPAD_CHAN_0,
                &mut self.vpad,
                1,
                core::ptr::null_mut(),
            );
            sys::VPADGetTPCalibratedPoint(
                sys::VPADChan::VPAD_CHAN_0,
                &mut self.vpad.tpNormal,
                &mut self.vpad.tpNormal,
            );
        }
    }

    pub fn as_swkbd(&mut self) -> sys::nn_swkbd_ControllerInfo {
        sys::nn_swkbd_ControllerInfo {
            vpad: &mut self.vpad,
            kpad: [
                &mut self.kpad[0],
                &mut self.kpad[0],
                &mut self.kpad[0],
                &mut self.kpad[0],
            ],
        }
    }

    pub fn as_erreula(&mut self) -> sys::nn_erreula_ControllerInfo {
        sys::nn_erreula_ControllerInfo {
            vpad: &self.vpad,
            kpad: [&self.kpad[0], &self.kpad[0], &self.kpad[0], &self.kpad[0]],
        }
    }
}
