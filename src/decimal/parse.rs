use std::str::from_utf8_unchecked;

use crate::decimal::{Decimal, ParseError};
use crate::u256::U256;
use crate::Sign;

type Digit = u64;

const RADIX: u8 = 10;
const POWER: usize = 19;

/// Creates and initializes a Decimal from string.
#[inline]
pub const fn parse_str(s: &str) -> Decimal {
    match from_str(s) {
        Ok(n) => n,
        Err(e) => panic!("{}", e.description()),
    }
}

/// Creates and initializes a Decimal from string.
pub const fn from_str(s: &str) -> Result<Decimal, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    let buf = s.as_bytes();
    let len = buf.len();
    
    let mut sign = Sign::NoSign;

    let mut decimal_offset: Option<i64> = None;
    let mut exponent_value: Option<i64> = None;
    let mut dot = None;

    let mut is_first_digit = true;

    // Digits are stored as little endian (the least significant digit is first).
    let mut value = U256::ZERO;

    let mut i = 0;

    match buf[i] {
        b'+' => {
            sign = Sign::Plus;
            i = 1;
        }
        b'-' => {
            sign = Sign::Minus;
            i = 1;
        }
        _ => {}
    }

    if i == 1 && len == 1 {
        return Err(ParseError::Empty);
    }

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
            value = U256::from_digit(n);
            is_first_digit = false;
        } else if digits_count > 0 {
            let multiplier = U256::from_digit(base(digits_count as u64));
            let Some(v) = value.checked_mul(multiplier) else {
                return Err(ParseError::PosOverflow);
            };

            let next = U256::from_digit(n);
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

    Ok(Decimal::new(value, sign, scale))
}

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
        9 =>  1000000000,
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