//! Fast Decimal arbitrary precision library
//!
//! [Decimal] allows storing real number to arbitrary precision; which
//! avoids common floating point errors (such as 0.1 + 0.2 ≠ 0.3) at the
//! cost of complexity.
//!
//! Internally, `Decimal` uses a unsigned 256-bit integer, paired with a signed 64-bit
//! integer which determines the position of the decimal point. Therefore,
//! the precision *is not* actually arbitrary, but limited to 2<sup>63</sup>
//! decimal places.
//!
//! Common numerical operations are overloaded, so we can treat them
//! the same way we treat other numbers.
//!
//! It is not recommended to convert a floating point number to a decimal
//! directly, as the floating point representation may be unexpected.
//!
//! # Example
//!
//! ```
//! use fastnum::decimal::Decimal;
//! use std::str::FromStr;
//!
//! let input = "0.8";
//! let dec = Decimal::from_str(&input).unwrap();
//! let float = f32::from_str(&input).unwrap();
//!
//! println!("Input ({}) with decimals: {} vs {})", input, dec, float);
//! ```

// mod extras;
mod error;
mod impls;
mod parse;
mod utils;

pub use error::ParseError;
pub use parse::parse_str;

#[macro_use]
mod macros;

use crate::u256::U256;
use crate::Sign;

/// Decimal
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Decimal {
    /// A 256-bit unsigned integer.
    value: U256,

    /// Sign
    sign: Sign,

    /// A positive scale means a negative power of 10.
    scale: i64,
}

impl Decimal {
    /// A constant `Decimal` with value `0`, useful for static initialization.
    pub const ZERO: Self = Self::new(U256::ZERO, Sign::NoSign, 0);

    /// A constant `Decimal` with value `1`, useful for static initialization.
    pub const ONE: Self = Self::new(U256::ONE, Sign::NoSign, 0);

    pub const TEN: Self = Self::new(U256::TEN, Sign::NoSign, 0);

    /// Creates and initializes a `Decimal`.
    ///
    #[inline]
    pub(crate) const fn new(value: U256, sign: Sign, scale: i64) -> Self {
        Self { value, sign, scale }
    }

    /// Returns the scale of the `Decimal`, the total number of
    /// digits to the right of the decimal point (including insignificant
    /// leading zeros).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::decimal;
    /// use std::str::FromStr;
    ///
    /// let a = decimal!(12345);  // No fractional part
    /// let b = decimal!(123.45);  // Fractional part
    /// let c = decimal!(0.0000012345);  // Completely fractional part
    /// let d = decimal!(500000000);  // Negative-fractional part
    ///
    /// assert_eq!(a.fractional_digit_count(), 0);
    /// assert_eq!(b.fractional_digit_count(), 2);
    /// assert_eq!(c.fractional_digit_count(), 10);
    /// assert_eq!(d.fractional_digit_count(), -9);
    /// ```
    #[inline]
    pub const fn fractional_digit_count(&self) -> i64 {
        self.scale
    }

    /// Return the sign of the `Decimal` as [Sign].
    ///
    /// ```
    /// use fastnum::{Sign, decimal};
    ///
    /// assert_eq!(decimal!(-1).sign(), Sign::Minus);
    /// assert_eq!(decimal!(0).sign(),  Sign::NoSign);
    /// assert_eq!(decimal!(1).sign(),  Sign::Plus);
    /// ```
    #[inline]
    pub const fn sign(&self) -> Sign {
        self.sign
    }

    #[inline]
    pub const fn from_scale(scale: i64) -> Self {
        Self::new(U256::ONE, Sign::NoSign, -scale)
    }

    pub const fn normalized(mut self) -> Decimal {
        while !self.value.is_zero() && self.value.rem(U256::TEN).is_zero() {
            self.value = self.value.div(U256::TEN);
            self.scale -= 1;
        }
        self
    }
}

#[cfg(feature = "test-util")]
impl Decimal {
    pub const fn _integer_digits(&self) -> U256 {
        self.value
    }
}

impl Default for Decimal {
    #[inline]
    fn default() -> Decimal {
        Self::ZERO
    }
}
