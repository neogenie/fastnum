use crate::decimal::{Decimal, UnsignedDecimal};

const MAX_RUST_DECIMAL_MANTISSA: u128 = (1u128 << 96) - 1;

/// Core conversion logic that extracts the mantissa and scale from fastnum internal representation
/// and converts it to rust_decimal format.
const fn convert_to_rust_decimal_core<const N: usize>(
    fast_digits_bnum: &crate::int::UInt<N>,
    fn_scale: i16,
    is_negative: bool,
    is_finite: bool,
) -> rust_decimal::Decimal {
    if !is_finite {
        panic!("Cannot convert non-finite value (NaN or Infinity) to rust_decimal::Decimal at compile time");
    }

    let limbs = fast_digits_bnum.digits();

    let initial_mantissa_u128: u128 = if N == 0 {
        0
    } else if N == 1 {
        limbs[0] as u128
    } else if N == 2 {
        (limbs[1] as u128) << 64 | (limbs[0] as u128)
    } else {
        let mut i = 2;
        while i < N {
            if limbs[i] != 0 {
                panic!("Mantissa too large for rust_decimal::Decimal");
            }
            i += 1;
        }
        // If all higher limbs are zero, construct from the first two limbs
        (limbs[1] as u128) << 64 | (limbs[0] as u128)
    };

    let mut working_fn_scale = fn_scale;
    let mut working_mantissa_u128 = initial_mantissa_u128;

    // rust_decimal::Decimal stores m / 10^e where e is 0-28.
    // fastnum::Decimal stores digits * 10^-scale.
    // If fastnum scale is negative, e.g., -2 for 12300 (digits=123, scale=-2),
    // it means the number is digits * 10^abs(scale).
    // So, rust_decimal mantissa should be fastnum_digits * 10^abs(fn_scale)
    // and rust_decimal scale should be 0.
    if working_fn_scale < 0 {
        let positive_scale_factor = -working_fn_scale as u32;
        if positive_scale_factor > 0 {
            let mut multiplier: u128 = 1;
            let mut current_power: u32 = 0;
            while current_power < positive_scale_factor {
                if multiplier > u128::MAX / 10 {
                    panic!("Mantissa overflow when adjusting for negative scale (multiplier too large)");
                }
                multiplier *= 10;
                current_power += 1;
            }
            if initial_mantissa_u128 > u128::MAX / multiplier {
                panic!("Mantissa overflow when adjusting for negative scale (multiplication)");
            }
            working_mantissa_u128 = initial_mantissa_u128 * multiplier;
        }
        working_fn_scale = 0;
    }

    let rd_scale = working_fn_scale as u32;

    if rd_scale > rust_decimal::Decimal::MAX_SCALE {
        panic!("Scale out of bounds for rust_decimal::Decimal");
    }

    if working_mantissa_u128 > MAX_RUST_DECIMAL_MANTISSA {
        panic!("Mantissa too large for rust_decimal::Decimal");
    }

    let lo = (working_mantissa_u128 & 0xFFFFFFFF) as u32;
    let mid = ((working_mantissa_u128 >> 32) & 0xFFFFFFFF) as u32;
    let hi = ((working_mantissa_u128 >> 64) & 0xFFFFFFFF) as u32;

    rust_decimal::Decimal::from_parts(lo, mid, hi, is_negative, rd_scale)
}

impl<const N: usize> Decimal<N> {
    /// Converts this `fastnum::Decimal` to a `rust_decimal::Decimal` at compile time.
    ///
    /// This function will panic during compilation if the conversion is not possible,
    /// for example, if the value is too large to be represented by `rust_decimal::Decimal`
    /// or the scale is out of bounds.
    ///
    /// Requires the `rust_decimal` feature to be enabled.
    ///
    /// # Panics
    ///
    /// Panics at compile-time if:
    /// - The `Decimal` is NaN or Infinity.
    /// - The number of digits in the coefficient (after adjusting for negative scale) exceeds
    ///   the capacity of `rust_decimal::Decimal` (approximately 28-29 digits or 96 bits).
    /// - The scale is outside the range supported by `rust_decimal::Decimal` (0-28).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{dec128, D128};
    ///
    /// const FASTNUM_VAL: D128 = dec128!(0.27);
    /// const RUST_DEC_VAL: rust_decimal::Decimal = FASTNUM_VAL.to_rust_decimal();
    /// assert_eq!(RUST_DEC_VAL, rust_decimal::Decimal::new(27, 2));
    ///
    /// const FASTNUM_NEG_SCALE: D128 = dec128!(12300);
    /// const RUST_DEC_NEG_SCALE: rust_decimal::Decimal = FASTNUM_NEG_SCALE.to_rust_decimal();
    /// assert_eq!(RUST_DEC_NEG_SCALE, rust_decimal::Decimal::new(12300, 0));
    /// ```
    pub const fn to_rust_decimal(&self) -> rust_decimal::Decimal {
        let fast_digits_bnum = self.digits();
        let fn_scale = self.fractional_digits_count();
        let is_negative = self.is_negative();
        let is_finite = self.is_finite();

        convert_to_rust_decimal_core(&fast_digits_bnum, fn_scale, is_negative, is_finite)
    }
}

impl<const N: usize> UnsignedDecimal<N> {
    /// Converts this `fastnum::UnsignedDecimal` to a `rust_decimal::Decimal` at compile time.
    ///
    /// This function will panic during compilation if the conversion is not possible,
    /// for example, if the value is too large to be represented by `rust_decimal::Decimal`
    /// or the scale is out of bounds.
    ///
    /// Requires the `rust_decimal` feature to be enabled.
    ///
    /// # Panics
    ///
    /// Panics at compile-time if:
    /// - The `UnsignedDecimal` is NaN or Infinity.
    /// - The number of digits in the coefficient (after adjusting for negative scale) exceeds
    ///   the capacity of `rust_decimal::Decimal` (approximately 28-29 digits or 96 bits).
    /// - The scale is outside the range supported by `rust_decimal::Decimal` (0-28).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec128, UD128};
    ///
    /// const FASTNUM_VAL: UD128 = udec128!(0.27);
    /// const RUST_DEC_VAL: rust_decimal::Decimal = FASTNUM_VAL.to_rust_decimal();
    /// assert_eq!(RUST_DEC_VAL, rust_decimal::Decimal::new(27, 2));
    ///
    /// const FASTNUM_NEG_SCALE: UD128 = udec128!(12300);
    /// const RUST_DEC_NEG_SCALE: rust_decimal::Decimal = FASTNUM_NEG_SCALE.to_rust_decimal();
    /// assert_eq!(RUST_DEC_NEG_SCALE, rust_decimal::Decimal::new(12300, 0));
    /// ```
    pub const fn to_rust_decimal(&self) -> rust_decimal::Decimal {
        let fast_digits_bnum = self.digits();
        let fn_scale = self.fractional_digits_count();
        let is_negative = false;
        let is_finite = self.is_finite();

        convert_to_rust_decimal_core(&fast_digits_bnum, fn_scale, is_negative, is_finite)
    }
}
