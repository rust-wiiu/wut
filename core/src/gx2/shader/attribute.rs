use super::buffer::Buffer;
use ::core::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use wut_math::FloatingMathExt;
use wut_sys as sys;

pub trait Attributes: Sized {
    fn new(group: &mut sys::WHBGfxShaderGroup) -> Result<Self, ()>;
}

pub struct Attribute<T: AttributeFormat> {
    index: u32,
    offset: u32,
    _marker: PhantomData<T>,
}

impl<T: AttributeFormat> Attribute<T> {
    pub fn new(
        group: &mut sys::WHBGfxShaderGroup,
        name: &str,
        index: u32,
        offset: u32,
    ) -> Result<Self, ()> {
        let s = Self {
            index,
            offset,
            _marker: PhantomData,
        };

        let name = alloc::ffi::CString::new(name).unwrap();
        if unsafe {
            sys::WHBGfxInitShaderAttribute(
                group,
                name.as_ptr(),
                s.index,
                s.offset,
                T::gx2_attribute_format(),
            )
        } == 0
        {
            Err(())
        } else {
            Ok(s)
        }
    }

    pub fn set_buffer(&mut self, buffer: &mut Buffer<T>) {
        unsafe {
            sys::GX2RSetAttributeBuffer(
                buffer.as_raw_mut(),
                self.index,
                buffer.stride() as u32,
                self.offset,
            );
        }
    }
}

// region: AttributeFormat

macro_rules! cond_impl_neg {
    ($name:ident, f32, $($field:tt),*) => {
        impl Neg for $name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self {
                    $($field: -self.$field),*
                }
            }
        }
    };

    ($name:ident, i8, $($field:tt),*) => {
        impl Neg for $name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self {
                    $($field: -self.$field),*
                }
            }
        }
    };

    ($name:ident, $type:ty, $($field:tt),*) => {};
}

macro_rules! struct_impl {
    ($name:ident, $type:tt, $($field:tt),*) => {
        // Add implementation
        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    $($field: self.$field + rhs.$field),*
                }
            }
        }

        // Subtract implementation
        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    $($field: self.$field - rhs.$field),*
                }
            }
        }

        // Multiply implementation
        impl Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self {
                    $($field: self.$field * rhs.$field),*
                }
            }
        }

        // Divide implementation
        impl Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self {
                    $($field: self.$field / rhs.$field),*
                }
            }
        }

        // AddAssign implementation
        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$field += rhs.$field);*
            }
        }

        // SubAssign implementation
        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$field -= rhs.$field);*
            }
        }

        // MulAssign implementation
        impl MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                $(self.$field *= rhs.$field);*
            }
        }

        // DivAssign implementation
        impl DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                $(self.$field /= rhs.$field);*
            }
        }

        // Scalar operations (vector op scalar)
        impl Mul<$type> for $name {
            type Output = Self;

            fn mul(self, scalar: $type) -> Self::Output {
                Self {
                    $($field: self.$field * scalar),*
                }
            }
        }

        impl Div<$type> for $name {
            type Output = Self;

            fn div(self, scalar: $type) -> Self::Output {
                Self {
                    $($field: self.$field / scalar),*
                }
            }
        }

        impl MulAssign<$type> for $name {
            fn mul_assign(&mut self, scalar: $type) {
                $(self.$field *= scalar);*
            }
        }

        impl DivAssign<$type> for $name {
            fn div_assign(&mut self, scalar: $type) {
                $(self.$field /= scalar);*
            }
        }

        cond_impl_neg!($name, $type, $($field),*);
    };
}

