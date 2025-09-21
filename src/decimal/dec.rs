//! # Signed Decimal

mod cmp;
mod construct;
mod consts;
mod control_block;
mod convert;
mod extra_precision;
mod extras;
mod format;
mod impls;
mod intrinsics;
mod parse;
mod resize;
mod round;
mod scale;
mod truncate;

pub(crate) mod math;
pub(crate) mod utils;

pub(crate) use control_block::ControlBlock;
pub(crate) use extra_precision::ExtraPrecision;

use core::{cmp::Ordering, fmt, num::FpCategory, panic};

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

use crate::{
    bint::UInt,
    decimal,
    decimal::{
        dec::{consts::consts_impl, intrinsics::Intrinsics, math::consts::Consts, round::round},
        doc,
        signals::Signals,
        Context, DecimalError, ParseError, RoundingMode, Sign, UnsignedDecimal,
    },
};

/// # Decimal
///
/// Generic signed N-bits decimal number.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Decimal<const N: usize> {
    /// An N-bit unsigned integer coefficient. Represent significant decimal
    /// digits.
    digits: UInt<N>,

    /// Control block
    #[doc(hidden)]
    cb: ControlBlock,
}

consts_impl!();

impl<const N: usize> Decimal<N> {
    /// Creates and initializes decimal from parts.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// assert_eq!(D256::from_parts(u256!(12345), -4, Sign::Minus, Context::default()), dec256!(-1.2345));
    /// ```
    #[track_caller]
    #[must_use]
    #[inline]
    pub const fn from_parts(digits: UInt<N>, exp: i32, sign: Sign, ctx: Context) -> Self {
        construct::construct(
            digits,
            exp,
            sign,
            Signals::empty(),
            ctx,
            ExtraPrecision::new(),
        )
        .check()
    }

    /// Creates and initializes decimal from string.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// assert_eq!(D256::from_str("-1.2345", Context::default()), Ok(dec256!(-1.2345)));
    /// ```
    #[track_caller]
    #[inline]
    pub const fn from_str(s: &str, ctx: Context) -> Result<Self, ParseError> {
        parse::from_slice(s.as_bytes(), ctx)
    }

    /// Parse decimal from string.
    ///
    /// # Panics
    ///
    /// This function will panic if `Decimal<N>` can't be constructed
    /// from a given string.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// assert_eq!(D256::parse_str("1.2345", Context::default()), dec256!(1.2345));
    /// ```
    ///
    ///
    /// ```should_panic
    /// use fastnum::{*, decimal::*};
    ///
    /// let _ = D256::parse_str("Hello", Context::default());
    /// ```
    #[track_caller]
    #[must_use]
    #[inline]
    pub const fn parse_str(s: &str, ctx: Context) -> Self {
        match Self::from_str(s, ctx) {
            Ok(n) => n,
            Err(e) => {
                panic!("{}", e.description())
            }
        }
    }

    /// Returns the internal big integer, representing the
    /// [_Coefficient_](crate#representation) of a given `Decimal`, including
    /// significant trailing zeros.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{dec256, u256};
    ///
    /// let a = dec256!(-123.45);
    /// assert_eq!(a.digits(), u256!(12345));
    ///
    /// let b = dec256!(-1.0);
    /// assert_eq!(b.digits(), u256!(10));
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn digits(&self) -> UInt<N> {
        self.digits
    }

    /// Return the count of digits in the non-scaled integer representation
    #[must_use]
    #[inline(always)]
    pub const fn digits_count(&self) -> usize {
        self.digits.decimal_digits() as usize
    }

