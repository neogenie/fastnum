use core::num::FpCategory;
use num_traits::Float;

use crate::decimal::{dec::convert, Decimal, RoundingMode};

impl<const N: usize> Float for Decimal<N> {
    #[inline]
    fn nan() -> Self {
        Self::NAN
    }

    #[inline]
    fn infinity() -> Self {
        Self::INFINITY
    }

    #[inline]
    fn neg_infinity() -> Self {
        Self::NEG_INFINITY
    }

    #[inline]
    fn neg_zero() -> Self {
        Self::ZERO.neg()
    }

    #[inline]
    fn min_value() -> Self {
        Self::MIN
    }

    #[inline]
    fn min_positive_value() -> Self {
        Self::MIN_POSITIVE
    }

    #[inline]
    fn epsilon() -> Self {
        Self::EPSILON
    }

    #[inline]
    fn max_value() -> Self {
        Self::MAX
    }

    #[inline]
    fn is_nan(self) -> bool {
        Self::is_nan(&self)
    }

    #[inline]
    fn is_infinite(self) -> bool {
        Self::is_infinite(&self)
    }

    #[inline]
    fn is_finite(self) -> bool {
        Self::is_finite(&self)
    }

    #[inline]
    fn is_normal(self) -> bool {
        Self::is_normal(&self)
    }

    #[inline]
    fn is_subnormal(self) -> bool {
        Self::is_subnormal(&self)
    }

    #[inline]
    fn classify(self) -> FpCategory {
        Self::classify(&self)
    }

    #[inline]
    fn floor(self) -> Self {
        self.floor()
    }

    #[inline]
    fn ceil(self) -> Self {
        self.ceil()
    }

    #[inline]
    fn round(self) -> Self {
        self.round(0)
    }

    #[inline]
    fn trunc(self) -> Self {
        self.with_rounding_mode(RoundingMode::Down).round(0)
    }

    #[inline]
    fn fract(self) -> Self {
        self - self.trunc()
    }

    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn signum(self) -> Self {
        Self::signum(&self)
    }

    #[inline]
    fn is_sign_positive(self) -> bool {
        Self::is_sign_positive(&self)
    }

    #[inline]
    fn is_sign_negative(self) -> bool {
        Self::is_sign_negative(&self)
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }

    #[inline]
    fn recip(self) -> Self {
        self.recip()
    }

    #[inline]
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        self.pow(n)
    }

    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    #[inline]
    fn exp(self) -> Self {
        self.exp()
    }

    #[inline]
    fn exp2(self) -> Self {
        self.exp2()
    }

    #[inline]
    fn ln(self) -> Self {
        self.ln()
    }

    #[inline]
    fn log(self, base: Self) -> Self {
        self.log(base)
    }

    #[inline]
    fn log2(self) -> Self {
        self.log2()
    }

    #[inline]
    fn log10(self) -> Self {
        self.log10()
    }

    #[inline]
    fn to_degrees(self) -> Self {
        self.to_degrees()
    }

    #[inline]
    fn to_radians(self) -> Self {
        self.to_radians()
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }

    #[inline]
    fn abs_sub(self, other: Self) -> Self {
        self.abs_sub(other)
    }

    #[inline]
    fn cbrt(self) -> Self {
        self.cbrt()
    }

    #[inline]
    fn hypot(self, other: Self) -> Self {
        self.hypot(other)
    }

    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }

    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }

    #[inline]
    fn tan(self) -> Self {
        self.tan()
    }

    #[inline]
    fn asin(self) -> Self {
        self.asin()
    }

    #[inline]
    fn acos(self) -> Self {
        self.acos()
    }

    #[inline]
    fn atan(self) -> Self {
        self.atan()
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }

    #[inline]
    fn exp_m1(self) -> Self {
        self.exp_m1()
    }

    #[inline]
    fn ln_1p(self) -> Self {
        self.ln_1p()
    }

    #[inline]
    fn sinh(self) -> Self {
        self.sinh()
    }

    #[inline]
    fn cosh(self) -> Self {
        self.cosh()
    }

    #[inline]
    fn tanh(self) -> Self {
        self.tanh()
    }

    #[inline]
    fn asinh(self) -> Self {
        self.asinh()
    }

    #[inline]
    fn acosh(self) -> Self {
        self.acosh()
    }

    #[inline]
    fn atanh(self) -> Self {
        self.atanh()
    }

    #[inline]
    fn integer_decode(self) -> (u64, i16, i8) {
        convert::to_f64(self).integer_decode()
    }

    #[inline]
    fn copysign(self, sign: Self) -> Self {
        if self.is_negative() == sign.is_negative() {
            self
        } else {
            self.neg()
        }
    }
}
