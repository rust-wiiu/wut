#![no_std]

mod bindings;

use core::{f32, f64};

use bindings as C;

pub trait FloatingMathExt {
    /// Computes the arccosine of a number. Return value is in radians in the range [0, pi] or NaN if the number is outside the range [-1, 1].
    fn acos(x: Self) -> Self;

    /// Inverse hyperbolic cosine function.
    fn acosh(x: Self) -> Self;

    /// Computes the arcsine of a number. Return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
    fn asin(x: Self) -> Self;

    /// Inverse hyperbolic sine function.
    fn asinh(x: Self) -> Self;

    /// Computes the arctangent of a number. Return value is in radians in the range [-pi/2, pi/2];
    fn atan(x: Self) -> Self;

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    fn atan2(x: Self, y: Self) -> Self;

    /// Inverse hyperbolic tangent function.
    fn atanh(x: Self) -> Self;

    /// Returns the cube root of a number.
    fn cbrt(x: Self) -> Self;

    /// Returns the smallest integer greater than or equal to self.
    ///
    /// This function always returns the precise result.
    fn ceil(x: Self) -> Self;

    /// Computes the cosine of a number (in radians).
    fn cos(x: Self) -> Self;

    /// Hyperbolic cosine function.
    fn cosh(x: Self) -> Self;

    /// Returns `e^(self)`, (the exponential function).
    fn exp(x: Self) -> Self;

    /// Returns `2^(self)`.
    fn exp2(x: Self) -> Self;

    /// Returns `e^(self) - 1` in a way that is accurate even if the number is close to zero.
    fn exp_m1(x: Self) -> Self; // expm1

    /// The positive difference of two numbers.
    ///
    /// * If `self <= other`: `0.0`
    /// * Else: `self - other`
    fn abs_sub(x: Self, y: Self) -> Self;

    /// Returns the largest integer less than or equal to self.
    ///
    /// This function always returns the precise result.
    fn floor(x: Self) -> Self;

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target architecture has a dedicated `fma` CPU instruction. However, this is not always true, and will be heavily dependant on designing algorithms with specific target hardware in mind.
    ///
    /// # Precision
    ///
    /// The result of this operation is guaranteed to be the rounded infinite-precision result. It is specified by IEEE 754 as `fusedMultiplyAdd` and guaranteed not to change.
    fn mul_add(x: Self, y: Self, z: Self) -> Self; // fma

    /// Computes the floating-point remainder of `x / y`.
    ///
    /// The return value is `x - n * y`, where `n` is the integer quotient of `x / y` rounded toward zero.
    fn fmod(x: Self, y: Self) -> Self;

    /// Compute the distance between the origin and a point (`x`, `y`) on the Euclidean plane. Equivalently, compute the length of the hypotenuse of a right-angle triangle with other sides having length `x.abs()` and `y.abs()`.
    fn hypot(x: Self, y: Self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// The result might not be correctly rounded owing to implementation details; self.log2() can produce more accurate results for base 2, and self.log10() can produce more accurate results for base 10.
    fn log(x: Self) -> Self;

    /// Returns the base 10 logarithm of the number.
    fn log10(x: Self) -> Self;

    /// Returns the base 2 logarithm of the number.
    fn log2(x: Self) -> Self;

    // Raises a number to a floating point power.
    fn powf(x: Self, n: Self) -> Self; // pow

    /// Computes the sine of a number (in radians).
    fn sin(x: Self) -> Self;

    /// Hyperbolic sine function.
    fn sinh(x: Self) -> Self;

    /// Returns the square root of a number.
    ///
    /// Returns `NaN` if self is a negative number other than `-0.0`.
    fn sqrt(x: Self) -> Self;

    /// Computes the tangent of a number (in radians).
    fn tan(x: Self) -> Self;

    /// Hyperbolic tangent function.
    fn tanh(x: Self) -> Self;

    /// Gamma function.
    fn gamma(x: Self) -> Self; // tgamma

    /// Convert radians to degree.
    fn to_degrees(x: Self) -> Self;

    /// Convert degree to radians.
    fn to_radians(x: Self) -> Self;
}

impl FloatingMathExt for f32 {
    fn acos(x: Self) -> Self {
        unsafe { C::acosf(x) }
    }

    fn acosh(x: Self) -> Self {
        unsafe { C::acoshf(x) }
    }

    fn asin(x: Self) -> Self {
        unsafe { C::asinf(x) }
    }

    fn asinh(x: Self) -> Self {
        unsafe { C::asinhf(x) }
    }

    fn atan(x: Self) -> Self {
        unsafe { C::atanf(x) }
    }

    fn atan2(x: Self, y: Self) -> Self {
        unsafe { C::atan2f(x, y) }
    }

    fn atanh(x: Self) -> Self {
        unsafe { C::atanhf(x) }
    }

    fn cbrt(x: Self) -> Self {
        unsafe { C::cbrtf(x) }
    }

    fn ceil(x: Self) -> Self {
        unsafe { C::ceilf(x) }
    }

    fn cos(x: Self) -> Self {
        unsafe { C::cosf(x) }
    }