    /// Return the scale of the `Decimal`, the total number of
    /// digits to the right of the decimal point (including insignificant
    /// leading zeros).
    ///
    /// # Examples:
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// let a = dec256!(12345);  // No fractional part
    /// let b = dec256!(123.45);  // Fractional part
    /// let c = dec256!(0.0000012345);  // Completely fractional part
    /// let d = dec256!(500000000);  // No fractional part
    /// let e = dec256!(5e9);  // Negative-fractional part
    ///
    /// assert_eq!(a.fractional_digits_count(), 0);
    /// assert_eq!(b.fractional_digits_count(), 2);
    /// assert_eq!(c.fractional_digits_count(), 10);
    /// assert_eq!(d.fractional_digits_count(), 0);
    /// assert_eq!(e.fractional_digits_count(), -9);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn fractional_digits_count(&self) -> i16 {
        self.cb.get_scale()
    }

    /// Return the sign of the `Decimal` as [Sign].
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{decimal::Sign, dec256};
    ///
    /// assert_eq!(dec256!(-1.0).sign(), Sign::Minus);
    /// assert_eq!(dec256!(0.0).sign(),  Sign::Plus);
    /// assert_eq!(dec256!(+1.0).sign(),  Sign::Plus);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn sign(&self) -> Sign {
        self.cb.get_sign()
    }

    /// Returns `true` if the given decimal number is the result of division by
    /// zero and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
    /// let res = dec256!(1.0).with_ctx(ctx) / dec256!(0).with_ctx(ctx);
    ///
    /// assert!(res.is_op_div_by_zero());
    /// ```
    ///
    /// More about [`OP_DIV_BY_ZERO`](Signals::OP_DIV_BY_ZERO) signal.
    #[must_use]
    #[inline(always)]
    pub const fn is_op_div_by_zero(&self) -> bool {
        self.cb.is_signals_raised(Signals::OP_DIV_BY_ZERO)
    }

    /// Return `true` if the argument has [Signals::OP_OVERFLOW] signal flag,
    /// and `false` otherwise.
    #[must_use]
    #[inline(always)]
    pub const fn is_op_overflow(&self) -> bool {
        self.cb.is_signals_raised(Signals::OP_OVERFLOW)
    }

    /// Return `true` if the argument has [Signals::OP_UNDERFLOW] signal flag,
    /// and `false` otherwise.
    #[must_use]
    #[inline(always)]
    pub const fn is_op_underflow(&self) -> bool {
        self.cb.is_signals_raised(Signals::OP_UNDERFLOW)
    }

    /// Return `true` if the argument has [Signals::OP_INVALID] signal flag, and
    /// `false` otherwise.
    #[must_use]
    #[inline(always)]
    pub const fn is_op_invalid(&self) -> bool {
        self.cb.is_signals_raised(Signals::OP_INVALID)
    }

    /// Return `true` if the argument has [Signals::OP_SUBNORMAL] signal flag,
    /// and `false` otherwise.
    #[must_use]
    #[inline(always)]
    pub const fn is_op_subnormal(&self) -> bool {
        self.cb.is_signals_raised(Signals::OP_SUBNORMAL)
    }

    /// Return `true` if the argument has [Signals::OP_INEXACT] signal flag, and
    /// `false` otherwise.
    #[must_use]
    #[inline(always)]
    pub const fn is_op_inexact(&self) -> bool {
        self.cb.is_signals_raised(Signals::OP_INEXACT)
    }

    /// Return `true` if the argument has [Signals::OP_ROUNDED] signal flag, and
    /// `false` otherwise.
    #[must_use]
    #[inline(always)]
    pub const fn is_op_rounded(&self) -> bool {
        self.cb.is_signals_raised(Signals::OP_ROUNDED)
    }

    /// Return `true` if the argument has [Signals::OP_CLAMPED] signal flag, and
    /// `false` otherwise.
    #[must_use]
    #[inline(always)]
    pub const fn is_op_clamped(&self) -> bool {
        self.cb.is_signals_raised(Signals::OP_CLAMPED)
    }

    /// Return `true` if the argument has no signal flags, and `false`
    /// otherwise.
    #[must_use]
    #[inline(always)]
    pub const fn is_op_ok(&self) -> bool {
        self.cb.is_op_ok()
    }

    /// Return the [`signaling block`](Signals) of given decimal.
    #[must_use]
    #[inline(always)]
    pub const fn op_signals(&self) -> Signals {
        self.signals()
    }

    /// Return the decimal category of the number.
    /// If only one property is going to be tested, it is generally faster to
    /// use the specific predicate instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::num::FpCategory;
    /// use fastnum::{dec256, D256};
    ///
    /// let num = dec256!(12.4);
    /// let inf = D256::INFINITY;
    ///
    /// assert_eq!(num.classify(), FpCategory::Normal);
    /// assert_eq!(inf.classify(), FpCategory::Infinite);
    /// ```
    #[must_use]
    #[inline]
    pub const fn classify(&self) -> FpCategory {
        if self.cb.is_nan() {
            FpCategory::Nan
        } else if self.cb.is_infinity() {
            FpCategory::Infinite
        } else if self.digits.is_zero() {
            FpCategory::Zero
        } else if self.is_subnormal() {
            FpCategory::Subnormal
        } else {
            FpCategory::Normal
        }
    }

    /// Return `true` if the number is neither [zero], [`±Infinity`],
    /// [subnormal], or [`NaN`] and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let num = dec256!(12.4);
    /// let subnormal = dec256!(1E-30000) / dec256!(1E2768);
    /// let inf = D256::INFINITY;
    /// let nan = D256::NAN;
    /// let zero = D256::ZERO;
    ///
    /// assert!(num.is_normal());
    ///
    /// assert!(!zero.is_normal());
    /// assert!(!nan.is_normal());
    /// assert!(!nan.is_normal());
    /// assert!(!subnormal.is_normal());
    /// ```
    ///
    /// [subnormal]: crate#normal-numbers-subnormal-numbers-and-underflow
    /// [zero]: crate#signed-zero
    /// [`±Infinity`]: crate#special-values
    /// [`NaN`]: crate#special-values
    #[must_use]
    #[inline]
    pub const fn is_normal(&self) -> bool {
        matches!(self.classify(), FpCategory::Normal)
    }

    /// Return `true` if the number is [subnormal] and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let num = dec256!(12.4);
    /// let subnormal = dec256!(1E-30000) / dec256!(1E2768);
    /// let inf = D256::INFINITY;
    /// let nan = D256::NAN;
    /// let zero = D256::ZERO;
    ///
    /// assert!(subnormal.is_subnormal());
    ///
    /// assert!(!num.is_subnormal());
    /// assert!(!zero.is_subnormal());
    /// assert!(!nan.is_subnormal());
    /// assert!(!nan.is_subnormal());
    /// ```
    ///
    /// [subnormal]: crate#normal-numbers-subnormal-numbers-and-underflow
    #[must_use]
    #[inline(always)]
    pub const fn is_subnormal(&self) -> bool {
        self.is_op_subnormal()
    }

    /// Return `true` if this number is neither [`±Infinity`] nor [`NaN`] and
    /// `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{D256, dec256};
    ///
    /// let d = dec256!(7.0);
    /// let inf = D256::INFINITY;
    /// let neg_inf = D256::NEG_INFINITY;
    /// let nan = D256::NAN;
    ///
    /// assert!(d.is_finite());
    ///
    /// assert!(!nan.is_finite());
    /// assert!(!inf.is_finite());
    /// assert!(!neg_inf.is_finite());
    /// ```
    ///
    /// [`±Infinity`]: crate#special-values
    /// [`NaN`]: crate#special-values
    #[must_use]
    #[inline(always)]
    pub const fn is_finite(&self) -> bool {
        !self.cb.is_special()
    }

    /// Return `true` if this value is positive or negative [`Infinity`] and
    /// `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{D256, dec256};
    ///
    /// let d = dec256!(7.0);
    /// let inf = D256::INFINITY;
    /// let neg_inf = D256::NEG_INFINITY;
    /// let nan = D256::NAN;
    ///
    /// assert!(inf.is_infinite());
    /// assert!(neg_inf.is_infinite());
    ///
    /// assert!(!d.is_infinite());
    /// assert!(!nan.is_infinite());
    /// ```
    ///
    /// [`Infinity`]: crate#special-values
    #[must_use]
    #[inline(always)]
    pub const fn is_infinite(&self) -> bool {
        self.cb.is_infinity()
    }

    /// Return `true` if this value is [`NaN`] and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{D256, dec256};
    ///
    /// let nan = D256::NAN;
    /// let d = dec256!(7.0);
    ///
    /// assert!(nan.is_nan());
    /// assert!(!d.is_nan());
    /// ```
    ///
    /// [`NaN`]: crate#special-values
    #[must_use]
    #[inline(always)]
    pub const fn is_nan(&self) -> bool {
        self.cb.is_nan()
    }

    /// Return `true` if this value is positive, including [`+0.0`],
    /// [`+Infinity`] and [`NaN`], and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{D256, dec256};
    ///
    /// let d = dec256!(7.0);
    /// let neg_zero = dec256!(-0.0);
    /// let neg_d = dec256!(-7.0);
    ///
    /// assert!(d.is_sign_positive());
    /// assert!(D256::ZERO.is_sign_positive());
    /// assert!(D256::INFINITY.is_sign_positive());
    /// assert!(D256::NAN.is_sign_positive());
    ///
    /// assert!(!neg_d.is_sign_positive());
    /// assert!(!neg_zero.is_sign_positive());
    /// assert!(!D256::NEG_INFINITY.is_sign_positive());
    /// ```
    ///
    /// [`+0.0`]: crate#signed-zero
    /// [`+Infinity`]: crate#special-values
    /// [`NaN`]: crate#special-values
    #[must_use]
    #[inline(always)]
    pub const fn is_sign_positive(&self) -> bool {
        !self.cb.is_negative()
    }

    /// Return `true` if this value is negative, including [`-0.0`] and
    /// [`-Infinity`] and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{D256, dec256};
    ///
    /// let d = dec256!(7.0);
    /// let neg_zero = dec256!(-0.0);
    /// let neg_d = dec256!(-7.0);
    ///
    /// assert!(neg_d.is_sign_negative());
    /// assert!(neg_zero.is_sign_negative());
    /// assert!(D256::NEG_INFINITY.is_sign_negative());
    ///
    /// assert!(!d.is_sign_negative());
    /// assert!(!D256::ZERO.is_sign_negative());
    /// assert!(!D256::INFINITY.is_sign_negative());
    /// assert!(!D256::NAN.is_sign_negative());
    /// ```
    ///
    /// [`-0.0`]: crate#signed-zero
    /// [`-Infinity`]: crate#special-values
    /// [`NaN`]: crate#special-values
    #[must_use]
    #[inline(always)]
    pub const fn is_sign_negative(&self) -> bool {
        self.cb.is_negative()
    }

    /// Return `true` if the referenced decimal is [`±0.0`] and `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = dec256!(0);
    /// assert!(a.is_zero());
    ///
    /// let b = dec256!(0.0);
    /// assert!(b.is_zero());
    ///
    /// let c = dec256!(-0.00);
    /// assert!(c.is_zero());
    ///
    /// let d = dec256!(-0.1);
    /// assert!(!d.is_zero());
    /// ```
    ///
    /// [`±0.0`]: crate#signed-zero
    #[must_use]
    #[inline(always)]
    pub const fn is_zero(&self) -> bool {
        self.digits.is_zero() && !self.cb.is_special() && !self.cb.has_extra_precision()
    }

    /// Return `true` if the referenced decimal is strictly `1` and `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = dec256!(1);
    /// assert!(a.is_one());
    ///
    /// let b = dec256!(10e-1);
    /// assert!(!b.is_one());
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn is_one(&self) -> bool {
        self.digits.is_one()
            && self.cb.get_scale() == 0
            && !self.cb.is_negative()
            && !self.cb.is_special()
            && !self.cb.has_extra_precision()
    }

    /// Return `true` if this value is positive, including [`+0.0`],
    /// [`+Infinity`] and [`NaN`], and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let d = dec256!(7.0);
    /// let neg_zero = dec256!(-0.0);
    /// let neg_d = dec256!(-7.0);
    ///
    /// assert!(d.is_positive());
    /// assert!(D256::ZERO.is_positive());
    /// assert!(D256::INFINITY.is_positive());
    /// assert!(D256::NAN.is_positive());
    ///
    /// assert!(!neg_d.is_positive());
    /// assert!(!neg_zero.is_positive());
    /// assert!(!D256::NEG_INFINITY.is_positive());
    /// ```
    ///
    /// [`+0.0`]: crate#signed-zero
    /// [`+Infinity`]: crate#special-values
    /// [`NaN`]: crate#special-values
    #[must_use]
    #[inline(always)]
    pub const fn is_positive(&self) -> bool {
        !self.cb.is_negative()
    }

    /// Return `true` if this value is negative, including [`-0.0`] and
    /// [`-Infinity`] and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let d = dec256!(7.0);
    /// let neg_zero = dec256!(-0.0);
    /// let neg_d = dec256!(-7.0);
    ///
    /// assert!(neg_d.is_negative());
    /// assert!(neg_zero.is_negative());
    /// assert!(D256::NEG_INFINITY.is_negative());
    ///
    /// assert!(!d.is_negative());
    /// assert!(!D256::ZERO.is_negative());
    /// assert!(!D256::INFINITY.is_negative());
    /// assert!(!D256::NAN.is_negative());
    /// ```
    ///
    /// [`-0.0`]: crate#signed-zero
    /// [`-Infinity`]: crate#special-values
    /// [`NaN`]: crate#special-values
    #[must_use]
    #[inline(always)]
    pub const fn is_negative(&self) -> bool {
        self.cb.is_negative()
    }

    #[doc = doc::with_ctx::with_ctx!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn with_ctx(self, ctx: Context) -> Self {
        self.set_ctx(ctx).round_extra_precision().check()
    }

    #[doc = doc::with_rounding_mode::with_rounding_mode!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn with_rounding_mode(self, rm: RoundingMode) -> Self {
        self.set_rounding_mode(rm).round_extra_precision().check()
    }

    /// Invert the sign of the given decimal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(+1.0).neg(), dec256!(-1.0));
    /// assert_eq!(dec256!(1.0).neg(), dec256!(-1.0));
    /// assert_eq!(dec256!(-1.0).neg(), dec256!(1.0));
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn neg(mut self) -> Self {
        self.cb.neg();
        self
    }

    /// Get the absolute value of the decimal (non-negative sign).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1.0).abs(), dec256!(1.0));
    /// assert_eq!(dec256!(-1.0).abs(), dec256!(1.0));
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn abs(self) -> Self {
        math::abs::abs(self).check()
    }

    /// Get the absolute value of the decimal (non-negative sign) as
    /// [UnsignedDecimal].
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{dec256, udec256};
    ///
    /// assert_eq!(dec256!(1.0).unsigned_abs(), udec256!(1.0));
    /// assert_eq!(dec256!(-1.0).unsigned_abs(), udec256!(1.0));
    /// ```
    #[must_use]
    #[inline]
    pub const fn unsigned_abs(self) -> UnsignedDecimal<N> {
        UnsignedDecimal::new(self.abs())
    }

    /// The quantum of a finite number is given by: 1 × 10<sup>exp</sup>.
    /// This is the value of a unit in the least significant position of the
    /// coefficient of a finite number.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{D256, dec256, decimal::Context};
    ///
    /// let ctx = Context::default();
    ///
    /// assert_eq!(D256::quantum(0, ctx), dec256!(1));
    /// assert_eq!(D256::quantum(-0, ctx), dec256!(1));
    /// assert_eq!(D256::quantum(-3, ctx), dec256!(0.001));
    /// assert_eq!(D256::quantum(3, ctx), dec256!(1000));
    /// ```
    #[must_use]
    #[track_caller]
    #[inline]
    pub const fn quantum(exp: i32, ctx: Context) -> Self {
        scale::quantum(exp, ctx).check()
    }

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or
    ///   [`INFINITY`](Self::INFINITY)
    /// - `-1.0` if the number is negative, `-0.0` or
    ///   [`NEG_INFINITY`](Self::NEG_INFINITY)
    /// - [`NAN`](Self::NAN) if the number is [`NAN`](Self::NAN)
    ///
    /// ```
    /// use fastnum::{D256, dec256};
    ///
    /// let d = dec256!(3.5);
    ///
    /// assert_eq!(d.signum(), dec256!(1.0));
    /// assert_eq!(D256::NEG_INFINITY.signum(), dec256!(-1.0));
    ///
    /// assert!(D256::NAN.signum().is_nan());
    /// ```
    #[must_use]
    #[inline]
    pub const fn signum(&self) -> Self {
        if self.is_nan() {
            Self::NAN
        } else if self.is_negative() {
            Self::ONE.neg()
        } else {
            Self::ONE
        }
    }

    /// Reduces a decimal number to its shortest (coefficient)
    /// form shifting all significant trailing zeros into the exponent.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = dec256!(-1234500);
    /// assert_eq!(a.digits(), u256!(1234500));
    /// assert_eq!(a.fractional_digits_count(), 0);
    ///
    /// let b = a.reduce();
    /// assert_eq!(b.digits(), u256!(12345));
    /// assert_eq!(b.fractional_digits_count(), -2);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn reduce(self) -> Self {
        scale::reduce(self).check()
    }

    /// Tests for `self` and `other` values to be equal, and is used by `==`
    /// operator.
    #[must_use]
    #[inline(always)]
    pub const fn eq(&self, other: &Self) -> bool {
        cmp::eq(self, other)
    }

    /// Tests for `self` and `other` values to be equal, and is used by `==`
    /// operator.
    #[must_use]
    #[inline(always)]
    pub const fn ne(&self, other: &Self) -> bool {
        cmp::ne(self, other)
    }

    /// Compares and returns the maximum of two signed decimal values.
    ///
    /// Returns the second argument if the comparison determines them to be
    /// equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{dec256};
    ///
    /// assert_eq!(dec256!(1).max(dec256!(2)), dec256!(2));
    /// assert_eq!(dec256!(2).max(dec256!(2)), dec256!(2));
    /// ```
    #[must_use]
    #[inline]
    pub const fn max(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => other,
            _ => self,
        }
    }

    /// Compares and returns the minimum of two signed decimal values.
    ///
    /// Returns the first argument if the comparison determines them to be
    /// equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1).min(dec256!(2)), dec256!(1));
    /// assert_eq!(dec256!(2).min(dec256!(2)), dec256!(2));
    /// ```
    #[must_use]
    #[inline]
    pub const fn min(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => self,
            _ => other,
        }
    }

    /// Restrict a signed decimal value to a certain interval.
    ///
    /// Returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise, this returns `self`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(-3).clamp(dec256!(-2), dec256!(1)), dec256!(-2));
    /// assert_eq!(dec256!(0).clamp(dec256!(-2), dec256!(1)), dec256!(0));
    /// assert_eq!(dec256!(2).clamp(dec256!(-2), dec256!(1)), dec256!(1));
    /// ```
    #[must_use]
    #[inline]
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min.le(&max));
        if let Ordering::Less = self.cmp(&min) {
            min
        } else if let Ordering::Greater = self.cmp(&max) {
            max
        } else {
            self
        }
    }

    /// The positive difference of two decimal numbers.
    ///
    /// # Examples
    ///
    /// * If `self <= other`: `0:0`
    /// * Else: `self - other`
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1).abs_sub(dec256!(3)), dec256!(0));
    /// assert_eq!(dec256!(3).abs_sub(dec256!(1)), dec256!(2));
    /// ```
    #[must_use]
    #[inline]
    pub const fn abs_sub(self, other: Self) -> Self {
        if self.le(&other) {
            Self::ZERO
        } else {
            math::sub::sub(self, other)
        }
    }

    /// Tests signed decimal `self` less than `other` and is used by the `<`
    /// operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1.0).lt(&dec256!(1.0)), false);
    /// assert_eq!(dec256!(1.0).lt(&dec256!(2.0)), true);
    /// assert_eq!(dec256!(2.0).lt(&dec256!(1.0)), false);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn lt(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Less)
    }

    /// Tests signed decimal `self` less than or equal to `other` and is used by
    /// the `<=` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1.0).le(&dec256!(1.0)), true);
    /// assert_eq!(dec256!(1.0).le(&dec256!(2.0)), true);
    /// assert_eq!(dec256!(2.0).le(&dec256!(1.0)), false);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn le(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Less | Ordering::Equal)
    }

    /// Tests signed decimal `self` greater than `other` and is used by the `>`
    /// operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1.0).gt(&dec256!(1.0)), false);
    /// assert_eq!(dec256!(1.0).gt(&dec256!(2.0)), false);
    /// assert_eq!(dec256!(2.0).gt(&dec256!(1.0)), true);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn gt(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Greater)
    }

    /// Tests signed decimal `self` greater than or equal to `other` and is used
    /// by the `>=` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1.0).ge(&dec256!(1.0)), true);
    /// assert_eq!(dec256!(1.0).ge(&dec256!(2.0)), false);
    /// assert_eq!(dec256!(2.0).ge(&dec256!(1.0)), true);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn ge(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Greater | Ordering::Equal)
    }

    /// This method returns an [`Ordering`] between `self` and `other`.
    ///
    /// By convention, `self.cmp(&other)` returns the ordering matching the
    /// expression `self <operator> other` if true.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(dec256!(5).cmp(&dec256!(10)), Ordering::Less);
    /// assert_eq!(dec256!(10).cmp(&dec256!(5)), Ordering::Greater);
    /// assert_eq!(dec256!(5).cmp(&dec256!(5)), Ordering::Equal);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn cmp(&self, other: &Self) -> Ordering {
        cmp::cmp(self, other)
    }

    /// Calculates `self` + `rhs`.
    ///
    /// Is internally used by the `+` operator.
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("addition operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = D256::ONE;
    /// let b = D256::TWO;
    ///
    /// let c = a + b;
    /// assert_eq!(c, dec256!(3));
    /// ```
    ///
    /// Panics if overflowed:
    ///
    /// ```should_panic
    /// use fastnum::*;
    ///
    /// let a = D256::MAX;
    /// let b = D256::MAX;
    ///
    /// let c = a + b;
    /// ```
    ///
    /// See more about [add and subtract](crate#addition-and-subtraction).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline(always)]
    pub const fn add(self, rhs: Self) -> Self {
        math::add::add(self, rhs).round_extra_precision().check()
    }

    /// Calculates `self` – `rhs`.
    ///
    /// Is internally used by the `-` operator.
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("subtract operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = D256::ONE;
    /// let b = D256::TWO;
    ///
    /// let c = a - b;
    /// assert_eq!(c, dec256!(-1));
    /// ```
    ///
    /// Panics if overflowed:
    ///
    /// ```should_panic
    /// use fastnum::*;
    ///
    /// let a = D256::MAX;
    /// let b = -D256::MAX;
    ///
    /// let c = a - b;
    /// ```
    /// See more about [add and subtract](crate#addition-and-subtraction).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline(always)]
    pub const fn sub(self, rhs: Self) -> Self {
        math::sub::sub(self, rhs).round_extra_precision().check()
    }

    /// Calculates `self` × `rhs`.
    ///
    /// Is internally used by the `*` operator.
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("multiplication operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = D256::FIVE;
    /// let b = D256::TWO;
    ///
    /// let c = a * b;
    /// assert_eq!(c, dec256!(10));
    /// ```
    ///
    /// Panics if overflowed:
    ///
    /// ```should_panic
    /// use fastnum::*;
    ///
    /// let a = D256::MAX;
    /// let b = D256::MAX;
    ///
    /// let c = a * b;
    /// ```
    ///
    /// See more about [multiplication](crate#multiplication).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline(always)]
    pub const fn mul(self, rhs: Self) -> Self {
        math::mul::mul(self, rhs).round_extra_precision().check()
    }

    /// Calculates `self` ÷ `rhs`.
    ///
    /// Is internally used by the `/` operator.
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("divide operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = D256::FIVE;
    /// let b = D256::TWO;
    ///
    /// let c = -a / b;
    /// assert_eq!(c, dec256!(-2.5));
    /// ```
    ///
    /// Panics if divided by zero:
    ///
    /// ```should_panic
    /// use fastnum::{dec256, D256};
    ///
    /// let a = D256::ONE;
    /// let b = D256::ZERO;
    ///
    /// let c = a / b;
    /// ```
    ///
    /// See more about [division](crate#division).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline(always)]
    pub const fn div(self, rhs: Self) -> Self {
        math::div::div(self, rhs).round_extra_precision().check()
    }

    /// Calculates `self` % `rhs`.
    ///
    /// Is internally used by the `%` operator.
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("reminder operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = D256::FIVE;
    /// let b = D256::TWO;
    ///
    /// let c = a % b;
    /// assert_eq!(c, dec256!(1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline(always)]
    pub const fn rem(self, rhs: Self) -> Self {
        math::rem::rem(self, rhs).round_extra_precision().check()
    }

    /// Raise a decimal number to decimal power.
    ///
    /// Using this function is generally slower than using `powi` for integer
    /// powers or `sqrt` method for `1/2` exponent.
    #[doc = doc::decimal_inexact!("power operation")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("power operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec256!(4).pow(dec256!(0.5)), dec256!(2));
    /// assert_eq!(dec256!(8).pow(dec256!(1) / dec256!(3)), dec256!(2));
    /// ```
    ///
    /// See more about the [power](crate#power) operation.
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn pow(self, n: Self) -> Self {
        math::pow::pow(self, n).round_extra_precision().check()
    }

    /// Raise a decimal number to an integer power.
    ///
    /// Using this function is generally faster than using `pow`
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("power operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec256!(2).powi(3), dec256!(8));
    /// assert_eq!(dec256!(9).powi(2), dec256!(81));
    /// assert_eq!(dec256!(1).powi(-2), dec256!(1));
    /// assert_eq!(dec256!(10).powi(20), dec256!(1e20));
    /// assert_eq!(dec256!(4).powi(-2), dec256!(0.0625));
    /// ```
    ///
    /// See more about the [power](crate#power) operation.
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn powi(self, n: i32) -> Self {
        math::powi::powi(self, n).round_extra_precision().check()
    }

    /// Take the square root of the decimal number using
    /// [Heron's method](https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method),
    /// a special case of [Newton's](https://en.wikipedia.org/wiki/Newton%27s_method) method.
    ///
    /// Returns [`NaN`](crate#nan) if `self` is a negative number other than
    /// `-0.0`.
    #[doc = doc::decimal_inexact!("square root operation")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("square root operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec128!(4).sqrt(), dec128!(2));
    /// assert_eq!(dec128!(1).sqrt(), dec128!(1));
    /// assert_eq!(dec128!(16).sqrt(), dec128!(4));
    /// assert_eq!(dec128!(2).sqrt(), D128::SQRT_2);
    /// ```
    ///
    /// See more about the [square-root](crate#square-root) operation.
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn sqrt(self) -> Self {
        math::sqrt::sqrt(self).round_extra_precision().check()
    }

    /// Take the cubic root of a decimal number using
    /// [Newton's method](https://en.wikipedia.org/wiki/Newton%27s_method).
    #[doc = doc::decimal_inexact!("cubic root operation")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("cubic root operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec128!(8).cbrt(), dec128!(2));
    /// ```
    ///
    /// See more about the [N-th root](crate#n-th-roots) operation.
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn cbrt(self) -> Self {
        math::cbrt::cbrt(self).round_extra_precision().check()
    }

    /// Take the N-th root of the decimal number using
    /// [Newton's method](https://en.wikipedia.org/wiki/Newton%27s_method).
    #[doc = doc::decimal_inexact!("N-th root operation")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("N-th root operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec128!(16).nth_root(4), dec128!(2));
    /// ```
    ///
    /// See more about the [N-th root](crate#n-th-roots) operation.
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn nth_root(self, n: u32) -> Self {
        math::nth_root::nth_root(self, n)
            .round_extra_precision()
            .check()
    }

    /// Returns _e<sup>self</sup>_, (the exponential function).
    #[doc = doc::decimal_inexact!("exponential function")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("exponent calculation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec128!(1).exp(), D128::E);
    /// ```
    ///
    /// See more about the [exponential function](crate#exponential-function).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn exp(self) -> Self {
        math::exp::exp(self).round_extra_precision().check()
    }

    /// Returns _e<sup>self</sup> – 1_.
    #[doc = doc::decimal_inexact!("exponential function")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("exponent calculation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{*, decimal::RoundingMode::*};
    ///
    /// // For exact result we need to use extra precision digits and no-rounding mode to keep precision between `.ln()` and `exp()` calls.
    /// // Than we can use default rounding mode for round extra digits.
    /// assert_eq!(dec128!(7.0).with_rounding_mode(No).ln().exp_m1().with_rounding_mode(HalfUp), D128::SIX);
    /// ```
    ///
    /// See more about the [exponential function](crate#exponential-function).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn exp_m1(self) -> Self {
        math::exp::exp_m1(self).round_extra_precision().check()
    }

    /// Returns _2<sup>self</sup>_, (the binary exponential function).
    #[doc = doc::decimal_inexact!("binary exponential function")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("binary exponential function")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec128!(0).exp2(), dec128!(1));
    /// assert_eq!(dec128!(1).exp2(), dec128!(2));
    /// assert_eq!(dec128!(2).exp2(), dec128!(4));
    /// assert_eq!(dec128!(3).exp2(), dec128!(8));
    /// ```
    /// See more about the [binary exponential
    /// function](crate#binary-exponential-function).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn exp2(self) -> Self {
        math::exp2::exp2(self).round_extra_precision().check()
    }

    #[doc = doc::log::ln!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn ln(self) -> Self {
        math::log::ln(self).round_extra_precision().check()
    }

    #[doc = doc::log::ln_1p!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn ln_1p(self) -> Self {
        math::log::ln_1p(self).round_extra_precision().check()
    }

    #[doc = doc::log::log!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn log(self, base: Self) -> Self {
        math::log::log(self, base).round_extra_precision().check()
    }

    #[doc = doc::log::log2!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn log2(self) -> Self {
        math::log::log2(self).round_extra_precision().check()
    }

    #[doc = doc::log::log10!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn log10(self) -> Self {
        math::log::log10(self).round_extra_precision().check()
    }

    /// Calculate the length of the hypotenuse of a right-angle triangle given
    /// legs of length `x` and `y`.
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("hypotenuse calculate operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// let x = dec256!(2);
    /// let y = dec256!(3);
    ///
    /// // sqrt(x^2 + y^2)
    /// assert_eq!(x.hypot(y), (x.powi(2) + y.powi(2)).sqrt());
    /// ```
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn hypot(self, other: Self) -> Self {
        math::hypot::hypot(self, other)
            .round_extra_precision()
            .check()
    }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("multiply-add operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec128!(10.0).mul_add(dec128!(4.0), dec128!(60)), dec128!(100));
    /// ```
    ///
    /// See more about the [fused multiply-add
    /// function](crate#fused-multiply-add).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn mul_add(self, a: Self, b: Self) -> Self {
        math::add::add(self.mul(a), b)
            .round_extra_precision()
            .check()
    }

    /// Returns the given decimal number rounded to `digits` precision after the
    /// decimal point, using [RoundingMode] from it [Context].
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("round operation (up-scale or down-scale)")]
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::{*, RoundingMode::*}};
    ///
    /// let n = dec256!(129.41675);
    ///
    /// // Default rounding mode is `HalfUp`
    /// assert_eq!(n.round(2),  dec256!(129.42));
    ///
    /// assert_eq!(n.with_rounding_mode(Up).round(2), dec256!(129.42));
    /// assert_eq!(n.with_rounding_mode(Down).round(-1), dec256!(120));
    /// assert_eq!(n.with_rounding_mode(HalfEven).round(4), dec256!(129.4168));
    /// ```
    /// See also:
    /// - More about [`round`](crate#rounding) decimals.
    /// - [RoundingMode].
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline(always)]
    pub const fn round(self, digits: i16) -> Self {
        self.rescale(digits)
    }

    #[doc = doc::trunc::trunc!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn trunc(self) -> Self {
        truncate::truncate(self, 0).check()
    }

    #[doc = doc::trunc::trunc_with_scale!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn trunc_with_scale(self, scale: i16) -> Self {
        truncate::truncate(self, scale).check()
    }

    /// Returns the largest integer less than or equal to a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec256!(3.99).floor(), dec256!(3));
    /// assert_eq!(dec256!(3.0).floor(), dec256!(3));
    /// assert_eq!(dec256!(3.01).floor(), dec256!(3));
    /// assert_eq!(dec256!(3.5).floor(), dec256!(3));
    /// assert_eq!(dec256!(4.0).floor(), dec256!(4));
    ///
    /// assert_eq!(dec256!(-3.01).floor(), dec256!(-4));
    /// assert_eq!(dec256!(-3.1).floor(), dec256!(-4));
    /// assert_eq!(dec256!(-3.5).floor(), dec256!(-4));
    /// assert_eq!(dec256!(-4.0).floor(), dec256!(-4));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn floor(self) -> Self {
        round::floor(self)
    }

    /// Finds the nearest integer greater than or equal to `x`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec256!(3.01).ceil(), dec256!(4));
    /// assert_eq!(dec256!(3.99).ceil(), dec256!(4));
    /// assert_eq!(dec256!(4.0).ceil(), dec256!(4));
    /// assert_eq!(dec256!(1.0001).ceil(), dec256!(2));
    /// assert_eq!(dec256!(1.00001).ceil(), dec256!(2));
    /// assert_eq!(dec256!(1.000001).ceil(), dec256!(2));
    /// assert_eq!(dec256!(1.00000000000001).ceil(), dec256!(2));
    ///
    /// assert_eq!(dec256!(-3.01).ceil(), dec256!(-3));
    /// assert_eq!(dec256!(-3.5).ceil(), dec256!(-3));
    /// assert_eq!(dec256!(-4.0).ceil(), dec256!(-4));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn ceil(self) -> Self {
        round::ceil(self)
    }

    /// Returns the given decimal number _re-scaled_ to `digits` precision after
    /// the decimal point.
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("rescale operation")]
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// assert_eq!(dec256!(2.17).rescale(3), dec256!(2.170));
    /// assert_eq!(dec256!(2.17).rescale(2), dec256!(2.17));
    /// assert_eq!(dec256!(2.17).rescale(1), dec256!(2.2));
    /// assert_eq!(dec256!(2.17).rescale(0), dec256!(2));
    /// assert_eq!(dec256!(2.17).rescale(-1), dec256!(0));
    ///
    /// let ctx = Context::default().without_traps();
    ///
    /// assert!(D256::INFINITY.with_ctx(ctx).rescale(2).is_nan());
    /// assert!(D256::NEG_INFINITY.with_ctx(ctx).rescale(2).is_nan());
    /// assert!(D256::NAN.with_ctx(ctx).rescale(1).is_nan());
    /// ```
    ///
    /// See also:
    /// - More about [`rescale`](crate#rescale) decimals.
    /// - [Self::quantize].
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline(always)]
    pub const fn rescale(mut self, new_scale: i16) -> Self {
        scale::rescale(&mut self, new_scale);
        self.round_extra_precision().check()
    }

    /// Returns a value equal to `self` (rounded), having the exponent of
    /// `other`.
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("quantize operation")]
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// let ctx = Context::default().without_traps();
    ///
    /// assert_eq!(dec256!(2.17).quantize(dec256!(0.001)), dec256!(2.170));
    /// assert_eq!(dec256!(2.17).quantize(dec256!(0.01)), dec256!(2.17));
    /// assert_eq!(dec256!(2.17).quantize(dec256!(0.1)), dec256!(2.2));
    /// assert_eq!(dec256!(2.17).quantize(dec256!(1e+0)), dec256!(2));
    /// assert_eq!(dec256!(2.17).quantize(dec256!(1e+1)), dec256!(0));
    ///
    /// assert_eq!(D256::NEG_INFINITY.quantize(D256::INFINITY), D256::NEG_INFINITY);
    ///
    /// assert!(dec256!(2).with_ctx(ctx).quantize(D256::INFINITY).is_nan());
    ///
    /// assert_eq!(dec256!(-0.1).quantize(dec256!(1)), dec256!(-0));
    /// assert_eq!(dec256!(-0).quantize(dec256!(1e+5)), dec256!(-0E+5));
    ///
    /// assert!(dec128!(0.34028).with_ctx(ctx).quantize(dec128!(1e-32765)).is_nan());
    /// assert!(dec128!(-0.34028).with_ctx(ctx).quantize(dec128!(1e-32765)).is_nan());
    ///
    /// assert_eq!(dec256!(217).quantize(dec256!(1e-1)), dec256!(217.0));
    /// assert_eq!(dec256!(217).quantize(dec256!(1e+0)), dec256!(217));
    /// assert_eq!(dec256!(217).quantize(dec256!(1e+1)), dec256!(2.2E+2));
    /// assert_eq!(dec256!(217).quantize(dec256!(1e+2)), dec256!(2E+2));
    /// ```
    ///
    /// See also:
    /// - More about [`quantize`](crate#quantize) decimals.
    /// - [Self::rescale].
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn quantize(self, other: Self) -> Self {
        scale::quantize(self, other).round_extra_precision().check()
    }

    /// Returns:
    /// - `true` if no [Exceptional condition] [Signals] flag has been trapped
    ///   by [Context] trap-enabler, and
    /// - `false` otherwise.
    ///
    /// [Exceptional condition]: crate#signaling-flags-and-trap-enablers
    #[inline(always)]
    pub const fn is_ok(&self) -> bool {
        self.cb.trap_signals().is_empty()
    }

    /// Returns:
    /// - `Some(Self)` if no [Exceptional condition] [Signals] flag has been
    ///   trapped by [Context] trap-enabler, and
    /// - `None` otherwise.
    ///
    /// [Exceptional condition]: crate#signaling-flags-and-trap-enablers
    #[inline]
    pub const fn ok(self) -> Option<Self> {
        if self.cb.trap_signals().is_empty() {
            Some(self)
        } else {
            None
        }
    }

    /// Takes the reciprocal (inverse) of a number, `1/x`.
    #[doc = doc::decimal_inexact!("reciprocal")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("reciprocal operation")]
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec256!(2).recip(), dec256!(0.5));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn recip(self) -> Self {
        math::recip::recip(self).round_extra_precision().check()
    }

    /// Converts radians to degrees.
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("conversion")]
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(D128::PI.to_degrees(), dec128!(180));
    /// assert_eq!(D128::TAU.to_degrees(), dec128!(360));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_degrees(self) -> Self {
        math::mul::mul(math::div::div(self, Self::PI), Consts::C_180)
            .round_extra_precision()
            .check()
    }

    /// Converts degrees to radians.
    ///
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("conversion")]
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec128!(180).to_radians(), D128::PI);
    /// assert_eq!(dec128!(360).to_radians(), D128::TAU);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_radians(self) -> Self {
        math::div::div(math::mul::mul(self, Self::PI), Consts::C_180)
            .round_extra_precision()
            .check()
    }

    /// Create a string of this decimal in scientific notation.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// let n = dec256!(-12345678);
    /// assert_eq!(&n.to_scientific_notation(), "-1.2345678e7");
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub fn to_scientific_notation(&self) -> String {
        let mut output = String::new();
        self.write_scientific_notation(&mut output)
            .expect("Could not write to string");
        output
    }

    /// Create a string of this decimal in engineering notation.
    ///
    /// Engineering notation is scientific notation with the exponent
    /// coerced to a multiple of three
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// let n = dec256!(-12345678);
    /// assert_eq!(&n.to_engineering_notation(), "-12.345678e6");
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub fn to_engineering_notation(&self) -> String {
        let mut output = String::new();
        self.write_engineering_notation(&mut output)
            .expect("Could not write to string");
        output
    }

    /// _Deprecated_, use [`resize`](Self::resize) instead.
    #[deprecated(since = "0.5.0")]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn transmute<const M: usize>(self) -> Decimal<M> {
        self.resize()
    }

    #[doc = doc::resize::resize!(64)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn resize<const M: usize>(self) -> Decimal<M> {
        resize::resize(self).round_extra_precision().check()
    }

    /// Returns `true` if the decimal number is even.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(3).is_even(), false);
    /// assert_eq!(dec256!(4).is_even(), true);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn is_even(&self) -> bool {
        math::utils::is_even(self)
    }

    /// Returns `true` if the decimal number is odd.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(3).is_odd(), true);
    /// assert_eq!(dec256!(4).is_odd(), false);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn is_odd(&self) -> bool {
        math::utils::is_odd(self)
    }

    /// Returns `true` if the decimal number is integral.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(3.3).is_integral(), false);
    /// assert_eq!(dec256!(4).is_integral(), true);
    /// assert_eq!(dec256!(1.0).is_integral(), true);
    /// assert_eq!(dec256!(10.0).is_integral(), true);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn is_integral(&self) -> bool {
        math::utils::is_integral(self)
    }

    /// Computes _sin(self)_ (trigonometric sine of decimal number in radians).
    #[doc = doc::decimal_inexact!("trigonometric sine")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("trigonometric sine operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(D128::FRAC_PI_2.sin(), dec128!(1));
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn sin(self) -> Self {
        math::sin::sin(self).round_extra_precision().check()
    }

    /// Computes _cos(self)_ (trigonometric cosine of decimal number in
    /// radians).
    #[doc = doc::decimal_inexact!("trigonometric cosine")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("trigonometric cosine operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(D128::TAU.cos(), dec128!(1));
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn cos(self) -> Self {
        math::cos::cos(self).round_extra_precision().check()
    }

    /// Computes _tan(self)_ (trigonometric tangent of decimal number in
    /// radians).
    #[doc = doc::decimal_inexact!("trigonometric tangent")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("trigonometric tangent operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(D128::FRAC_PI_4.tan(), dec128!(1));
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn tan(self) -> Self {
        math::tan::tan(self).round_extra_precision().check()
    }

    /// Computes _arcsin(self)_ (trigonometric arcsine of decimal number).
    ///
    /// Return value is in radians in the range [-π/2, π/2] or `NaN`
    /// if the number is outside the range [-1, 1].
    #[doc = doc::decimal_inexact!("trigonometric arcsine")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("trigonometric arcsine operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(D128::FRAC_PI_2.sin().asin(), D128::FRAC_PI_2);
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn asin(self) -> Self {
        math::asin::asin(self).round_extra_precision().check()
    }

    /// Computes _arccos(self)_ (trigonometric arccosine of decimal number).
    ///
    /// Return value is in radians in the range [0, π] or `NaN`
    /// if the number is outside the range [-1, 1].
    #[doc = doc::decimal_inexact!("trigonometric arccosine")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("trigonometric arccosine operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec256!(1).acos(), dec256!(0));
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn acos(self) -> Self {
        math::acos::acos(self).round_extra_precision().check()
    }

    /// Computes _arctan(self)_ (trigonometric arctangent of decimal number).
    ///
    /// Return value is in radians in the range [-π/2, π/2].
    #[doc = doc::decimal_inexact!("trigonometric arctangent")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("trigonometric arctangent operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(D128::ZERO.atan(), D128::ZERO);
    /// assert_eq!(D128::ONE.atan(), D128::FRAC_PI_4);
    /// assert_eq!(D128::ONE.neg().atan(), D128::FRAC_PI_4.neg());
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn atan(self) -> Self {
        math::atan::atan(self).round_extra_precision().check()
    }

    /// Computes the [_four quadrant_ arctangent](https://en.wikipedia.org/wiki/Atan2)
    /// of `self` (`y`) and `other` (`x`).
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-π/2, π/2]`
    /// * `y >= 0`: `arctan(y/x) + π` -> `(pi/2, π]`
    /// * `y < 0`: `arctan(y/x) - π` -> `(-π, -π/2)`
    #[doc = doc::decimal_inexact!("trigonometric 2-argument arctangent")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("trigonometric 2-argument arctangent operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec128!(-3.0).atan2(dec128!(3.0)), D128::FRAC_PI_4.neg());
    /// assert_eq!(dec128!(3.0).atan2(dec128!(-3.0)), D128::FRAC_PI_4 * D128::THREE);
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn atan2(self, other: Self) -> Self {
        math::atan2::atan2(self, other)
            .round_extra_precision()
            .check()
    }

    /// Simultaneously computes the sine and cosine of the number, `x`.
    /// Returns `(sin(x), cos(x))`.
    #[doc = doc::decimal_inexact!("trigonometric sine and cosine function")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("trigonometric sine and cosine computation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(D128::FRAC_PI_2.sin_cos(), (dec128!(1), dec128!(0)));
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = math::sin_cos::sin_cos(self);
        (
            sin.round_extra_precision().check(),
            cos.round_extra_precision().check(),
        )
    }

    /// Computes _sinh(self)_ (hyperbolic sine of decimal number).
    #[doc = doc::decimal_inexact!("hyperbolic sine")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("hyperbolic sine operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec128!(1).sinh(), (D128::E * D128::E - dec128!(1.0)) / (dec128!(2.0) * D128::E));
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn sinh(self) -> Self {
        math::sinh::sinh(self).round_extra_precision().check()
    }

    /// Computes _cosh(self)_ (hyperbolic cosine of decimal number).
    #[doc = doc::decimal_inexact!("hyperbolic cosine")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("hyperbolic cosine operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec256!(1).cosh(), (D256::E * D256::E + dec256!(1.0)) / (dec256!(2.0) * D256::E));
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn cosh(self) -> Self {
        math::cosh::cosh(self).round_extra_precision().check()
    }

    /// Computes _tanh(self)_ (hyperbolic tangent of decimal number).
    #[doc = doc::decimal_inexact!("hyperbolic tangent")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("hyperbolic tangent operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let e2 = D128::E * D128::E;
    /// assert_eq!(dec128!(1).tanh(), (e2 - dec128!(1.0)) / (e2 + dec128!(1.0)));
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn tanh(self) -> Self {
        math::tanh::tanh(self).round_extra_precision().check()
    }

    /// Computes _arsinh(self)_ (inverse hyperbolic sine of decimal number).
    #[doc = doc::decimal_inexact!("inverse hyperbolic sine")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("inverse hyperbolic sine operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec128!(1).sinh().asinh(), dec128!(1));
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn asinh(self) -> Self {
        math::asinh::asinh(self).round_extra_precision().check()
    }

    /// Computes _arcosh(self)_ (inverse hyperbolic cosine of decimal number).
    #[doc = doc::decimal_inexact!("inverse hyperbolic cosine")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("inverse hyperbolic cosine operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec128!(1).cosh().acosh(), dec128!(1));
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn acosh(self) -> Self {
        math::acosh::acosh(self).round_extra_precision().check()
    }

    /// Computes _artanh(self)_ (inverse hyperbolic tangent of decimal number).
    #[doc = doc::decimal_inexact!("inverse hyperbolic tangent")]
    /// # Panics:
    #[doc = doc::decimal_operation_panics!("inverse hyperbolic tangent operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(D256::ZERO.tanh().atanh(), D256::ZERO);
    /// ```
    ///
    /// See more about the [trigonometric
    /// functions](crate#trigonometric-functions).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn atanh(self) -> Self {
        math::atanh::atanh(self).round_extra_precision().check()
    }

    /// Converts from [UnsignedDecimal] to a signed [Decimal] number.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let d = udec256!(1.2345);
    ///
    /// assert_eq!(D256::from_unsigned(d), dec256!(1.2345));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn from_unsigned(ud: UnsignedDecimal<N>) -> Self {
        ud.to_signed()
    }

    /// Try converts from [Decimal] to [UnsignedDecimal].
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(dec256!(1.2345).try_to_unsigned(), Ok(udec256!(1.2345)));
    /// assert!(dec256!(-1.2345).try_to_unsigned().is_err());
    /// ```
    #[inline]
    pub const fn try_to_unsigned(self) -> Result<UnsignedDecimal<N>, DecimalError> {
        UnsignedDecimal::try_from_signed(self)
    }
}

