// color

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// Red channel 0 - 255
    pub r: u8,
    /// Green channel 0 - 255
    pub g: u8,
    /// Blue channel 0 - 255
    pub b: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn red() -> Self {
        Self { r: 255, g: 0, b: 0 }
    }

    pub fn green() -> Self {
        Self { r: 0, g: 255, b: 0 }
    }

    pub fn blue() -> Self {
        Self { r: 0, g: 0, b: 255 }
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8)
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self {
            r: ((value >> 24) & 0xFF) as u8,
            g: ((value >> 16) & 0xFF) as u8,
            b: ((value >> 8) & 0xFF) as u8,
        }
    }
}

#[derive(Debug, Error)]
pub enum ColorParseError {
    #[error("string has invalid length (expected 6)")]
    InvalidLength,
    #[error("string contains invalid charcter (expect 0-9a-fA-F)")]
    InvalidCharacter(char),
}

impl TryFrom<&str> for Color {
    type Error = ColorParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 6 {
            return Err(ColorParseError::InvalidLength);
        }

        let mut result = 0;

        for c in value.chars() {
            result <<= 4;

            result |= match c {
                '0'..='9' => c as u32 - '0' as u32,
                'a'..='f' => c as u32 - 'a' as u32 + 10,
                'A'..='F' => c as u32 - 'A' as u32 + 10,
                _ => return Err(ColorParseError::InvalidCharacter(c)),
            };
        }

        Ok(Color::from(result))
    }
}
