mod consts;
mod extras;
mod impls;

use core::{cmp::Ordering, num::FpCategory};

use crate::{
    decimal::{
        doc, udec::consts::consts_impl, Context, Decimal, DecimalError, Flags, ParseError,
        RoundingMode, Sign, Signal,
    },
    int::UInt,
};

/// # Unsigned Decimal
///
/// Generic unsigned N-bits decimal number.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct UnsignedDecimal<const N: usize>(Decimal<N>);

consts_impl!();

impl<const N: usize> UnsignedDecimal<N> {
    /// Creates and initializes unsigned decimal from parts.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// assert_eq!(UD256::from_parts(u256!(12345), -4, Context::default()), udec256!(1.2345));
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_parts(digits: UInt<N>, exp: i32, ctx: Context) -> Self {
        Self::from_signed(Decimal::from_parts(digits, exp, Sign::Plus, ctx))
    }

    /// Creates and initializes an unsigned decimal from string.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// assert_eq!(UD256::from_str("1.2345", Context::default()), Ok(udec256!(1.2345)));
    /// assert_eq!(UD256::from_str("-1.2345", Context::default()), Err(ParseError::Signed));
    /// ```
    #[track_caller]
    #[inline]
    pub const fn from_str(s: &str, ctx: Context) -> Result<Self, ParseError> {
        match Decimal::<N>::from_str(s, ctx) {
            Ok(d) => {
                if d.is_negative() {
                    Err(ParseError::Signed)
                } else {
                    Ok(Self::new(d))
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Parse an unsigned decimal from string.
    ///
    /// # Panics
    ///
    /// This function will panic if `UnsignedDecimal<N>` can't be constructed
    /// from a given string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// assert_eq!(UD256::parse_str("1.2345", Context::default()), udec256!(1.2345));
    /// ```
    ///
    /// Should panic:
    ///
    /// ```should_panic
    /// use fastnum::{*, decimal::*};
    ///
    /// let _ = UD256::parse_str("-1.2345", Context::default());
    /// ```
    #[track_caller]
    #[must_use]
    #[inline]
    pub const fn parse_str(s: &str, ctx: Context) -> Self {
        match Self::from_str(s, ctx) {
            Ok(n) => n,
            Err(e) => panic!("{}", e.description()),
        }
    }

    /// Returns the internal big integer, representing the
    /// [_Coefficient_](crate#representation) of a given `UnsignedDecimal`,
    /// including significant trailing zeros.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256, u256};
    ///
    /// let a = udec256!(123.45);
    /// assert_eq!(a.digits(), u256!(12345));
    ///
    /// let b = udec256!(1.0);
    /// assert_eq!(b.digits(), u256!(10));
    /// ```
    #[inline]
    pub const fn digits(&self) -> UInt<N> {
        self.0.digits()
    }

    /// Returns the count of digits in the non-scaled integer representation
    #[inline]
    pub const fn digits_count(&self) -> usize {
        self.0.digits_count()
    }

    /// Returns the scale of the `UnsignedDecimal`, the total number of
    /// digits to the right of the decimal point (including insignificant
    /// leading zeros).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// let a = udec256!(12345);  // No fractional part
    /// let b = udec256!(123.45);  // Fractional part
    /// let c = udec256!(0.0000012345);  // Completely fractional part
    /// let d = udec256!(500000000);  // No fractional part
    /// let e = udec256!(5e9);  // Negative-fractional part
    ///
    /// assert_eq!(a.fractional_digits_count(), 0);
    /// assert_eq!(b.fractional_digits_count(), 2);
    /// assert_eq!(c.fractional_digits_count(), 10);
    /// assert_eq!(d.fractional_digits_count(), 0);
    /// assert_eq!(e.fractional_digits_count(), -9);
    /// ```
    #[inline]
    pub const fn fractional_digits_count(&self) -> i16 {
        self.0.fractional_digits_count()
    }

    /// Return if the referenced unsigned decimal is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256};
    ///
    /// let a = udec256!(0);
    /// assert!(a.is_zero());
    ///
    /// let b = udec256!(0.0);
    /// assert!(b.is_zero());
    ///
    /// let c = udec256!(0.00);
    /// assert!(c.is_zero());
    ///
    /// let d = udec256!(0.1);
    /// assert!(!d.is_zero());
    /// ```
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Return if the referenced unsigned decimal is strictly [Self::ONE].
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256};
    ///
    /// let a = udec256!(1);
    /// assert!(a.is_one());
    ///
    /// let b = udec256!(10e-1);
    /// assert!(!b.is_one());
    /// ```
    #[inline]
    pub const fn is_one(&self) -> bool {
        self.0.is_one()
    }

    /// Returns `true` if the given decimal number is the result of division by
    /// zero and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// let ctx = Context::default().without_traps();
    /// let res = udec256!(1.0).with_ctx(ctx) / udec256!(0).with_ctx(ctx);
    ///
    /// assert!(res.is_op_div_by_zero());
    /// ```
    ///
    /// More about [`OP_DIV_BY_ZERO`](Signal::OP_DIV_BY_ZERO) signal.
    #[must_use]
    #[inline]
    pub const fn is_op_div_by_zero(&self) -> bool {
        self.0.is_op_div_by_zero()
    }

    /// Return `true` if the argument has [Signal::OP_INVALID] signal flag, and
    /// `false` otherwise.
    #[must_use]
    #[inline]
    pub const fn is_op_invalid(&self) -> bool {
        self.0.is_op_invalid()
    }

    /// Return `true` if the argument has [Signal::OP_SUBNORMAL] signal flag,
    /// and `false` otherwise.
    #[must_use]
    #[inline]
    pub const fn is_op_subnormal(&self) -> bool {
        self.0.is_op_subnormal()
    }

    /// Return `true` if the argument has [Signal::OP_INEXACT] signal flag, and
    /// `false` otherwise.
    #[must_use]
    #[inline]
    pub const fn is_op_inexact(&self) -> bool {
        self.0.is_op_inexact()
    }

    /// Return `true` if the argument has [Signal::OP_ROUNDED] signal flag, and
    /// `false` otherwise.
    #[must_use]
    #[inline]
    pub const fn is_op_rounded(&self) -> bool {
        self.0.is_op_rounded()
    }

    /// Return `true` if the argument has [Signal::OP_CLAMPED] signal flag, and
    /// `false` otherwise.
    #[must_use]
    #[inline]
    pub const fn is_op_clamped(&self) -> bool {
        self.0.is_op_clamped()
    }

    /// Return `true` if the argument has [Signal::OP_OVERFLOW] signal flag, and
    /// `false` otherwise.
    #[must_use]
    #[inline]
    pub const fn is_op_overflow(&self) -> bool {
        self.0.is_op_overflow()
    }

    /// Return `true` if the argument has [Signal::OP_UNDERFLOW] signal flag,
    /// and `false` otherwise.
    #[must_use]
    #[inline]
    pub const fn is_op_underflow(&self) -> bool {
        self.0.is_op_underflow()
    }

    /// Return `true` if the argument has no signal flags, and `false`
    /// otherwise.
    #[must_use]
    #[inline]
    pub const fn is_op_ok(&self) -> bool {
        self.0.is_op_ok()
    }

    /// Return the [`signaling block`](Signal) of given unsigned decimal.
    #[must_use]
    #[inline]
    pub const fn op_signals(&self) -> Signal {
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
    /// use fastnum::{udec256, UD256};
    ///
    /// let num = udec256!(12.4);
    /// let inf = UD256::INFINITY;
    ///
    /// assert_eq!(num.classify(), FpCategory::Normal);
    /// assert_eq!(inf.classify(), FpCategory::Infinite);
    /// ```
    #[must_use]
    #[inline]
    pub const fn classify(&self) -> FpCategory {
        self.0.classify()
    }

    /// Return `true` if the number is neither [zero], [`Infinity`],
    /// [subnormal], or [`NaN`] and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let num = udec256!(12.4);
    /// let subnormal = udec256!(1E-30000) / udec256!(1E2768);
    /// let inf = UD256::INFINITY;
    /// let nan = UD256::NAN;
    /// let zero = UD256::ZERO;
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
    /// [`Infinity`]: crate#special-values
    /// [`NaN`]: crate#special-values
    #[must_use]
    #[inline]
    pub const fn is_normal(self) -> bool {
        self.0.is_normal()
    }

    /// Return `true` if the number is [subnormal] and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let num = udec256!(12.4);
    /// let subnormal = udec256!(1E-30000) / udec256!(1E2768);
    /// let inf = UD256::INFINITY;
    /// let nan = UD256::NAN;
    /// let zero = UD256::ZERO;
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
    #[inline]
    pub const fn is_subnormal(self) -> bool {
        self.0.is_subnormal()
    }

    /// Return `true` if this number is neither [`Infinity`] nor [`NaN`] and
    /// `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{UD256, udec256};
    ///
    /// let d = udec256!(7.0);
    /// let inf = UD256::INFINITY;
    /// let nan = UD256::NAN;
    ///
    /// assert!(d.is_finite());
    ///
    /// assert!(!nan.is_finite());
    /// assert!(!inf.is_finite());
    /// ```
    ///
    /// [`Infinity`]: crate#special-values
    /// [`NaN`]: crate#special-values
    #[must_use]
    #[inline]
    pub const fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    /// Return `true` if this value is [`Infinity`] and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{UD256, udec256};
    ///
    /// let d = udec256!(7.0);
    /// let inf = UD256::INFINITY;
    /// let nan = UD256::NAN;
    ///
    /// assert!(inf.is_infinite());
    ///
    /// assert!(!d.is_infinite());
    /// assert!(!nan.is_infinite());
    /// ```
    ///
    /// [`Infinity`]: crate#special-values
    #[must_use]
    #[inline]
    pub const fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    /// Return `true` if this value is [`NaN`] and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{UD256, udec256};
    ///
    /// let nan = UD256::NAN;
    /// let d = udec256!(7.0);
    ///
    /// assert!(nan.is_nan());
    /// assert!(!d.is_nan());
    /// ```
    ///
    /// [`NaN`]: crate#special-values
    #[must_use]
    #[inline]
    pub const fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    /// Apply [Context] to the given decimal number.
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn with_ctx(mut self, ctx: Context) -> Self {
        self.0 = self.0.with_ctx(ctx);
        self
    }

    /// Apply [RoundingMode] to the given decimal number.
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn with_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.0 = self.0.with_rounding_mode(rm);
        self
    }

    /// _Deprecated_, use [`quantum`](Self::quantum) instead.
    #[must_use]
    #[deprecated(since = "0.1.2")]
    #[track_caller]
    #[inline]
    pub const fn from_scale(exp: i16) -> Self {
        Self::quantum(exp as i32, Context::default())
    }

    /// The quantum of a finite number is given by: 1 × 10<sup>exp</sup>.
    /// This is the value of a unit in the least significant position of the
    /// coefficient of a finite number.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// let ctx = Context::default();
    ///
    /// assert_eq!(UD256::quantum(0, ctx), udec256!(1));
    /// assert_eq!(UD256::quantum(-0, ctx), udec256!(1));
    /// assert_eq!(UD256::quantum(-3, ctx), udec256!(0.001));
    /// assert_eq!(UD256::quantum(3, ctx), udec256!(1000));
    /// ```
    #[must_use]
    #[track_caller]
    #[inline]
    pub const fn quantum(exp: i32, ctx: Context) -> Self {
        Self::new(Decimal::<N>::quantum(exp, ctx))
    }

    /// Reduces a decimal number to its shortest (coefficient)
    /// form shifting all significant trailing zeros into the exponent.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256, u256, decimal::Context};
    ///
    /// let a = udec256!(1234500);
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
        Self::new(self.0.reduce())
    }

    /// _Deprecated_, use [`reduce`](Self::reduce) instead.
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[deprecated(since = "0.1.4")]
    #[inline]
    pub const fn normalized(self) -> Self {
        self.reduce()
    }

    /// Invert sign of the given unsigned decimal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(udec256!(1.0).neg(), dec256!(-1.0));
    /// ```
    #[must_use]
    #[inline]
    pub const fn neg(self) -> Decimal<N> {
        self.0.neg()
    }

    /// Tests for `self` and `other` values to be equal, and is used by `==`
    /// operator.
    #[must_use]
    #[inline]
    pub const fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    /// Tests for `self` and `other` values to be equal, and is used by `==`
    /// operator.
    #[must_use]
    #[inline]
    pub const fn ne(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }

    /// Compares and returns the maximum of two unsigned decimal values.
    ///
    /// Returns the second argument if the comparison determines them to be
    /// equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256};
    ///
    /// assert_eq!(udec256!(1).max(udec256!(2)), udec256!(2));
    /// assert_eq!(udec256!(2).max(udec256!(2)), udec256!(2));
    /// ```
    #[must_use]
    #[inline]
    pub const fn max(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => other,
            _ => self,
        }
    }

    /// Compares and returns the minimum of two undecimal values.
    ///
    /// Returns the first argument if the comparison determines them to be
    /// equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(1).min(udec256!(2)), udec256!(1));
    /// assert_eq!(udec256!(2).min(udec256!(2)), udec256!(2));
    /// ```
    #[must_use]
    #[inline]
    pub const fn min(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => self,
            _ => other,
        }
    }

