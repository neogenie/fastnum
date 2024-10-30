
use std::str::from_utf8_unchecked;

use crate::decimal::ParseError;

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal, $name: ident) => {
        pub mod $name {
            use crate::decimal::{unsigned::UnsignedDecimal, ParseError};
            use crate::$UINT;

            use super::{
                base, make_scale, parse_exp, Digit, POWER, RADIX,
            };

            /// Creates and initializes a [crate::$UINT] from string.
            #[inline]
            pub const fn parse_str(s: &str) -> UnsignedDecimal<$UINT> {
                match from_str(s) {
                    Ok(n) => n,
                    Err(e) => panic!("{}", e.description()),
                }
            }

            /// Creates and initializes a Decimal from string.
            pub(crate) const fn from_str(s: &str) -> Result<UnsignedDecimal<$UINT>, ParseError> {
                if s.is_empty() {
                    return Err(ParseError::Empty);
                }

                let buf = s.as_bytes();
                let len = buf.len();

                let mut decimal_offset: Option<i64> = None;
                let mut exponent_value: Option<i64> = None;
                let mut dot = None;

                let mut is_first_digit = true;

                // Digits are stored as little endian (the least significant digit is first).
                let mut value = $UINT::ZERO;

                let mut i = 0;

                while i < len {
                    let mut digits_count = 0;
                    let mut n = 0;

                    while i < len && (digits_count < POWER) {
                        let b = buf[i];
                        let d = match b {
                            b'.' => {
                                if dot.is_some() {
                                    return Err(ParseError::InvalidLiteral);
                                } else {
                                    dot = Some(digits_count);
                                    i += 1;
                                    continue;
                                }
                            }
                            b'_' => {
                                i += 1;
                                continue;
                            }
                            b'E' | b'e' => {
                                if exponent_value.is_some() {
                                    return Err(ParseError::InvalidLiteral);
                                } else {
                                    match parse_exp(buf, i + 1) {
                                        Ok(exp) => {
                                            exponent_value = Some(exp);
                                        }
                                        Err(e) => {
                                            return Err(e);
                                        }
                                    }
                                    i = len;
                                    break;
                                }
                            }
                            b'0'..=b'9' => b - b'0',
                            _ => {
                                return Err(ParseError::InvalidLiteral);
                            }
                        };

                        n = n * (RADIX as Digit) + d as Digit;
                        digits_count += 1;
                        i += 1;
                    }

                    if is_first_digit {
                        if digits_count == 0 {
                            return Err(ParseError::Empty);
                        }
                        value = $UINT::from_digit(n);
                        is_first_digit = false;
                    } else if digits_count > 0 {
                        let multiplier = $UINT::from_digit(base(digits_count as u64));
                        let Some(v) = value.checked_mul(multiplier) else {
                            return Err(ParseError::PosOverflow);
                        };

                        let next = $UINT::from_digit(n);
                        let Some(v) = v.checked_add(next) else {
                            return Err(ParseError::PosOverflow);
                        };

                        value = v;
                    }

                    if let Some(current) = decimal_offset {
                        decimal_offset = Some(current + digits_count as i64);
                    } else if let Some(dot_pos) = dot {
                        decimal_offset = Some(digits_count as i64 - dot_pos as i64);
                    }
                }

                let scale = match make_scale(decimal_offset, exponent_value) {
                    Ok(scale) => scale,
                    Err(e) => {
                        return Err(e);
                    }
                };

                Ok(UnsignedDecimal::<$UINT>::new(value, scale))
            }
            
            macro_rules! from_float_impl {
                ($n: ident, $f: ty, $module: ident, $uint: ty) => {
                    pub(crate) const fn $n(n: $f) -> Result<UnsignedDecimal<$UINT>, ParseError> {
                        use crate::decimal::math::$module::*;
                        
                        #[inline]
                        const fn uint(digit: $uint) -> $UINT {
                            $UINT::from_digit(digit as u64)
                        }
        
                        if is_nan(n) {
                            return Err(ParseError::NaN);
                        }
        
                        let b = to_bits(n);
        
                        let sign = b & SIGN_MASK != 0;
        
                        if sign {
                            return Err(ParseError::Signed);
                        }
        
                        let frac = b & MAN_MASK;
                        let exp = b & EXP_MASK;
                        
                        if frac == 0 && exp == EXP_MASK {
                            return Err(ParseError::Infinite);
                        }
        
                        if frac == 0 && exp == 0 {
                            return Ok(UnsignedDecimal::<$UINT>::ZERO);
                        }
        
                        if exp == 0 {
                            // subnormal
                            // let frac = frac << 1;
                            
                            const POW: u32 = (MAX_EXP - 2) as u32 + (MANTISSA_DIGITS - 1);
                            const SUBNORMAL_BASE: ($UINT, bool) = $UINT::from_digit(5).overflowing_pow(POW);
                            
                            let subnormal_base = match SUBNORMAL_BASE {
                                (subnormal_base, false) => subnormal_base,
                                (_, true) => {
                                    return Err(ParseError::PosOverflow);
                                },
                            };
                            
                            let result = match subnormal_base.overflowing_mul(uint(frac)) {
                                (r, false) => r,
                                _ => {
                                    return Err(ParseError::PosOverflow);
                                }
                            };
                            let scale = POW as i64;
                            
                            Ok(UnsignedDecimal::new(result, scale))
                        } else {
                            // normal
                            let frac = frac | MAN_MASK_NORMAL;
                            let pow = (exp >> (MANTISSA_DIGITS - 1)) as i64 - (MAX_EXP - 1) - (MANTISSA_DIGITS - 1) as i64;
        
                            let result;
                            let scale;
                            
                            if pow == 0 {
                                result = uint(frac);
                                scale = 0;
                            } else if pow < 0 {
                                let mut trailing_zeros = frac.trailing_zeros();
                                if trailing_zeros > -pow as u32 {
                                    trailing_zeros = -pow as u32;
                                }
                                
                                let reduced_frac = frac >> trailing_zeros;
                                let reduced_pow = pow + trailing_zeros as i64;
        
                                let (shift, false) = $UINT::from_digit(5).overflowing_pow(-reduced_pow as u32) else {
                                    return Err(ParseError::PosOverflow);
                                };
        
                                result = match shift.overflowing_mul(uint(reduced_frac)) {
                                    (r, false) => r,
                                    _ => {
                                        return Err(ParseError::PosOverflow);
                                    }
                                };
                                scale = -reduced_pow;
                            } else {
                                let (shift, false) = $UINT::from_digit(2).overflowing_pow(pow as u32) else {
                                    return Err(ParseError::PosOverflow);
                                };
                                result = match shift.overflowing_mul(uint(frac)) {
                                    (r, false) => r,
                                    _ => {
                                        return Err(ParseError::PosOverflow);
                                    }
                                };
                                scale = 0;
                            }
                            
                            Ok(UnsignedDecimal::new(result, scale))
                        }
                    }
                }
            }
            
            from_float_impl!(from_f32, f32, f32, u32);
            from_float_impl!(from_f64, f64, f64, u64);
        }
    };
}

