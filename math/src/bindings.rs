/* automatically generated by rust-bindgen 0.71.1 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
unsafe extern "C" { pub fn ceil(arg1: f64) -> f64;}

pub const _M_LN2: f64 = 0.6931471805599453;
pub const FP_NAN: u32 = 0;
pub const FP_INFINITE: u32 = 1;
pub const FP_ZERO: u32 = 2;
pub const FP_SUBNORMAL: u32 = 3;
pub const FP_NORMAL: u32 = 4;
pub const FP_ILOGB0: i32 = -2147483647;
pub const FP_ILOGBNAN: u32 = 2147483647;
pub const MATH_ERRNO: u32 = 1;
pub const MATH_ERREXCEPT: u32 = 2;
pub const _MATH_ERRHANDLING_ERRNO: u32 = 1;
pub const _MATH_ERRHANDLING_ERREXCEPT: u32 = 0;
pub const math_errhandling: u32 = 1;
pub const M_E: f64 = 2.718281828459045;
pub const M_LOG2E: f64 = 1.4426950408889634;
pub const M_LOG10E: f64 = 0.4342944819032518;
pub const M_LN2: f64 = 0.6931471805599453;
pub const M_LN10: f64 = 2.302585092994046;
pub const M_PI: f64 = 3.141592653589793;
pub const M_PI_2: f64 = 1.5707963267948966;
pub const M_PI_4: f64 = 0.7853981633974483;
pub const M_1_PI: f64 = 0.3183098861837907;
pub const M_2_PI: f64 = 0.6366197723675814;
pub const M_2_SQRTPI: f64 = 1.1283791670955126;
pub const M_SQRT2: f64 = 1.4142135623730951;
pub const M_SQRT1_2: f64 = 0.7071067811865476;
pub const M_TWOPI: f64 = 6.283185307179586;
pub const M_SQRTPI: f64 = 1.772453850905516;
pub const M_SQRT3: f64 = 1.7320508075688772;
pub const M_IVLN10: f64 = 0.4342944819032518;
pub const M_LOG2_E: f64 = 0.6931471805599453;
pub type float_t = f32;
pub type double_t = f64;
unsafe extern "C" {
    pub fn atan(arg1: f64) -> f64;
    pub fn cos(arg1: f64) -> f64;
    pub fn sin(arg1: f64) -> f64;
    pub fn tan(arg1: f64) -> f64;
    pub fn tanh(arg1: f64) -> f64;
    pub fn frexp(arg1: f64, arg2: *mut ::core::ffi::c_int) -> f64;
    pub fn modf(arg1: f64, arg2: *mut f64) -> f64;
    pub fn fabs(arg1: f64) -> f64;
    pub fn floor(arg1: f64) -> f64;
    pub fn acos(arg1: f64) -> f64;
    pub fn asin(arg1: f64) -> f64;
    pub fn atan2(arg1: f64, arg2: f64) -> f64;
    pub fn cosh(arg1: f64) -> f64;
    pub fn sinh(arg1: f64) -> f64;
    pub fn exp(arg1: f64) -> f64;
    pub fn ldexp(arg1: f64, arg2: ::core::ffi::c_int) -> f64;
    pub fn log(arg1: f64) -> f64;
    pub fn log10(arg1: f64) -> f64;
    pub fn pow(arg1: f64, arg2: f64) -> f64;
    pub fn sqrt(arg1: f64) -> f64;
    pub fn fmod(arg1: f64, arg2: f64) -> f64;
    pub fn finite(arg1: f64) -> ::core::ffi::c_int;
    pub fn finitef(arg1: f32) -> ::core::ffi::c_int;
    pub fn isinff(arg1: f32) -> ::core::ffi::c_int;
    pub fn isnanf(arg1: f32) -> ::core::ffi::c_int;
    pub fn isinf(arg1: f64) -> ::core::ffi::c_int;
    pub fn isnan(arg1: f64) -> ::core::ffi::c_int;
    pub fn __isinff(arg1: f32) -> ::core::ffi::c_int;
    pub fn __isinfd(arg1: f64) -> ::core::ffi::c_int;
    pub fn __isnanf(arg1: f32) -> ::core::ffi::c_int;
    pub fn __isnand(arg1: f64) -> ::core::ffi::c_int;
    pub fn __fpclassifyf(arg1: f32) -> ::core::ffi::c_int;
    pub fn __fpclassifyd(arg1: f64) -> ::core::ffi::c_int;
    pub fn __signbitf(arg1: f32) -> ::core::ffi::c_int;
    pub fn __signbitd(arg1: f64) -> ::core::ffi::c_int;
    pub fn infinity() -> f64;
    pub fn nan(arg1: *const ::core::ffi::c_char) -> f64;
    pub fn copysign(arg1: f64, arg2: f64) -> f64;
    pub fn logb(arg1: f64) -> f64;
    pub fn ilogb(arg1: f64) -> ::core::ffi::c_int;
    pub fn asinh(arg1: f64) -> f64;
    pub fn cbrt(arg1: f64) -> f64;
    pub fn nextafter(arg1: f64, arg2: f64) -> f64;
    pub fn rint(arg1: f64) -> f64;
    pub fn scalbn(arg1: f64, arg2: ::core::ffi::c_int) -> f64;
    pub fn exp2(arg1: f64) -> f64;
    pub fn scalbln(arg1: f64, arg2: ::core::ffi::c_long) -> f64;
    pub fn tgamma(arg1: f64) -> f64;
    pub fn nearbyint(arg1: f64) -> f64;
    pub fn lrint(arg1: f64) -> ::core::ffi::c_long;
    pub fn llrint(arg1: f64) -> ::core::ffi::c_longlong;
    pub fn round(arg1: f64) -> f64;
    pub fn lround(arg1: f64) -> ::core::ffi::c_long;
    pub fn llround(arg1: f64) -> ::core::ffi::c_longlong;
    pub fn trunc(arg1: f64) -> f64;
    pub fn remquo(arg1: f64, arg2: f64, arg3: *mut ::core::ffi::c_int) -> f64;
    pub fn fdim(arg1: f64, arg2: f64) -> f64;
    pub fn fmax(arg1: f64, arg2: f64) -> f64;
    pub fn fmin(arg1: f64, arg2: f64) -> f64;
    pub fn fma(arg1: f64, arg2: f64, arg3: f64) -> f64;
    pub fn log1p(arg1: f64) -> f64;
    pub fn expm1(arg1: f64) -> f64;
    pub fn acosh(arg1: f64) -> f64;
    pub fn atanh(arg1: f64) -> f64;
    pub fn remainder(arg1: f64, arg2: f64) -> f64;
    pub fn gamma(arg1: f64) -> f64;
    pub fn lgamma(arg1: f64) -> f64;
    pub fn erf(arg1: f64) -> f64;
    pub fn erfc(arg1: f64) -> f64;
    pub fn log2(arg1: f64) -> f64;
    pub fn hypot(arg1: f64, arg2: f64) -> f64;
    pub fn atanf(arg1: f32) -> f32;
    pub fn cosf(arg1: f32) -> f32;
    pub fn sinf(arg1: f32) -> f32;
    pub fn tanf(arg1: f32) -> f32;
    pub fn tanhf(arg1: f32) -> f32;
    pub fn frexpf(arg1: f32, arg2: *mut ::core::ffi::c_int) -> f32;
    pub fn modff(arg1: f32, arg2: *mut f32) -> f32;
    pub fn ceilf(arg1: f32) -> f32;
    pub fn fabsf(arg1: f32) -> f32;
    pub fn floorf(arg1: f32) -> f32;
    pub fn acosf(arg1: f32) -> f32;
    pub fn asinf(arg1: f32) -> f32;
    pub fn atan2f(arg1: f32, arg2: f32) -> f32;
    pub fn coshf(arg1: f32) -> f32;
    pub fn sinhf(arg1: f32) -> f32;
    pub fn expf(arg1: f32) -> f32;
    pub fn ldexpf(arg1: f32, arg2: ::core::ffi::c_int) -> f32;
    pub fn logf(arg1: f32) -> f32;
    pub fn log10f(arg1: f32) -> f32;
    pub fn powf(arg1: f32, arg2: f32) -> f32;
    pub fn sqrtf(arg1: f32) -> f32;
    pub fn fmodf(arg1: f32, arg2: f32) -> f32;
    pub fn exp2f(arg1: f32) -> f32;
    pub fn scalblnf(arg1: f32, arg2: ::core::ffi::c_long) -> f32;
    pub fn tgammaf(arg1: f32) -> f32;
    pub fn nearbyintf(arg1: f32) -> f32;
    pub fn lrintf(arg1: f32) -> ::core::ffi::c_long;
    pub fn llrintf(arg1: f32) -> ::core::ffi::c_longlong;
    pub fn roundf(arg1: f32) -> f32;
    pub fn lroundf(arg1: f32) -> ::core::ffi::c_long;
    pub fn llroundf(arg1: f32) -> ::core::ffi::c_longlong;
    pub fn truncf(arg1: f32) -> f32;
    pub fn remquof(arg1: f32, arg2: f32, arg3: *mut ::core::ffi::c_int) -> f32;
    pub fn fdimf(arg1: f32, arg2: f32) -> f32;
    pub fn fmaxf(arg1: f32, arg2: f32) -> f32;
    pub fn fminf(arg1: f32, arg2: f32) -> f32;
    pub fn fmaf(arg1: f32, arg2: f32, arg3: f32) -> f32;
    pub fn infinityf() -> f32;
    pub fn nanf(arg1: *const ::core::ffi::c_char) -> f32;
    pub fn copysignf(arg1: f32, arg2: f32) -> f32;
    pub fn logbf(arg1: f32) -> f32;
    pub fn ilogbf(arg1: f32) -> ::core::ffi::c_int;
    pub fn asinhf(arg1: f32) -> f32;
    pub fn cbrtf(arg1: f32) -> f32;
    pub fn nextafterf(arg1: f32, arg2: f32) -> f32;
    pub fn rintf(arg1: f32) -> f32;
    pub fn scalbnf(arg1: f32, arg2: ::core::ffi::c_int) -> f32;
    pub fn log1pf(arg1: f32) -> f32;
    pub fn expm1f(arg1: f32) -> f32;
    pub fn acoshf(arg1: f32) -> f32;
    pub fn atanhf(arg1: f32) -> f32;
    pub fn remainderf(arg1: f32, arg2: f32) -> f32;
    pub fn gammaf(arg1: f32) -> f32;
    pub fn lgammaf(arg1: f32) -> f32;
    pub fn erff(arg1: f32) -> f32;
    pub fn erfcf(arg1: f32) -> f32;
    pub fn log2f(arg1: f32) -> f32;
    pub fn hypotf(arg1: f32, arg2: f32) -> f32;
    pub fn drem(arg1: f64, arg2: f64) -> f64;
    pub fn dremf(arg1: f32, arg2: f32) -> f32;
    pub fn gamma_r(arg1: f64, arg2: *mut ::core::ffi::c_int) -> f64;
    pub fn lgamma_r(arg1: f64, arg2: *mut ::core::ffi::c_int) -> f64;
    pub fn gammaf_r(arg1: f32, arg2: *mut ::core::ffi::c_int) -> f32;
    pub fn lgammaf_r(arg1: f32, arg2: *mut ::core::ffi::c_int) -> f32;
    pub fn y0(arg1: f64) -> f64;
    pub fn y1(arg1: f64) -> f64;
    pub fn yn(arg1: ::core::ffi::c_int, arg2: f64) -> f64;
    pub fn j0(arg1: f64) -> f64;
    pub fn j1(arg1: f64) -> f64;
    pub fn jn(arg1: ::core::ffi::c_int, arg2: f64) -> f64;
    pub fn y0f(arg1: f32) -> f32;
    pub fn y1f(arg1: f32) -> f32;
    pub fn ynf(arg1: ::core::ffi::c_int, arg2: f32) -> f32;
    pub fn j0f(arg1: f32) -> f32;
    pub fn j1f(arg1: f32) -> f32;
    pub fn jnf(arg1: ::core::ffi::c_int, arg2: f32) -> f32;
    pub fn __signgam() -> *mut ::core::ffi::c_int;
}
