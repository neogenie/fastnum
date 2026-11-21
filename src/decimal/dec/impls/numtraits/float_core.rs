use core::num::FpCategory;
use num_traits::float::FloatCore;

use crate::decimal::{dec::convert, Decimal, RoundingMode};

impl<const N: usize> FloatCore for Decimal<N> {
    #[inline]
    fn infinity() -> Self {
        Self::INFINITY
    }

    #[inline]
    fn neg_infinity() -> Self {
        Self::NEG_INFINITY
    }

    #[inline]
    fn nan() -> Self {
        Self::NAN
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
        self.with_rounding_mode(RoundingMode::Floor).round(0)
    }

    #[inline]
    fn ceil(self) -> Self {
        self.with_rounding_mode(RoundingMode::Ceiling).round(0)
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
        Self::fract(self)
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
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }

    #[inline]
    fn recip(self) -> Self {
        self.recip()
    }

    #[inline]
    fn powi(self, exp: i32) -> Self {
        self.powi(exp)
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
    fn integer_decode(self) -> (u64, i16, i8) {
        convert::to_f64(self).integer_decode()
    }
}
