mod extras;
mod format;
mod impls;

pub mod parse;

use std::fmt;

use crate::{U128, U256, U512};

/// Unsigned N-bits decimal number.
#[derive(Copy, Clone)]
pub struct UnsignedDecimal<UINT> {
    /// Unsigned integer for significant digits of a decimal number.
    value: UINT,

    /// A positive scale means a negative power of 10.
    scale: i64,
}

impl<UINT> UnsignedDecimal<UINT> {
    /// Creates and initializes a `UnsignedDecimal`.
    ///
    #[inline]
    pub(crate) const fn new(value: UINT, scale: i64) -> Self {
        Self { value, scale }
    }

    /// Returns the scale of the `UnsignedDecimal`, the total number of
    /// digits to the right of the decimal point (including insignificant
    /// leading zeros).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    /// use std::str::FromStr;
    ///
    /// let a = udec256!(12345);  // No fractional part
    /// let b = udec256!(123.45);  // Fractional part
    /// let c = udec256!(0.0000012345);  // Completely fractional part
    /// let d = udec256!(500000000);  // Negative-fractional part
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
}

impl<UINT: Copy> UnsignedDecimal<UINT> {
    #[inline]
    pub const fn significant_digits(&self) -> UINT {
        self.value
    }
}

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal) => {
        /// Unsigned decimal number with $bits-bit integer for significant digits.
        impl UnsignedDecimal<$UINT> {
            /// A constant `UnsignedDecimal` with value `0`, useful for static initialization.
            pub const ZERO: Self = Self::new($UINT::ZERO, 0);

            // #[doc = doc::consts::min!(U 512)]
            pub const MIN: Self = Self::new($UINT::MIN, 0);

            // #[doc = doc::consts::max!(U 512)]
            pub const MAX: Self = Self::new($UINT::MAX, i64::MIN);

            /// A constant `UnsignedDecimal` with value `1`, useful for static initialization.
            pub const ONE: Self = Self::new($UINT::ONE, 0);

            pub const TEN: Self = Self::new($UINT::TEN, 0);

            /// Return if the referenced decimal is zero
            pub const fn is_zero(&self) -> bool {
                self.value.is_zero()
            }

            #[inline]
            pub const fn from_scale(scale: i64) -> Self {
                Self::new($UINT::ONE, -scale)
            }

            pub const fn normalized(mut self) -> Self {
                if self.value.is_zero() {
                    self.scale = 0;
                } else {
                    while !self.value.is_zero() && self.value.rem($UINT::TEN).is_zero() {
                        self.value = self.value.div($UINT::TEN);
                        self.scale -= 1;
                    }
                }
                self
            }

            /// Create string of this unsigned decimal in scientific notation
            ///
            /// ```
            /// use fastnum::udec256;
            ///
            /// let n = udec256!(12345678);
            /// assert_eq!(&n.to_scientific_notation(), "1.2345678e7");
            /// ```
            #[inline]
            pub fn to_scientific_notation(&self) -> String {
                let mut output = String::new();
                self.write_scientific_notation(&mut output)
                    .expect("Could not write to string");
                output
            }

            /// Write bigdecimal in scientific notation to writer `w`
            #[inline]
            pub(crate) fn write_scientific_notation<W: fmt::Write>(
                &self,
                w: &mut W,
            ) -> fmt::Result {
                if self.is_zero() {
                    return w.write_str("0e0");
                }
                let digits = self.value.to_str_radix(10);
                let scale = self.scale;
                format::write_scientific_notation(digits, scale, w)
            }

            /// Create string of this bigdecimal in engineering notation
            ///
            /// Engineering notation is scientific notation with the exponent
            /// coerced to a multiple of three
            ///
            /// ```
            /// use fastnum::udec256;
            /// let n = udec256!(12345678);
            /// assert_eq!(&n.to_engineering_notation(), "12.345678e6");
            /// ```
            ///
            #[inline]
            pub fn to_engineering_notation(&self) -> String {
                let mut output = String::new();
                self.write_engineering_notation(&mut output)
                    .expect("Could not write to string");
                output
            }

            /// Write bigdecimal in engineering notation to writer `w`
            #[inline]
            pub(crate) fn write_engineering_notation<W: fmt::Write>(
                &self,
                w: &mut W,
            ) -> fmt::Result {
                if self.is_zero() {
                    return w.write_str("0e0");
                }
                let digits = self.value.to_str_radix(10);
                let scale = self.scale;
                format::write_engineering_notation(digits, scale, w)
            }
        }

        impl Default for UnsignedDecimal<$UINT> {
            #[inline]
            fn default() -> Self {
                Self::ZERO
            }
        }
    };
}

macro_impl!(U128, 128);
macro_impl!(U256, 256);
macro_impl!(U512, 512);