type Digit = u64;

const RADIX: u8 = 10;
const POWER: usize = 19;

macro_impl!(U128, 128, d128);
macro_impl!(U256, 256, d256);
macro_impl!(U512, 512, d512);

#[inline]
const fn make_scale(
    decimal_offset: Option<i64>,
    exponent_value: Option<i64>,
) -> Result<i64, ParseError> {
    match (decimal_offset, exponent_value) {
        (None, None) => Ok(0),
        (Some(decimal_offset), None) => Ok(decimal_offset),
        (None, Some(exp)) => match exp.checked_neg() {
            None => Err(ParseError::ExponentOverflow),
            Some(scale) => Ok(scale),
        },
        (Some(decimal_offset), Some(exp)) => match decimal_offset.checked_sub(exp) {
            None => Err(ParseError::ExponentOverflow),
            Some(scale) => Ok(scale),
        },
    }
}

#[inline]
const fn parse_exp(buf: &[u8], pos: usize) -> Result<i64, ParseError> {
    if pos >= buf.len() {
        return Err(ParseError::Empty);
    }

    let src = unsafe { from_utf8_unchecked(buf.split_at(pos).1) };

    match i64::from_str_radix(src, 10) {
        Ok(exp) => Ok(exp),
        Err(e) => {
            let e = ParseError::from_int_error_kind(e.kind());
            Err(match e {
                ParseError::PosOverflow => ParseError::ExponentOverflow,
                ParseError::NegOverflow => ParseError::ExponentOverflow,
                _ => e,
            })
        }
    }
}

#[inline]
const fn base(n: Digit) -> Digit {
    match n {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1000,
        4 => 10000,
        5 => 100000,
        6 => 1000000,
        7 => 10000000,
        8 => 100000000,
        9 => 1000000000,
        10 => 10000000000,
        11 => 100000000000,
        12 => 1000000000000,
        13 => 10000000000000,
        14 => 100000000000000,
        15 => 1000000000000000,
        16 => 10000000000000000,
        17 => 100000000000000000,
        18 => 1000000000000000000,
        19 => 10000000000000000000,
        _ => panic!("base number overflow"),
    }
}