#[doc(hidden)]
impl<const N: usize> Decimal<N> {
    pub(crate) const SIGNALING_NAN: Self = Self::new(UInt::ZERO, ControlBlock::SIGNALING_NAN);

    const TYPE_NAME: &'static str = decimal::utils::fmt::type_name!("D");

    #[inline(always)]
    pub(crate) const fn new(digits: UInt<N>, cb: ControlBlock) -> Self {
        Self { digits, cb }
    }

    #[inline(always)]
    pub(crate) const fn decimal_power(&self) -> i32 {
        self.digits_count() as i32 - self.cb.get_scale() as i32
    }

    #[inline(always)]
    pub(crate) fn control_block(&self) -> ControlBlock {
        self.cb
    }

    #[inline(always)]
    pub(crate) const fn signals(&self) -> Signals {
        self.cb.get_signals()
    }

    #[inline(always)]
    pub(crate) const fn context(&self) -> Context {
        self.cb.get_context()
    }

    #[inline(always)]
    pub(crate) const fn raise_signals(mut self, signals: Signals) -> Self {
        self.cb.raise_signals(signals);
        self
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) const fn quiet_signals(mut self, signals: Signals) -> Self {
        self.cb.quiet_signals(signals);
        self
    }

    #[inline(always)]
    pub(crate) const fn compound(mut self, other: &Self) -> Self {
        self.cb.compound(&other.cb);
        self
    }