macro_rules! struct_def {
    ($(#[doc = $doc:expr])? $name:ident, $type:tt, $($field:tt),*) => {
        $(#[doc = $doc])?
        #[derive(Debug, Default, Clone, Copy)]
        #[repr(C)]
        pub struct $name {
            $(pub $field: $type),*
        }
    };
    ($(#[doc = $doc:expr])? $name:ident, $type:tt) => {
        $(#[doc = $doc])?
        #[derive(Debug, Default, Clone, Copy)]
        #[repr(C)]
        pub struct $name(pub $type);
    };
}

macro_rules! impl_from {
    ($name:ident, $type:ty) => {
        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }
    };

    ($name:ident, $type:ty, $field1:ident, $field2:ident) => {
        impl From<($type, $type)> for $name {
            fn from(value: ($type, $type)) -> Self {
                Self {
                    $field1: value.0,
                    $field2: value.1,
                }
            }
        }
    };

    ($name:ident, $type:ty, $field1:ident, $field2:ident, $field3:ident) => {
        impl From<($type, $type, $type)> for $name {
            fn from(value: ($type, $type, $type)) -> Self {
                Self {
                    $field1: value.0,
                    $field2: value.1,
                    $field3: value.2,
                }
            }
        }
    };

    ($name:ident, $type:ty, $field1:ident, $field2:ident, $field3:ident, $field4:ident) => {
        impl From<($type, $type, $type, $type)> for $name {
            fn from(value: ($type, $type, $type, $type)) -> Self {
                Self {
                    $field1: value.0,
                    $field2: value.1,
                    $field3: value.2,
                    $field4: value.3,
                }
            }
        }
    };
}

macro_rules! attribute_format {
    ($(#[doc = $doc:expr])? $name:ident, $type:tt, $($field:tt),*) => {
        struct_def!($(#[doc = $doc])? $name, $type, $($field),*);
        struct_impl!($name, $type, $($field),*);

        impl_from!($name, $type, $($field),*);
    };
    ($(#[doc = $doc:expr])? $name:ident, $type:tt) => {
        struct_def!($(#[doc = $doc])? $name, $type);
        struct_impl!($name, $type, 0);
        impl_from!($name, $type);
    };
}

attribute_format!(
    /// 4 element f32 vector.
    Float4, f32, x, y, z, w);
impl AttributeFormat for Float4 {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_FLOAT_32_32_32_32
    }
}

impl From<crate::gx2::color::Color> for Float4 {
    fn from(value: crate::gx2::color::Color) -> Self {
        Self {
            x: value.r as f32,
            y: value.g as f32,
            z: value.b as f32,
            w: value.a as f32,
        }
    }
}

attribute_format!(
    /// 3 element f32 vector.
    Float3, f32, x, y, z);
impl AttributeFormat for Float3 {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_FLOAT_32_32_32
    }
}

attribute_format!(
    /// 2 element f32 vector.
    Float2, f32, x, y);
impl AttributeFormat for Float2 {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_FLOAT_32_32
    }
}

attribute_format!(
    /// f32 scalar.
    Float, f32);
impl AttributeFormat for Float {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_FLOAT_32
    }
}

attribute_format!(
    // 4 element i8 vector.
    Int4, i8, x, y, z, w
);
impl AttributeFormat for Int4 {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_SINT_8_8_8_8
    }
}

attribute_format!(
    /// 2 element i8 vector.
    Int2, i8, x, y);
impl AttributeFormat for Int2 {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_SINT_8_8
    }
}

attribute_format!(
    // i8 scalar.
    Int, i8
);
impl AttributeFormat for Int {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_SINT_8
    }
}

attribute_format!(
    /// 4 element u8 vector.
    Uint4, u8, x, y, z, w);
impl AttributeFormat for Uint4 {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_UINT_8_8_8_8
    }
}

attribute_format!(
    /// 3 element u8 vector.
    Uint2, u8, x, y);
impl AttributeFormat for Uint2 {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_UINT_8_8
    }
}

attribute_format!(
    /// u8 scalar.
    Uint, u8);
impl AttributeFormat for Uint {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_UINT_8
    }
}

attribute_format!(
    /// 4 element signed normed 8bit decimal vector.
    Snorm4, i8, x, y, z, w);
impl AttributeFormat for Snorm4 {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_SNORM_8_8_8_8
    }
}

attribute_format!(
    /// 2 element signed normed 8bit decimal vector.
    Snorm2, i8, x, y);
impl AttributeFormat for Snorm2 {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_SNORM_8_8
    }
}

attribute_format!(
    /// Signed normed 8bit decimal scalar.
    Snorm, i8);
impl AttributeFormat for Snorm {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_SNORM_8
    }
}

impl From<f32> for Snorm {
    fn from(value: f32) -> Self {
        const MAX: f32 = i8::MAX as f32;
        Self((value * MAX).round().clamp(-MAX, MAX) as i8)
    }
}

impl From<(f32, f32)> for Snorm2 {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: Snorm::from(value.0).0,
            y: Snorm::from(value.1).0,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Snorm4 {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self {
            x: Snorm::from(value.0).0,
            y: Snorm::from(value.1).0,
            z: Snorm::from(value.2).0,
            w: Snorm::from(value.3).0,
        }
    }
}

attribute_format!(
    /// 4 element unsigned normed 8bit decimal vector.
    Norm4, u8, x, y, z, w);
impl AttributeFormat for Norm4 {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_UNORM_8_8_8_8
    }
}

attribute_format!(
    /// 2 element unsigned normed 8bit decimal vector.
    Norm2, u8, x, y);
impl AttributeFormat for Norm2 {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_UNORM_8_8
    }
}

attribute_format!(
    /// Unsigned normed 8bit decimal scalar.
    Norm, u8);
impl AttributeFormat for Norm {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type {
        sys::GX2AttribFormat::GX2_ATTRIB_FORMAT_UNORM_8
    }
}

impl From<f32> for Norm {
    fn from(value: f32) -> Self {
        Self((value.clamp(0.0, 1.0) * 255.0).round() as u8)
    }
}

impl From<(f32, f32)> for Norm2 {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: Norm::from(value.0).0,
            y: Norm::from(value.1).0,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Norm4 {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self {
            x: Norm::from(value.0).0,
            y: Norm::from(value.1).0,
            z: Norm::from(value.2).0,
            w: Norm::from(value.3).0,
        }
    }
}

pub trait AttributeFormat: Default + Clone + Copy {
    fn gx2_attribute_format() -> sys::GX2AttribFormat::Type;
}

// endregion
