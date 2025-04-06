#![no_std]

mod bindings;

use core::{f32, f64};

use bindings as C;

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
        unsafe { C::acosf(self) }
    }

    fn acosh(self) -> Self {
        unsafe { C::acoshf(self) }
    }

    fn asin(self) -> Self {
        unsafe { C::asinf(self) }
    }

    fn asinh(self) -> Self {
        unsafe { C::asinhf(self) }
    }

    fn atan(self) -> Self {
        unsafe { C::atanf(self) }
    }

    fn atan2(self, other: Self) -> Self {
        unsafe { C::atan2f(self, other) }
    }

    fn atanh(self) -> Self {
        unsafe { C::atanhf(self) }
    }

    fn cbrt(self) -> Self {
        unsafe { C::cbrtf(self) }
    }

    fn ceil(self) -> Self {
        unsafe { C::ceilf(self) }
    }

    fn cos(self) -> Self {
        unsafe { C::cosf(self) }
    }

    fn cosh(self) -> Self {
        unsafe { C::coshf(self) }
    }

    fn exp(self) -> Self {
        unsafe { C::expf(self) }
    }

    fn exp2(self) -> Self {
        unsafe { C::exp2f(self) }
    }

    fn exp_m1(self) -> Self {
        unsafe { C::expm1f(self) }
    }

    fn abs_sub(self, other: Self) -> Self {
        unsafe { C::fdimf(self, other) }
    }

    fn floor(self) -> Self {
        unsafe { C::floorf(self) }
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        unsafe { C::fmaf(self, a, b) }
    }

    fn fmod(self, other: Self) -> Self {
        unsafe { C::fmodf(self, other) }
    }

    fn hypot(self, other: Self) -> Self {
        unsafe { C::hypotf(self, other) }
    }

    fn log(self) -> Self {
        unsafe { C::logf(self) }
    }

    fn log10(self) -> Self {
        unsafe { C::log10f(self) }
    }

    fn log2(self) -> Self {
        unsafe { C::log2f(self) }
    }

    fn powf(self, n: Self) -> Self {
        unsafe { C::powf(self, n) }
    }

    fn sin(self) -> Self {
        unsafe { C::sinf(self) }
    }

    fn sinh(self) -> Self {
        unsafe { C::sinhf(self) }
    }

    fn sqrt(self) -> Self {
        unsafe { C::sqrtf(self) }
    }

    fn tan(self) -> Self {
        unsafe { C::tanf(self) }
    }

    fn tanh(self) -> Self {
        unsafe { C::tanhf(self) }
    }

    fn gamma(self) -> Self {
        unsafe { C::tgammaf(self) }
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
        unsafe { C::acos(self) }
    }

    fn acosh(self) -> Self {
        unsafe { C::acosh(self) }
    }

    fn asin(self) -> Self {
        unsafe { C::asin(self) }
    }

    fn asinh(self) -> Self {
        unsafe { C::asinh(self) }
    }

    fn atan(self) -> Self {
        unsafe { C::atan(self) }
    }

    fn atan2(self, other: Self) -> Self {
        unsafe { C::atan2(self, other) }
    }

    fn atanh(self) -> Self {
        unsafe { C::atanh(self) }
    }

    fn cbrt(self) -> Self {
        unsafe { C::cbrt(self) }
    }

    fn ceil(self) -> Self {
        unsafe { C::ceil(self) }
    }

    fn cos(self) -> Self {
        unsafe { C::cos(self) }
    }

    fn cosh(self) -> Self {
        unsafe { C::cosh(self) }
    }

    fn exp(self) -> Self {
        unsafe { C::exp(self) }
    }

    fn exp2(self) -> Self {
        unsafe { C::exp2(self) }
    }

    fn exp_m1(self) -> Self {
        unsafe { C::expm1(self) }
    }

    fn abs_sub(self, other: Self) -> Self {
        unsafe { C::fdim(self, other) }
    }

    fn floor(self) -> Self {
        unsafe { C::floor(self) }
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        unsafe { C::fma(self, a, b) }
    }

    fn fmod(self, y: Self) -> Self {
        unsafe { C::fmod(self, y) }
    }

    fn hypot(self, other: Self) -> Self {
        unsafe { C::hypot(self, other) }
    }

    fn log(self) -> Self {
        unsafe { C::log(self) }
    }

    fn log10(self) -> Self {
        unsafe { C::log10(self) }
    }

    fn log2(self) -> Self {
        unsafe { C::log2(self) }
    }

    fn powf(self, n: Self) -> Self {
        unsafe { C::pow(self, n) }
    }

    fn sin(self) -> Self {
        unsafe { C::sin(self) }
    }

    fn sinh(self) -> Self {
        unsafe { C::sinh(self) }
    }

    fn sqrt(self) -> Self {
        unsafe { C::sqrt(self) }
    }

    fn tan(self) -> Self {
        unsafe { C::tan(self) }
    }

    fn tanh(self) -> Self {
        unsafe { C::tanh(self) }
    }

    fn gamma(self) -> Self {
        unsafe { C::tgamma(self) }
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
