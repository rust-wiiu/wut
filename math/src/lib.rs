#![no_std]

mod bindings;

use bindings as sys;
use core::{f32, f64};

pub trait FloatingMathExt {
    /// Computes the arccosine of a number. Return value is in radians in the range [0, pi] or NaN if the number is outside the range [-1, 1].
    fn acos(self) -> Self;

    /// Inverse hyperbolic cosine function.
    fn acosh(self) -> Self;

    /// Computes the arcsine of a number. Return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
    fn asin(self) -> Self;

    /// Inverse hyperbolic sine function.
    fn asinh(self) -> Self;

    /// Computes the arctangent of a number. Return value is in radians in the range [-pi/2, pi/2];
    fn atan(self) -> Self;

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    fn atan2(self, other: Self) -> Self;

    /// Inverse hyperbolic tangent function.
    fn atanh(self) -> Self;

    /// Returns the cube root of a number.
    fn cbrt(self) -> Self;

    /// Returns the smallest integer greater than or equal to self.
    ///
    /// This function always returns the precise result.
    fn ceil(self) -> Self;

    /// Computes the cosine of a number (in radians).
    fn cos(self) -> Self;

    /// Hyperbolic cosine function.
    fn cosh(self) -> Self;

    /// Returns `e^(self)`, (the exponential function).
    fn exp(self) -> Self;

    /// Returns `2^(self)`.
    fn exp2(self) -> Self;

    /// Returns `e^(self) - 1` in a way that is accurate even if the number is close to zero.
    fn exp_m1(self) -> Self; // expm1

    /// The positive difference of two numbers.
    ///
    /// * If `self <= other`: `0.0`
    /// * Else: `self - other`
    fn abs_sub(self, other: Self) -> Self;

    /// Returns the largest integer less than or equal to self.
    ///
    /// This function always returns the precise result.
    fn floor(self) -> Self;

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target architecture has a dedicated `fma` CPU instruction. However, this is not always true, and will be heavily dependant on designing algorithms with specific target hardware in mind.
    ///
    /// # Precision
    ///
    /// The result of this operation is guaranteed to be the rounded infinite-precision result. It is specified by IEEE 754 as `fusedMultiplyAdd` and guaranteed not to change.
    fn mul_add(self, a: Self, b: Self) -> Self; // fma

    /// Computes the floating-point remainder of `x / y`.
    ///
    /// The return value is `x - n * y`, where `n` is the integer quotient of `x / y` rounded toward zero.
    fn fmod(self, y: Self) -> Self;

    /// Compute the distance between the origin and a point (`x`, `y`) on the Euclidean plane. Equivalently, compute the length of the hypotenuse of a right-angle triangle with other sides having length `x.abs()` and `y.abs()`.
    fn hypot(self, other: Self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// The result might not be correctly rounded owing to implementation details; self.log2() can produce more accurate results for base 2, and self.log10() can produce more accurate results for base 10.
    fn log(self) -> Self;

    /// Returns the base 10 logarithm of the number.
    fn log10(self) -> Self;

    /// Returns the base 2 logarithm of the number.
    fn log2(self) -> Self;

    // Raises a number to a floating point power.
    fn powf(self, n: Self) -> Self; // pow

    /// Computes the sine of a number (in radians).
    fn sin(self) -> Self;

    /// Hyperbolic sine function.
    fn sinh(self) -> Self;

    /// Returns the square root of a number.
    ///
    /// Returns `NaN` if self is a negative number other than `-0.0`.
    fn sqrt(self) -> Self;

    /// Computes the tangent of a number (in radians).
    fn tan(self) -> Self;

    /// Hyperbolic tangent function.
    fn tanh(self) -> Self;

    /// Gamma function.
    fn gamma(self) -> Self; // tgamma

    /// Convert radians to degree.
    fn to_degrees(self) -> Self;

    /// Convert degree to radians.
    fn to_radians(self) -> Self;

    /// Round down if `x < 0.5` else up.
    fn round(self) -> Self;
}

impl FloatingMathExt for f32 {
    fn acos(self) -> Self {
        unsafe { sys::acosf(self) }
    }

    fn acosh(self) -> Self {
        unsafe { sys::acoshf(self) }
    }

    fn asin(self) -> Self {
        unsafe { sys::asinf(self) }
    }

    fn asinh(self) -> Self {
        unsafe { sys::asinhf(self) }
    }

    fn atan(self) -> Self {
        unsafe { sys::atanf(self) }
    }

    fn atan2(self, other: Self) -> Self {
        unsafe { sys::atan2f(self, other) }
    }

    fn atanh(self) -> Self {
        unsafe { sys::atanhf(self) }
    }

    fn cbrt(self) -> Self {
        unsafe { sys::cbrtf(self) }
    }

    fn ceil(self) -> Self {
        unsafe { sys::ceilf(self) }
    }

    fn cos(self) -> Self {
        unsafe { sys::cosf(self) }
    }