    /// Restrict an unsigned decimal value to a certain interval.
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
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(0).clamp(udec256!(3), udec256!(5)), udec256!(3));
    /// assert_eq!(udec256!(3).clamp(udec256!(1), udec256!(5)), udec256!(3));
    /// assert_eq!(udec256!(6).clamp(udec256!(1), udec256!(5)), udec256!(5));
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

    /// Tests unsigned decimal `self` less than `other` and is used by the `<`
    /// operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(1.0).lt(&udec256!(1.0)), false);
    /// assert_eq!(udec256!(1.0).lt(&udec256!(2.0)), true);
    /// assert_eq!(udec256!(2.0).lt(&udec256!(1.0)), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn lt(&self, other: &Self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self.cmp(other) {
            Ordering::Less => true,
            _ => false,
        }
    }

    /// Tests unsigned decimal `self` less than or equal to `other` and is used
    /// by the `<=` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(1.0).le(&udec256!(1.0)), true);
    /// assert_eq!(udec256!(1.0).le(&udec256!(2.0)), true);
    /// assert_eq!(udec256!(2.0).le(&udec256!(1.0)), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn le(&self, other: &Self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self.cmp(other) {
            Ordering::Less | Ordering::Equal => true,
            _ => false,
        }
    }

    /// Tests unsigned decimal `self` greater than `other` and is used by the
    /// `>` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(1.0).gt(&udec256!(1.0)), false);
    /// assert_eq!(udec256!(1.0).gt(&udec256!(2.0)), false);
    /// assert_eq!(udec256!(2.0).gt(&udec256!(1.0)), true);
    /// ```
    #[must_use]
    #[inline]
    pub const fn gt(&self, other: &Self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self.cmp(other) {
            Ordering::Greater => true,
            _ => false,
        }
    }

    /// Tests unsigned decimal `self` greater than or equal to `other` and is
    /// used by the `>=` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(1.0).ge(&udec256!(1.0)), true);
    /// assert_eq!(udec256!(1.0).ge(&udec256!(2.0)), false);
    /// assert_eq!(udec256!(2.0).ge(&udec256!(1.0)), true);
    /// ```
    #[must_use]
    #[inline]
    pub const fn ge(&self, other: &Self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self.cmp(other) {
            Ordering::Greater | Ordering::Equal => true,
            _ => false,
        }
    }

    /// This method returns an [`Ordering`] between `self` and `other`.
    ///
    /// By convention, `self.cmp(&other)` returns the ordering matching the
    /// expression `self <operator> other` if true.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(udec256!(5).cmp(&udec256!(10)), Ordering::Less);
    /// assert_eq!(udec256!(10).cmp(&udec256!(5)), Ordering::Greater);
    /// assert_eq!(udec256!(5).cmp(&udec256!(5)), Ordering::Equal);
    /// ```
    #[must_use]
    #[inline]
    pub const fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }

    /// Calculates `self` + `rhs`.
    ///
    /// Is internally used by the `+` operator.
    #[doc = doc::decimal_operation_panics!("addition operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = UD256::ONE;
    /// let b = UD256::TWO;
    ///
    /// let c = a + b;
    /// assert_eq!(c, udec256!(3));
    /// ```
    ///
    /// Panics if overflowed:
    ///
    /// ```should_panic
    /// use fastnum::*;
    ///
    /// let a = UD256::MAX;
    /// let b = UD256::MAX;
    ///
    /// let c = a + b;
    /// ```
    /// See more about [add and subtract](crate#addition-and-subtraction).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn add(self, rhs: Self) -> Self {
        Self::new(self.0.add(rhs.0))
    }

    /// Calculates `self` – `rhs`.
    ///
    /// Is internally used by the `-` operator.
    #[doc = doc::decimal_operation_panics!("subtract operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = UD256::FIVE;
    /// let b = UD256::TWO;
    ///
    /// let c = a - b;
    /// assert_eq!(c, udec256!(3));
    /// ```
    ///
    /// Panics if overflowed:
    ///
    /// ```should_panic
    /// use fastnum::*;
    ///
    /// let a = UD256::ZERO;
    /// let b = UD256::ONE;
    ///
    /// let c = a - b;
    /// ```
    /// See more about [add and subtract](crate#addition-and-subtraction).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn sub(self, rhs: Self) -> Self {
        let res = self.0.sub(rhs.0);
        Self::from_signed(res)
    }

    /// Calculates `self` × `rhs`.
    ///
    /// Is internally used by the `*` operator.
    #[doc = doc::decimal_operation_panics!("multiplication operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = UD256::FIVE;
    /// let b = UD256::TWO;
    ///
    /// let c = a * b;
    /// assert_eq!(c, udec256!(10));
    /// ```
    ///
    /// Panics if overflowed:
    ///
    /// ```should_panic
    /// use fastnum::*;
    ///
    /// let a = UD256::MAX;
    /// let b = UD256::MAX;
    ///
    /// let c = a * b;
    /// ```
    ///
    /// See more about [multiplication](crate#multiplication).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn mul(self, rhs: Self) -> Self {
        Self::new(self.0.mul(rhs.0))
    }

    /// Calculates `self` ÷ `rhs`.
    ///
    /// Is internally used by the `/` operator.
    #[doc = doc::decimal_operation_panics!("divide operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = UD256::FIVE;
    /// let b = UD256::TWO;
    ///
    /// let c = a / b;
    /// assert_eq!(c, udec256!(2.5));
    /// ```
    ///
    /// Panics if divided by zero:
    ///
    /// ```should_panic
    /// use fastnum::*;
    ///
    /// let a = UD256::ONE;
    /// let b = UD256::ZERO;
    ///
    /// let c = a / b;
    /// ```
    ///
    /// See more about [division](crate#division).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn div(self, rhs: Self) -> Self {
        Self::new(self.0.div(rhs.0))
    }

    /// Calculates `self` % `rhs`.
    ///
    /// Is internally used by the `%` operator.
    #[doc = doc::decimal_operation_panics!("reminder operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let a = UD256::FIVE;
    /// let b = UD256::TWO;
    ///
    /// let c = a % b;
    /// assert_eq!(c, udec256!(1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn rem(self, rhs: Self) -> Self {
        Self::new(self.0.rem(rhs.0))
    }

    /// Takes the reciprocal (inverse) of a number, `1/x`.
    #[doc = doc::decimal_operation_panics!("reciprocal operation")]
    #[doc = doc::decimal_inexact!("reciprocal")]
    /// # Examples
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(udec256!(2).recip(), udec256!(0.5));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn recip(self) -> Self {
        Self::new(self.0.recip())
    }

    /// Raise an unsigned decimal number to decimal power.
    ///
    /// Using this function is generally slower than using `powi` for integer
    /// exponents or `sqrt` method for `1/2` exponent.
    #[doc = doc::decimal_operation_panics!("power operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(udec256!(4).pow(dec256!(0.5)), udec256!(2));
    /// assert_eq!(udec256!(8).pow(dec256!(1) / dec256!(3)), udec256!(2));
    /// ```
    ///
    /// See more about the [power](crate#power) operation.
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn pow(self, n: Decimal<N>) -> Self {
        Self::new(self.0.pow(n))
    }

    /// Raise an unsigned decimal number to an integer power.
    ///
    /// Using this function is generally faster than using `pow`
    #[doc = doc::decimal_operation_panics!("power operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(udec256!(2).powi(3), udec256!(8));
    /// assert_eq!(udec256!(9).powi(2), udec256!(81));
    /// assert_eq!(udec256!(1).powi(-2), udec256!(1));
    /// assert_eq!(udec256!(10).powi(20), udec256!(1e20));
    /// assert_eq!(udec256!(4).powi(-2), udec256!(0.0625));
    /// ```
    ///
    /// See more about the [power](crate#power) operation.
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn powi(self, n: i32) -> Self {
        Self::new(self.0.powi(n))
    }

    /// Take the square root of the unsigned decimal number.
    ///
    /// Square-root can also be calculated by using the `power` operation (with
    /// a second operand of `0.5`). The result in that case will not be exact
    /// and may not be correctly rounded.
    #[doc = doc::decimal_operation_panics!("sqrt operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(udec128!(4).sqrt(), udec128!(2));
    /// assert_eq!(udec128!(1).sqrt(), udec128!(1));
    /// assert_eq!(udec128!(16).sqrt(), udec128!(4));
    /// ```
    ///
    /// See more about the [square-root](crate#square-root) operation.
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn sqrt(self) -> Self {
        Self::new(self.0.sqrt())
    }

    /// Returns _e<sup>self</sup>_, (the exponential function).
    #[doc = doc::decimal_operation_panics!("exponent operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(udec128!(1).exp(), UD128::E);
    /// ```
    ///
    /// See more about the [exponential function](crate#exponential-function).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn exp(self) -> Self {
        Self::new(self.0.exp())
    }

    /// Returns the natural logarithm of the unsigned decimal number.
    #[doc = doc::decimal_operation_panics!("logarithm operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(udec256!(2).ln(), D256::LN_2);
    /// ```
    ///
    /// See more about the [logarithm function](crate#logarithm-function).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn ln(self) -> Decimal<N> {
        self.0.ln()
    }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    #[doc = doc::decimal_operation_panics!("multiply-add operation")]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// assert_eq!(udec128!(10.0).mul_add(udec128!(4.0), udec128!(60)), udec128!(100));
    /// ```
    ///
    /// See more about the [fused multiply-add function](crate#multiply-add).
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn mul_add(self, a: Self, b: Self) -> Self {
        Self::new(self.0.mul_add(a.0, b.0))
    }

    /// Returns the given decimal number rounded to `digits` precision after the
    /// decimal point, using [RoundingMode] from it [Context].
    #[doc = doc::decimal_operation_panics!("round operation (up-scale or down-scale)")]
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::{*, RoundingMode::*}};
    ///
    /// let n = udec256!(129.41675);
    ///
    /// // Default rounding mode is `HalfUp`
    /// assert_eq!(n.round(2),  udec256!(129.42));
    ///
    /// assert_eq!(n.with_rounding_mode(Up).round(2), udec256!(129.42));
    /// assert_eq!(n.with_rounding_mode(Down).round(-1), udec256!(120));
    /// assert_eq!(n.with_rounding_mode(HalfEven).round(4), udec256!(129.4168));
    /// ```
    /// See also:
    /// - More about [`round`](crate#rounding) decimals.
    /// - [RoundingMode].
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn round(self, digits: i16) -> Self {
        Self::new(self.0.round(digits))
    }

    /// _Deprecated_, use [`rescale`](Self::rescale) instead.
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    #[deprecated(since = "0.1.4")]
    pub const fn with_scale(self, new_scale: i16) -> Self {
        Self::rescale(self, new_scale)
    }

    /// Returns the given decimal number _re-scaled_ to `digits` precision after
    /// the decimal point.
    #[doc = doc::decimal_operation_panics!("rescale operation")]
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// assert_eq!(udec256!(2.17).rescale(3), udec256!(2.170));
    /// assert_eq!(udec256!(2.17).rescale(2), udec256!(2.17));
    /// assert_eq!(udec256!(2.17).rescale(1), udec256!(2.2));
    /// assert_eq!(udec256!(2.17).rescale(0), udec256!(2));
    /// assert_eq!(udec256!(2.17).rescale(-1), udec256!(0));
    ///
    /// let ctx = Context::default().without_traps();
    ///
    /// assert!(UD256::INFINITY.with_ctx(ctx).rescale(2).is_nan());
    /// assert!(UD256::NAN.with_ctx(ctx).rescale(1).is_nan());
    /// ```
    ///
    /// See also:
    /// - More about [`rescale`](crate#rescale) decimals.
    /// - [Self::quantize].
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn rescale(mut self, new_scale: i16) -> Self {
        self.0 = self.0.rescale(new_scale);
        self
    }

    /// Returns a value equal to `self` (rounded), having the exponent of
    /// `other`.
    #[doc = doc::decimal_operation_panics!("quantize operation")]
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// let ctx = Context::default().without_traps();
    ///
    /// assert_eq!(udec256!(2.17).quantize(udec256!(0.001)), udec256!(2.170));
    /// assert_eq!(udec256!(2.17).quantize(udec256!(0.01)), udec256!(2.17));
    /// assert_eq!(udec256!(2.17).quantize(udec256!(0.1)), udec256!(2.2));
    /// assert_eq!(udec256!(2.17).quantize(udec256!(1e+0)), udec256!(2));
    /// assert_eq!(udec256!(2.17).quantize(udec256!(1e+1)), udec256!(0));
    ///
    /// assert_eq!(UD256::INFINITY.quantize(UD256::INFINITY), UD256::INFINITY);
    ///
    /// assert!(udec256!(2).with_ctx(ctx).quantize(UD256::INFINITY).is_nan());
    ///
    /// assert_eq!(udec256!(0.1).quantize(udec256!(1)), udec256!(0));
    /// assert_eq!(udec256!(0).quantize(udec256!(1e+5)), udec256!(0E+5));
    ///
    /// assert!(udec128!(0.34028).with_ctx(ctx).quantize(udec128!(1e-32765)).is_nan());
    ///
    /// assert_eq!(udec256!(217).quantize(udec256!(1e-1)), udec256!(217.0));
    /// assert_eq!(udec256!(217).quantize(udec256!(1e+0)), udec256!(217));
    /// assert_eq!(udec256!(217).quantize(udec256!(1e+1)), udec256!(2.2E+2));
    /// assert_eq!(udec256!(217).quantize(udec256!(1e+2)), udec256!(2E+2));
    /// ```
    ///
    /// See also:
    /// - More about [`quantize`](crate#quantize) decimals.
    /// - [Self::rescale].
    #[must_use = doc::must_use_op!()]
    #[track_caller]
    #[inline]
    pub const fn quantize(mut self, other: Self) -> Self {
        self.0 = self.0.quantize(other.0);
        self
    }

    /// Returns:
    /// - `true` if no [Exceptional condition] [Signal] flag has been trapped by
    ///   [Context] trap-enabler, and
    /// - `false` otherwise.
    ///
    /// [Exceptional condition]: crate#signaling-flags-and-trap-enablers
    #[inline(always)]
    pub const fn is_ok(&self) -> bool {
        self.0.is_ok()
    }

    /// Returns:
    /// - `Some(Self)` if no [Exceptional condition] [Signal] flag has been
    ///   trapped by [Context] trap-enabler, and
    /// - `None` otherwise.
    ///
    /// [Exceptional condition]: crate#signaling-flags-and-trap-enablers
    #[must_use]
    #[inline]
    pub const fn ok(self) -> Option<Self> {
        if self.is_ok() {
            None
        } else {
            Some(self)
        }
    }

    /// Create a string of this unsigned decimal in scientific notation.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// let n = udec256!(12345678);
    /// assert_eq!(&n.to_scientific_notation(), "1.2345678e7");
    /// ```
    #[inline]
    pub fn to_scientific_notation(&self) -> String {
        self.0.to_scientific_notation()
    }

    /// Create a string of this unsigned decimal in engineering notation.
    ///
    /// Engineering notation is scientific notation with the exponent
    /// coerced to a multiple of three.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// let n = udec256!(12345678);
    /// assert_eq!(&n.to_engineering_notation(), "12.345678e6");
    /// ```
    #[inline]
    pub fn to_engineering_notation(&self) -> String {
        self.0.to_engineering_notation()
    }
}

#[doc(hidden)]
impl<const N: usize> UnsignedDecimal<N> {
    #[inline(always)]
    pub(crate) const fn new(dec: Decimal<N>) -> Self {
        debug_assert!(!dec.is_negative());
        Self(dec)
    }

    #[inline]
    pub(crate) const fn from_signed(mut dec: Decimal<N>) -> Self {
        if dec.is_negative() {
            dec = dec.signaling_nan().check();
        }

        Self::new(dec)
    }

    #[inline]
    pub(crate) fn type_name() -> String {
        format!("UD{}", N * 64)
    }

    #[inline(always)]
    pub(crate) const fn flags(&self) -> Flags {
        self.0.flags()
    }

    #[inline(always)]
    pub(crate) const fn signals(&self) -> Signal {
        self.0.signals()
    }

    #[inline(always)]
    pub(crate) const fn context(&self) -> Context {
        self.0.context()
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) const fn ok_or_err(self) -> Result<Self, DecimalError> {
        match self.0.ok_or_err() {
            Ok(ok) => Ok(Self::new(ok)),
            Err(e) => Err(e),
        }
    }
}