    #[inline(always)]
    pub(crate) const fn signaling_nan(mut self) -> Self {
        self.cb.signaling_nan();
        self
    }

    #[inline(always)]
    pub(crate) const fn op_invalid(mut self) -> Self {
        self.cb.raise_signals(Signals::OP_INVALID);
        self
    }

    #[inline(always)]
    pub(crate) const fn op_overflow(mut self) -> Self {
        self.cb.raise_signals(Signals::OP_OVERFLOW);
        self
    }

    #[inline(always)]
    pub(crate) const fn set_sign(mut self, sign: Sign) -> Self {
        self.cb.set_sign(sign);
        self
    }

    #[inline(always)]
    pub(crate) const fn set_ctx(mut self, ctx: Context) -> Self {
        self.cb.set_context(ctx);
        self
    }

    #[inline(always)]
    pub(crate) const fn set_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.cb.set_rounding_mode(rm);
        self
    }

    #[inline(always)]
    pub(crate) const fn check(mut self) -> Self {
        let trapped = self.cb.trap_signals();

        if !trapped.is_empty() {
            DecimalError::from_signals(trapped).panic();
            self.cb.signaling_nan();
        }

        self
    }

    #[inline(always)]
    pub(crate) const fn ok_or_err(self) -> Result<Self, DecimalError> {
        let trapped = self.cb.trap_signals();

        if trapped.is_empty() {
            Ok(self)
        } else {
            Err(DecimalError::from_signals(trapped))
        }
    }

    #[inline(always)]
    pub(crate) const fn round_extra_precision(mut self) -> Self {
        round(&mut self);
        self
    }

    #[inline(always)]
    pub(crate) const fn without_extra_digits(mut self) -> Self {
        self.cb.reset_extra_precision();
        self
    }

    #[inline(always)]
    pub(crate) const fn has_extra_precision(&self) -> bool {
        self.cb.has_extra_precision()
    }

    #[inline]
    pub(crate) const fn type_name() -> &'static str {
        Self::TYPE_NAME
    }

    /// Write unsigned decimal in scientific notation to writer `w`.
    pub(crate) fn write_scientific_notation<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        if self.is_nan() {
            return w.write_str("NaN");
        }

        if self.is_sign_negative() {
            w.write_str("-")?;
        }

        if self.is_infinite() {
            return w.write_str("Inf");
        }

        if self.is_zero() {
            return w.write_str("0e0");
        }

        let digits = self.digits.to_str_radix(10);
        let scale = self.cb.get_scale();
        format::write_scientific_notation(digits, scale, w)
    }

    /// Write unsigned decimal in engineering notation to writer `w`.
    pub(crate) fn write_engineering_notation<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        if self.is_nan() {
            return w.write_str("NaN");
        }

        if self.is_sign_negative() {
            w.write_str("-")?;
        }

        if self.is_infinite() {
            return w.write_str("Inf");
        }

        if self.is_zero() {
            return w.write_str("0e0");
        }

        let digits = self.digits.to_str_radix(10);
        let scale = self.cb.get_scale();
        format::write_engineering_notation(digits, scale, w)
    }

    #[allow(unsafe_code)]
    #[inline(always)]
    pub(crate) const unsafe fn _transmute<const M: usize>(self) -> Decimal<M> {
        Decimal::new(self.digits._transmute(), self.cb)
    }
}