    fn cosh(self) -> Self {
        unsafe { sys::coshf(self) }
    }

    fn exp(self) -> Self {
        unsafe { sys::expf(self) }
    }

    fn exp2(self) -> Self {
        unsafe { sys::exp2f(self) }
    }

    fn exp_m1(self) -> Self {
        unsafe { sys::expm1f(self) }
    }

    fn abs_sub(self, other: Self) -> Self {
        unsafe { sys::fdimf(self, other) }
    }

    fn floor(self) -> Self {
        unsafe { sys::floorf(self) }
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        unsafe { sys::fmaf(self, a, b) }
    }

    fn fmod(self, other: Self) -> Self {
        unsafe { sys::fmodf(self, other) }
    }

    fn hypot(self, other: Self) -> Self {
        unsafe { sys::hypotf(self, other) }
    }

    fn log(self) -> Self {
        unsafe { sys::logf(self) }
    }

    fn log10(self) -> Self {
        unsafe { sys::log10f(self) }
    }

    fn log2(self) -> Self {
        unsafe { sys::log2f(self) }
    }

    fn powf(self, n: Self) -> Self {
        unsafe { sys::powf(self, n) }
    }

    fn sin(self) -> Self {
        unsafe { sys::sinf(self) }
    }

    fn sinh(self) -> Self {
        unsafe { sys::sinhf(self) }
    }

    fn sqrt(self) -> Self {
        unsafe { sys::sqrtf(self) }
    }

    fn tan(self) -> Self {
        unsafe { sys::tanf(self) }
    }

    fn tanh(self) -> Self {
        unsafe { sys::tanhf(self) }
    }

    fn gamma(self) -> Self {
        unsafe { sys::tgammaf(self) }
    }

    fn to_degrees(self) -> Self {
        self * 180.0 / f32::consts::PI
    }

    fn to_radians(self) -> Self {
        self * f32::consts::PI / 180.0
    }

    fn round(self) -> Self {
        (self + 0.51).floor()
    }
}

impl FloatingMathExt for f64 {
    fn acos(self) -> Self {
        unsafe { sys::acos(self) }
    }

    fn acosh(self) -> Self {
        unsafe { sys::acosh(self) }
    }

    fn asin(self) -> Self {
        unsafe { sys::asin(self) }
    }

    fn asinh(self) -> Self {
        unsafe { sys::asinh(self) }
    }

    fn atan(self) -> Self {
        unsafe { sys::atan(self) }
    }

    fn atan2(self, other: Self) -> Self {
        unsafe { sys::atan2(self, other) }
    }

    fn atanh(self) -> Self {
        unsafe { sys::atanh(self) }
    }

    fn cbrt(self) -> Self {
        unsafe { sys::cbrt(self) }
    }

    fn ceil(self) -> Self {
        unsafe { sys::ceil(self) }
    }

    fn cos(self) -> Self {
        unsafe { sys::cos(self) }
    }

    fn cosh(self) -> Self {
        unsafe { sys::cosh(self) }
    }

    fn exp(self) -> Self {
        unsafe { sys::exp(self) }
    }

    fn exp2(self) -> Self {
        unsafe { sys::exp2(self) }
    }

    fn exp_m1(self) -> Self {
        unsafe { sys::expm1(self) }
    }

    fn abs_sub(self, other: Self) -> Self {
        unsafe { sys::fdim(self, other) }
    }

    fn floor(self) -> Self {
        unsafe { sys::floor(self) }
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        unsafe { sys::fma(self, a, b) }
    }

    fn fmod(self, y: Self) -> Self {
        unsafe { sys::fmod(self, y) }
    }

    fn hypot(self, other: Self) -> Self {
        unsafe { sys::hypot(self, other) }
    }

    fn log(self) -> Self {
        unsafe { sys::log(self) }
    }

    fn log10(self) -> Self {
        unsafe { sys::log10(self) }
    }

    fn log2(self) -> Self {
        unsafe { sys::log2(self) }
    }

    fn powf(self, n: Self) -> Self {
        unsafe { sys::pow(self, n) }
    }

    fn sin(self) -> Self {
        unsafe { sys::sin(self) }
    }

    fn sinh(self) -> Self {
        unsafe { sys::sinh(self) }
    }

    fn sqrt(self) -> Self {
        unsafe { sys::sqrt(self) }
    }

    fn tan(self) -> Self {
        unsafe { sys::tan(self) }
    }

    fn tanh(self) -> Self {
        unsafe { sys::tanh(self) }
    }

    fn gamma(self) -> Self {
        unsafe { sys::tgamma(self) }
    }

    fn to_degrees(self) -> Self {
        self * 180.0 / f64::consts::PI
    }

    fn to_radians(self) -> Self {
        self * f64::consts::PI / 180.0
    }

    fn round(self) -> Self {
        (self + 0.51).floor()
    }
}
