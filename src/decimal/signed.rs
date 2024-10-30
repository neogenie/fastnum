mod impls;
mod sign;
mod extras;

pub use sign::Sign;

pub mod parse;

use crate::{U128, U256, U512};
use crate::decimal::unsigned::UnsignedDecimal;

/// Decimal
#[derive(Copy, Clone)]
pub struct Decimal<UINT> {
    /// A 256-bit decimal.
    value: UnsignedDecimal<UINT>,

    /// Sign
    sign: Sign,
}

impl<UINT> Decimal<UINT> {
    /// Creates and initializes a `Decimal`.
    ///
    #[inline]
    pub(crate) const fn new(value: UnsignedDecimal<UINT>, sign: Sign) -> Self {
        Self { value, sign }
    }

    /// Returns the scale of the `Decimal`, the total number of
    /// digits to the right of the decimal point (including insignificant
    /// leading zeros).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    /// use std::str::FromStr;
    ///
    /// let a = dec256!(12345);  // No fractional part
    /// let b = dec256!(123.45);  // Fractional part
    /// let c = dec256!(0.0000012345);  // Completely fractional part
    /// let d = dec256!(500000000);  // Negative-fractional part
    ///
    /// assert_eq!(a.fractional_digit_count(), 0);
    /// assert_eq!(b.fractional_digit_count(), 2);
    /// assert_eq!(c.fractional_digit_count(), 10);
    /// assert_eq!(d.fractional_digit_count(), -9);
    /// ```
    #[inline]
    pub const fn fractional_digit_count(&self) -> i64 {
        self.value.fractional_digit_count()
    }

    /// Return the sign of the `Decimal` as [Sign].
    ///
    /// ```
    /// use fastnum::{decimal::signed::Sign, dec256};
    ///
    /// assert_eq!(dec256!(-1).sign(), Sign::Minus);
    /// assert_eq!(dec256!(0).sign(),  Sign::NoSign);
    /// assert_eq!(dec256!(1).sign(),  Sign::Plus);
    /// ```
    #[inline]
    pub const fn sign(&self) -> Sign {
        self.sign
    }
}

impl<UINT: Copy> Decimal<UINT> {   
    #[inline]
    pub const fn negative(self) -> Self {
        Self::new(self.value, self.sign.not())
    }
}

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal) => {
        /// Unsigned decimal number with $bits-bit integer for significant digits.
        impl Decimal<$UINT> {
            /// A constant `Decimal` with value `0`, useful for static initialization.
            pub const ZERO: Self = Self::new(UnsignedDecimal::<$UINT>::ZERO, Sign::NoSign);
        
            /// A constant `Decimal` with value `1`, useful for static initialization.
            pub const ONE: Self = Self::new(UnsignedDecimal::<$UINT>::ONE, Sign::NoSign);
        
            pub const TEN: Self = Self::new(UnsignedDecimal::<$UINT>::TEN, Sign::NoSign);
            
            #[inline]
            pub const fn from_scale(scale: i64) -> Self {
                Self::new(UnsignedDecimal::<$UINT>::from_scale(scale), Sign::NoSign)
            }
        
            #[inline]
            pub const fn normalized(self) -> Self {
                Self::new(UnsignedDecimal::<$UINT>::normalized(self.value), self.sign)
            }
        }
    }
}

macro_impl!(U128, 128);
macro_impl!(U256, 256);
macro_impl!(U512, 512);

#[cfg(feature = "test-util")]
impl<UINT: Copy> Decimal<UINT> {
    #[inline]
    pub const fn significant_digits(&self) -> UINT {
        self.value.significant_digits()
    }
}
