use core::str::from_utf8_unchecked;

use crate::{
    decimal::{
        dec::{construct::construct_with_clength, ControlBlock, ExtraPrecision},
        Context, Decimal, DecimalError, ParseError, Sign,
    },
    int::{
        intrinsics::{Digit, POWER},
        math::overflowing_mul10,
        UInt,
    },
};

/// Creates and initializes a Decimal from string.
#[inline]
pub const fn from_slice<const N: usize>(
    buf: &[u8],
    ctx: Context,
) -> Result<Decimal<N>, ParseError> {
    if buf.is_empty() {
        return Err(ParseError::Empty);
    }

    let len = buf.len();
    let mut cb = ControlBlock::default().set_context(ctx);

    let mut i = 0;

    // Parse sign or `NAN` special case
    match buf[0] {
        b'+' => {
            i = 1;
        }
        b'-' => {
            cb = cb.neg();
            i = 1;
        }
        b'n' | b'N' => {
            if bytes_equal_ci(buf.split_at(1).1, b"an") {
                return Ok(Decimal::NAN);
            }
        }
        _ => {}
    }

    if i == len {
        return Err(ParseError::Empty);
    }

    // Parse special cases Inf/Infinity
    if bytes_equal_ci(buf.split_at(i).1, b"inf") || bytes_equal_ci(buf.split_at(i).1, b"infinity") {
        return Ok(Decimal::INFINITY.with_cb(cb));
    }

    let mut clength = 0;
    let mut decimal_offset: Option<i32> = None;
    let mut exponent_value: Option<i32> = None;
    let mut dot = None;
    let mut ovf;

    let mut is_first_digit = true;

    // Digits are stored as little endian (the least significant digit is first).
    let mut value = UInt::ZERO;

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

            n = ((n + (n << 2)) << 1) + d as Digit;
            digits_count += 1;
            i += 1;
        }

        clength += digits_count;

        if is_first_digit {
            if digits_count == 0 {
                return Err(ParseError::Empty);
            }
            value = UInt::from_digit(n);
            is_first_digit = false;
        } else if digits_count > 0 {
            (value, ovf) = overflowing_mul10(value, digits_count);

            if ovf {
                return Err(overflow(cb.sign()));
            }

            let next = UInt::from_digit(n);
            (value, ovf) = value.overflowing_add(next);

            if ovf {
                return Err(overflow(cb.sign()));
            }
        }

        if let Some(current) = decimal_offset {
            let Some(dig) = checked_u32_i32(digits_count) else {
                return Err(ParseError::ExponentOverflow);
            };
            let Some(current) = current.checked_add(dig) else {
                return Err(ParseError::ExponentOverflow);
            };
            decimal_offset = Some(current);
        } else if let Some(dot_pos) = dot {
            let Some(dig) = checked_u32_i32(digits_count) else {
                return Err(ParseError::ExponentOverflow);
            };
            let Some(pos) = checked_u32_i32(dot_pos) else {
                return Err(ParseError::ExponentOverflow);
            };
            let Some(current) = dig.checked_sub(pos) else {
                return Err(ParseError::ExponentOverflow);
            };
            decimal_offset = Some(current);
        }
    }

    let exp = match make_exp(decimal_offset, exponent_value) {
        Ok(exp) => exp,
        Err(e) => {
            return Err(e);
        }
    };

    let dec = construct_with_clength(value, exp, cb, ExtraPrecision::new(), clength);

    if dec.is_nan() {
        return Err(ParseError::Unknown);
    } else if dec.is_infinite() {
        return Err(overflow(dec.sign()));
    }

    match dec.ok_or_err() {
        Ok(dec) => Ok(dec),
        Err(e) => Err(match e {
            DecimalError::Overflow => overflow(dec.sign()),
            DecimalError::Underflow => overflow(dec.sign()),
            _ => ParseError::Unknown,
        }),
    }
}

#[inline]
const fn checked_u32_i32(u: u32) -> Option<i32> {
    let max = i32::MAX as u32;
    if u > max {
        None
    } else {
        Some(u as i32)
    }
}

#[inline]
const fn make_exp(
    decimal_offset: Option<i32>,
    exponent_value: Option<i32>,
) -> Result<i32, ParseError> {
    let decimal_offset = if let Some(decimal_offset) = decimal_offset {
        decimal_offset
    } else {
        0
    };

    let exponent_value = if let Some(exponent_value) = exponent_value {
        exponent_value
    } else {
        0
    };

    let Some(exp) = exponent_value.checked_sub(decimal_offset) else {
        return Err(ParseError::ExponentOverflow);
    };

    Ok(exp)
}

#[inline]
const fn parse_exp(buf: &[u8], pos: usize) -> Result<i32, ParseError> {
    if pos >= buf.len() {
        return Err(ParseError::Empty);
    }

    #[allow(unsafe_code)]
    let src = unsafe { from_utf8_unchecked(buf.split_at(pos).1) };

    match i32::from_str_radix(src, 10) {
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
const fn bytes_equal_ci(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut i = 0;
    while i < lhs.len() {
        if !lhs[i].eq_ignore_ascii_case(&rhs[i]) {
            return false;
        }
        i += 1;
    }
    true
}

#[inline(always)]
const fn overflow(sign: Sign) -> ParseError {
    if matches!(sign, Sign::Minus) {
        ParseError::NegOverflow
    } else {
        ParseError::PosOverflow
    }
}