    fn cosh(x: Self) -> Self {
        unsafe { C::coshf(x) }
    }

    fn exp(x: Self) -> Self {
        unsafe { C::expf(x) }
    }

    fn exp2(x: Self) -> Self {
        unsafe { C::exp2f(x) }
    }

    fn exp_m1(x: Self) -> Self {
        unsafe { C::expm1f(x) }
    }

    fn abs_sub(x: Self, y: Self) -> Self {
        unsafe { C::fdimf(x, y) }
    }

    fn floor(x: Self) -> Self {
        unsafe { C::floorf(x) }
    }

    fn mul_add(x: Self, y: Self, z: Self) -> Self {
        unsafe { C::fmaf(x, y, z) }
    }

    fn fmod(x: Self, y: Self) -> Self {
        unsafe { C::fmodf(x, y) }
    }

    fn hypot(x: Self, y: Self) -> Self {
        unsafe { C::hypotf(x, y) }
    }

    fn log(x: Self) -> Self {
        unsafe { C::logf(x) }
    }

    fn log10(x: Self) -> Self {
        unsafe { C::log10f(x) }
    }

    fn log2(x: Self) -> Self {
        unsafe { C::log2f(x) }
    }

    fn powf(x: Self, n: Self) -> Self {
        unsafe { C::powf(x, n) }
    }

    fn sin(x: Self) -> Self {
        unsafe { C::sinf(x) }
    }

    fn sinh(x: Self) -> Self {
        unsafe { C::sinhf(x) }
    }

    fn sqrt(x: Self) -> Self {
        unsafe { C::sqrtf(x) }
    }

    fn tan(x: Self) -> Self {
        unsafe { C::tanf(x) }
    }

    fn tanh(x: Self) -> Self {
        unsafe { C::tanhf(x) }
    }

    fn gamma(x: Self) -> Self {
        unsafe { C::tgammaf(x) }
    }

    fn to_degrees(x: Self) -> Self {
        x * 180.0 / f32::consts::PI
    }

    fn to_radians(x: Self) -> Self {
        x * f32::consts::PI / 180.0
    }
}

impl FloatingMathExt for f64 {
    fn acos(x: Self) -> Self {
        unsafe { C::acos(x) }
    }

    fn acosh(x: Self) -> Self {
        unsafe { C::acosh(x) }
    }

    fn asin(x: Self) -> Self {
        unsafe { C::asin(x) }
    }

    fn asinh(x: Self) -> Self {
        unsafe { C::asinh(x) }
    }

    fn atan(x: Self) -> Self {
        unsafe { C::atan(x) }
    }

    fn atan2(x: Self, y: Self) -> Self {
        unsafe { C::atan2(x, y) }
    }

    fn atanh(x: Self) -> Self {
        unsafe { C::atanh(x) }
    }

    fn cbrt(x: Self) -> Self {
        unsafe { C::cbrt(x) }
    }

    fn ceil(x: Self) -> Self {
        unsafe { C::ceil(x) }
    }

    fn cos(x: Self) -> Self {
        unsafe { C::cos(x) }
    }

    fn cosh(x: Self) -> Self {
        unsafe { C::cosh(x) }
    }

    fn exp(x: Self) -> Self {
        unsafe { C::exp(x) }
    }

    fn exp2(x: Self) -> Self {
        unsafe { C::exp2(x) }
    }

    fn exp_m1(x: Self) -> Self {
        unsafe { C::expm1(x) }
    }

    fn abs_sub(x: Self, y: Self) -> Self {
        unsafe { C::fdim(x, y) }
    }

    fn floor(x: Self) -> Self {
        unsafe { C::floor(x) }
    }

    fn mul_add(x: Self, y: Self, z: Self) -> Self {
        unsafe { C::fma(x, y, z) }
    }

    fn fmod(x: Self, y: Self) -> Self {
        unsafe { C::fmod(x, y) }
    }

    fn hypot(x: Self, y: Self) -> Self {
        unsafe { C::hypot(x, y) }
    }

    fn log(x: Self) -> Self {
        unsafe { C::log(x) }
    }

    fn log10(x: Self) -> Self {
        unsafe { C::log10(x) }
    }

    fn log2(x: Self) -> Self {
        unsafe { C::log2(x) }
    }

    fn powf(x: Self, n: Self) -> Self {
        unsafe { C::pow(x, n) }
    }

    fn sin(x: Self) -> Self {
        unsafe { C::sin(x) }
    }

    fn sinh(x: Self) -> Self {
        unsafe { C::sinh(x) }
    }

    fn sqrt(x: Self) -> Self {
        unsafe { C::sqrt(x) }
    }

    fn tan(x: Self) -> Self {
        unsafe { C::tan(x) }
    }

    fn tanh(x: Self) -> Self {
        unsafe { C::tanh(x) }
    }

    fn gamma(x: Self) -> Self {
        unsafe { C::tgamma(x) }
    }

    fn to_degrees(x: Self) -> Self {
        x * 180.0 / f64::consts::PI
    }

    fn to_radians(x: Self) -> Self {
        x * f64::consts::PI / 180.0
    }
}
