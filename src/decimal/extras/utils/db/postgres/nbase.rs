use core::cmp::Ordering;
use num_traits::Euclid;

use crate::{
    decimal::{dec::ControlBlock, Decimal, ParseError, Sign},
    int::{
        intrinsics::Intrinsics,
        math::{mul_div_rem_wide, to_i16},
        UInt,
    },
};

type D<const N: usize> = Decimal<N>;

pub enum NBase {
    /// A positive number
    Positive {
        /// How many digits come before the decimal point?
        weight: i16,
        /// How many significant digits are there?
        scale: u16,
        /// The digits in this number, stored in base 10,000
        digits: Vec<i16>,
    },
    /// A negative number
    Negative {
        /// How many digits come before the decimal point?
        weight: i16,
        /// How many significant digits are there?
        scale: u16,
        /// The digits in this number, stored in base 10,000
        digits: Vec<i16>,
    },
    /// Not a number
    NaN,
}

struct Consts<const N: usize>;

impl<const N: usize> Consts<N> {
    pub const NBASE: UInt<N> = UInt::<N>::from_digit(10_000);
}

macro_rules! checked {
    ($a: ident *= $b: expr) => {
        $a = $a.checked_mul($b).ok_or(ParseError::PosOverflow)?;
    };
    ($a: ident += $b: expr) => {
        $a = $a.checked_add($b).ok_or(ParseError::PosOverflow)?;
    };
    ($a: ident /= $b: expr) => {
        $a = $a.checked_div($b).ok_or(ParseError::PosOverflow)?;
    };
}

impl<const N: usize> TryFrom<D<N>> for NBase {
    type Error = ParseError;

    fn try_from(dec: D<N>) -> Result<Self, Self::Error> {
        if dec.is_nan() {
            return Ok(Self::NaN);
        }

        if dec.is_infinite() {
            return if dec.is_negative() {
                Err(ParseError::NegOverflow)
            } else {
                Err(ParseError::PosOverflow)
            };
        }

        let mut uint = dec.digits();

        if uint.is_zero() {
            return if dec.is_negative() {
                Ok(Self::Negative {
                    weight: 0,
                    scale: 0,
                    digits: vec![],
                })
            } else {
                Ok(Self::Positive {
                    weight: 0,
                    scale: 0,
                    digits: vec![],
                })
            };
        }

        let mut scale = dec.fractional_digits_count();
        let mut digits = Vec::with_capacity(0);
        let mut weight = 0;
        let mut exp = 0;

        if scale < 0 {
            (weight, exp) = (-scale).div_rem_euclid(&4);
            scale = 0;
        }

        exp += 4 - scale % 4;

        while !uint.is_zero() {
            let correction = Intrinsics::<N>::POWERS_OF_TEN.lookup(exp as u32);
            let (div, rem) = mul_div_rem_wide(uint, correction, Consts::<N>::NBASE);

            if !digits.is_empty() || !rem.is_zero() {
                digits.push(to_i16(rem).expect("10000 always fits in an i16"));
            }

            uint = div;
            exp = 0;
            weight += 1;
        }

        digits.reverse();

        let weight = weight - (scale / 4 + 1) - 1;

        if dec.is_negative() {
            Ok(Self::Negative {
                weight,
                scale: scale as u16,
                digits,
            })
        } else {
            Ok(Self::Positive {
                weight,
                scale: scale as u16,
                digits,
            })
        }
    }
}

impl<const N: usize> TryFrom<NBase> for D<N> {
    type Error = ParseError;

    fn try_from(value: NBase) -> Result<Self, Self::Error> {
        let (sign, weight, scale, digits) = match value {
            NBase::Positive {
                weight,
                scale,
                digits,
            } => (Sign::Plus, weight, scale, digits),
            NBase::Negative {
                weight,
                scale,
                digits,
            } => (Sign::Minus, weight, scale, digits),
            NBase::NaN => {
                return Ok(Self::NAN);
            }
        };

        let count = i16::try_from(digits.len()).map_err(|_| ParseError::PosOverflow)?;
        let scale = i16::try_from(scale).map_err(|_| ParseError::ExponentOverflow)?;

        let mut uint = UInt::<N>::ZERO;

        for digit in digits {
            let d = u64::try_from(digit).map_err(|_| ParseError::InvalidLiteral)?;
            checked!(uint *= Consts::<N>::NBASE);
            checked!(uint += UInt::<N>::from_digit(d));
        }

        let correction_exp = -(4 * (weight - (count - 1)));

        match scale.cmp(&correction_exp) {
            Ordering::Greater => {
                let scale_diff =
                    u32::try_from(scale - correction_exp).map_err(|_| ParseError::PosOverflow)?;
                let correction = UInt::<N>::TEN
                    .checked_pow(scale_diff)
                    .ok_or(ParseError::PosOverflow)?;
                checked!(uint *= correction);
            }
            Ordering::Less => {
                let scale_diff =
                    u32::try_from(correction_exp - scale).map_err(|_| ParseError::PosOverflow)?;
                let correction = UInt::<N>::TEN
                    .checked_pow(scale_diff)
                    .ok_or(ParseError::PosOverflow)?;
                checked!(uint /= correction);
            }
            Ordering::Equal => {}
        }

        Ok(D::new(uint, ControlBlock::basic(scale, sign)))
    }
}
