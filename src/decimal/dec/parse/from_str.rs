use core::{num::IntErrorKind, str::from_utf8_unchecked};

use crate::{
    bint::{
        intrinsics::{Digit, DIGIT_POWER_10},
        UInt,
    },
    decimal::{
        dec::{construct::construct, ExtraPrecision},
        signals::Signals,
        Context, Decimal, DecimalError, ParseError, Sign,
    },
};

const MAX_DIGITS_COUNT: i32 = DIGIT_POWER_10 as i32;

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
    let mut sign = Sign::Plus;

    let mut i = 0;

    // Parse sign or `NAN` special case
    match buf[0] {
        b'+' => {
            i = 1;
        }
        b'-' => {
            sign = Sign::Minus;
            i = 1;
        }
        b'n' | b'N' if bytes_equal_ci(buf.split_at(1).1, b"an") => {
            return Ok(Decimal::NAN);
        }
        _ => {}
    }

    if i == len {
        return Err(ParseError::Empty);
    }

    // Parse special cases Inf/Infinity
    if bytes_equal_ci(buf.split_at(i).1, b"inf") || bytes_equal_ci(buf.split_at(i).1, b"infinity") {
        return Ok(Decimal::INFINITY.set_ctx(ctx).set_sign(sign));
    }

    let mut clength: u32 = 0;
    let mut exp = 0;
    let mut decimal_offset: i32 = 1;

    let mut is_first_digit = true;

    // Digits are stored as little endian (the least significant digit is first).
    let mut value = UInt::ZERO;

    while i < len {
        let mut digits_count = 0;
        let mut n = 0;

        while i < len && (digits_count < MAX_DIGITS_COUNT) {
            let b = buf[i];
            match b {
                b'0'..=b'9' => {
                    let d = b - b'0';
                    n = ((n + (n << 2)) << 1) + d as Digit;
                    digits_count += 1;
                }
                b'.' => {
                    if decimal_offset <= 0 {
                        return Err(ParseError::InvalidLiteral);
                    } else {
                        decimal_offset = -(clength as i32 + digits_count);
                    }
                }
                b'E' | b'e' => {
                    i += 1;
                    if i >= len {
                        return Err(ParseError::Empty);
                    }

                    #[allow(unsafe_code)]
                    let src = unsafe { from_utf8_unchecked(buf.split_at(i).1) };

                    match i32::from_str_radix(src, 10) {
                        Ok(e) => {
                            exp = e;
                        }
                        Err(e) => {
                            return Err(match e.kind() {
                                IntErrorKind::Empty => ParseError::Empty,
                                IntErrorKind::InvalidDigit => ParseError::InvalidLiteral,
                                IntErrorKind::PosOverflow => ParseError::ExponentOverflow,
                                IntErrorKind::NegOverflow => ParseError::ExponentOverflow,
                                _ => ParseError::Unknown,
                            });
                        }
                    }

                    i = len;
                    break;
                }
                b'_' => {}
                _ => {
                    return Err(ParseError::InvalidLiteral);
                }
            };

            i += 1;
        }

        clength += digits_count as u32;

        if is_first_digit {
            if digits_count == 0 {
                return Err(ParseError::Empty);
            }
            value = UInt::from_digit(n);
            is_first_digit = false;
        } else if digits_count > 0 {
            match value.overflowing_mul(UInt::power_of_ten(digits_count as u32)) {
                (v, false) => {
                    value = v;
                }
                (_, true) => {
                    return Err(overflow(sign));
                }
            }

            let next = UInt::from_digit(n);
            match value.overflowing_add(next) {
                (v, false) => {
                    value = v;
                }
                (_, true) => {
                    return Err(overflow(sign));
                }
            }
        }
    }

    if decimal_offset <= 0 {
        exp += -(clength as i32 + decimal_offset);
    }

    let dec = construct(
        value,
        exp,
        sign,
        Signals::empty(),
        ctx,
        ExtraPrecision::new(),
    );

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

#[inline(always)]
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
