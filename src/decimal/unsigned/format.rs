use crate::decimal::signed::Sign;
use crate::decimal::RoundingMode;
use num_traits::ToPrimitive;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Write;
use std::ops::Neg;

include!(concat!(env!("OUT_DIR"), "/exponential_format_threshold.rs"));

pub(crate) fn write_scientific_notation<W: Write>(
    digits: String,
    scale: i64,
    w: &mut W,
) -> fmt::Result {
    let (first_digit, remaining_digits) = digits.as_str().split_at(1);
    w.write_str(first_digit)?;
    if !remaining_digits.is_empty() {
        w.write_str(".")?;
        w.write_str(remaining_digits)?;
    }
    write!(w, "e{}", remaining_digits.len() as i128 - scale as i128)
}

pub(crate) fn write_engineering_notation<W: Write>(
    digits: String,
    scale: i64,
    out: &mut W,
) -> fmt::Result {
    let digit_count = digits.len();

    let top_digit_exponent = digit_count as i128 - scale as i128;

    let shift_amount = match top_digit_exponent.rem_euclid(3) {
        0 => 3,
        i => i as usize,
    };

    let exp = top_digit_exponent - shift_amount as i128;

    // handle adding zero padding
    if let Some(padding_zero_count) = shift_amount.checked_sub(digits.len()) {
        let zeros = &"000"[..padding_zero_count];
        out.write_str(&digits)?;
        out.write_str(zeros)?;
        return write!(out, "e{}", exp);
    }

    let (head, rest) = digits.split_at(shift_amount);
    debug_assert_eq!(exp % 3, 0);

    out.write_str(head)?;

    if !rest.is_empty() {
        out.write_char('.')?;
        out.write_str(rest)?;
    }

    write!(out, "e{}", exp)
}

pub(crate) fn format(digits: String, scale: i64, f: &mut fmt::Formatter) -> fmt::Result {
    // number of zeros between most significant digit and decimal point
    let leading_zero_count = scale
        .to_u64()
        .and_then(|scale| scale.checked_sub(digits.len() as u64))
        .unwrap_or(0);

    // number of zeros between last significant digit and decimal point
    let trailing_zero_count = scale.checked_neg().and_then(|d| d.to_u64());

    // this ignores scientific-formatting if precision is requested
    let trailing_zeros = f
        .precision()
        .map(|_| 0)
        .or(trailing_zero_count)
        .unwrap_or(0);

    let leading_zero_threshold = EXPONENTIAL_FORMAT_LEADING_ZERO_THRESHOLD as u64;
    let trailing_zero_threshold = EXPONENTIAL_FORMAT_TRAILING_ZERO_THRESHOLD as u64;

    // use exponential form if decimal point is outside
    // the upper and lower thresholds of the decimal
    if leading_zero_threshold < leading_zero_count {
        format_exponential(digits, scale, f, "E")
    } else if trailing_zero_threshold < trailing_zeros {
        // non-scientific notation
        format_dotless_exponential(digits, scale, f, "e")
    } else {
        format_full_scale(digits, scale, f)
    }
}

pub(crate) fn format_exponential(
    digits: String,
    scale: i64,
    f: &mut fmt::Formatter,
    e_symbol: &str,
) -> fmt::Result {
    let exp = (scale as i128).neg();
    format_exponential_be_ascii_digits(digits, exp, f, e_symbol)
}

fn format_dotless_exponential(
    mut digits: String,
    scale: i64,
    f: &mut fmt::Formatter,
    e_symbol: &str,
) -> fmt::Result {
    debug_assert!(scale <= 0);
    write!(digits, "{}{:+}", e_symbol, -scale)?;
    f.pad_integral(true, "", &digits)
}

fn format_full_scale(digits: String, scale: i64, f: &mut fmt::Formatter) -> fmt::Result {
    let mut digits = digits.into_bytes();
    let mut exp = (scale as i128).neg();

    if scale <= 0 {
        // formatting an integer value (add trailing zeros to the right)
        zero_right_pad_integer_ascii_digits(&mut digits, &mut exp, f.precision());
    } else {
        let scale = scale as u64;
        // no-precision behaves the same as precision matching scale (i.e. no padding or rounding)
        let prec = f
            .precision()
            .and_then(|prec| prec.to_u64())
            .unwrap_or(scale);

        if scale < digits.len() as u64 {
            // format both integer and fractional digits (always 'trim' to precision)
            trim_ascii_digits(&mut digits, scale, prec, &mut exp);
        } else {
            // format only fractional digits
            shift_or_trim_fractional_digits(&mut digits, scale, prec, &mut exp);
        }
        // never print exp when in this branch
        exp = 0;
    }

    // move digits back into String form
    let mut buf = String::from_utf8(digits).unwrap();

    // add exp part to buffer (if not zero)
    if exp != 0 {
        write!(buf, "e{:+}", exp)?;
    }

    // write buffer to formatter
    f.pad_integral(true, "", &buf)
}

/// Fill appropriate number of zeros and decimal point into Vec of (ascii/utf-8) digits
///
/// Exponent is set to zero if zeros were added
///
fn zero_right_pad_integer_ascii_digits(
    digits: &mut Vec<u8>,
    exp: &mut i128,
    precision: Option<usize>,
) {
    debug_assert!(*exp >= 0);

    let trailing_zero_count = match exp.to_usize() {
        Some(n) => n,
        None => {
            return;
        }
    };
    let total_additional_zeros = trailing_zero_count.saturating_add(precision.unwrap_or(0));
    if total_additional_zeros > FMT_MAX_INTEGER_PADDING {
        return;
    }

    // requested 'prec' digits of precision after decimal point
    match precision {
        None if trailing_zero_count > 20 => {}
        None | Some(0) => {
            digits.resize(digits.len() + trailing_zero_count, b'0');
            *exp = 0;
        }
        Some(prec) => {
            digits.resize(digits.len() + trailing_zero_count, b'0');
            digits.push(b'.');
            digits.resize(digits.len() + prec, b'0');
            *exp = 0;
        }
    }
}

/// Fill zeros into utf-8 digits
fn trim_ascii_digits(digits: &mut Vec<u8>, scale: u64, prec: u64, exp: &mut i128) {
    debug_assert!(scale < digits.len() as u64);
    // there are both integer and fractional digits
    let integer_digit_count = (digits.len() as u64 - scale)
        .to_usize()
        .expect("Number of digits exceeds maximum usize");

    if prec < scale {
        let prec = prec.to_usize().expect("Precision exceeds maximum usize");
        apply_rounding_to_ascii_digits(digits, exp, integer_digit_count + prec);
    }

    if prec != 0 {
        digits.insert(integer_digit_count, b'.');
    }

    if scale < prec {
        let trailing_zero_count = (prec - scale).to_usize().expect("Too Big");

        // precision required beyond scale
        digits.resize(digits.len() + trailing_zero_count, b'0');
    }
}

fn shift_or_trim_fractional_digits(digits: &mut Vec<u8>, scale: u64, prec: u64, exp: &mut i128) {
    debug_assert!(scale >= digits.len() as u64);
    // there are no integer digits
    let leading_zeros = scale - digits.len() as u64;

    match prec.checked_sub(leading_zeros) {
        None => {
            digits.clear();
            digits.push(b'0');
            if prec > 0 {
                digits.push(b'.');
                digits.resize(2 + prec as usize, b'0');
            }
        }
        Some(0) => {
            // precision is at the decimal digit boundary, round one value
            let insig_digit = digits[0] - b'0';
            let trailing_zeros = digits[1..].iter().all(|&d| d == b'0');

            let rounded_value =
                RoundingMode::default().round_pair(Sign::NoSign, (0, insig_digit), trailing_zeros);

            digits.clear();
            if leading_zeros != 0 {
                digits.push(b'0');
                digits.push(b'.');
                digits.resize(1 + leading_zeros as usize, b'0');
            }
            digits.push(rounded_value + b'0');
        }
        Some(digit_prec) => {
            let digit_prec = digit_prec as usize;
            let leading_zeros = leading_zeros
                .to_usize()
                .expect("Number of leading zeros exceeds max usize");
            let trailing_zeros = digit_prec.saturating_sub(digits.len());
            if digit_prec < digits.len() {
                apply_rounding_to_ascii_digits(digits, exp, digit_prec);
            }
            digits.extend_from_slice(b"0.");
            digits.resize(digits.len() + leading_zeros, b'0');
            digits.rotate_right(leading_zeros + 2);

            // add any extra trailing zeros
            digits.resize(digits.len() + trailing_zeros, b'0');
        }
    }
}

fn format_exponential_be_ascii_digits(
    digits: String,
    mut exp: i128,
    f: &mut fmt::Formatter,
    e_symbol: &str,
) -> fmt::Result {
    let mut digits = digits.into_bytes();

    // how many zeros to pad at the end of the decimal
    let mut extra_trailing_zero_count = 0;

    if let Some(prec) = f.precision() {
        // 'prec' is number of digits after the decimal point
        let total_prec = prec + 1;
        let digit_count = digits.len();

        match total_prec.cmp(&digit_count) {
            Ordering::Equal => {
                // digit count is one more than precision - do nothing
            }
            Ordering::Less => {
                // round to smaller precision
                apply_rounding_to_ascii_digits(&mut digits, &mut exp, total_prec);
            }
            Ordering::Greater => {
                // increase number of zeros to add to end of digits
                extra_trailing_zero_count = total_prec - digit_count;
            }
        }
    }

    let needs_decimal_point = digits.len() > 1 || extra_trailing_zero_count > 0;

    let mut abs_int = String::from_utf8(digits).unwrap();

    // Determine the exponent value based on the scale
    //
    // # First case: the integer representation falls completely behind the
    //   decimal point.
    //
    //   Example of this.scale > abs_int.len():
    //   0.000001234509876
    //   abs_int.len() = 10
    //   scale = 15
    //   target is 1.234509876
    //   exponent = -6
    //
    //   Example of this.scale == abs_int.len():
    //   0.333333333333333314829616256247390992939472198486328125
    //   abs_int.len() = 54
    //   scale = 54
    //   target is 3.33333333333333314829616256247390992939472198486328125
    //   exponent = -1
    //
    // # Second case: the integer representation falls around, or before the
    //   decimal point
    //
    //   ## Case 2.1, entirely before the decimal point.
    //     Example of (abs_int.len() - this.scale) > abs_int.len():
    //     123450987600000
    //     abs_int.len() = 10
    //     scale = -5
    //     location = 15
    //     target is 1.234509876
    //     exponent = 14
    //
    //   ## Case 2.2, somewhere around the decimal point.
    //     Example of (abs_int.len() - this.scale) < abs_int.len():
    //     12.339999999999999857891452847979962825775146484375
    //     abs_int.len() = 50
    //     scale = 48
    //     target is 1.2339999999999999857891452847979962825775146484375
    //     exponent = 1
    //
    //     For the (abs_int.len() - this.scale) == abs_int.len() I couldn't
    //     come up with an example
    let exponent = abs_int.len() as i128 + exp - 1;

    if needs_decimal_point {
        // only add decimal point if there is more than 1 decimal digit
        abs_int.insert(1, '.');
    }

    if extra_trailing_zero_count > 0 {
        abs_int.extend(core::iter::repeat('0').take(extra_trailing_zero_count));
    }

    // always print exponent in exponential mode
    write!(abs_int, "{}{:+}", e_symbol, exponent)?;

    f.pad_integral(true, "", &abs_int)
}

/// Round big-endian digits in ascii
fn apply_rounding_to_ascii_digits(ascii_digits: &mut Vec<u8>, exp: &mut i128, prec: usize) {
    if ascii_digits.len() < prec {
        return;
    }

    // shift exp to align with new length of digits
    *exp += (ascii_digits.len() - prec) as i128;

    // true if all ascii_digits after precision are zeros
    let trailing_zeros = ascii_digits[prec + 1..].iter().all(|&d| d == b'0');

    let sig_digit = ascii_digits[prec - 1] - b'0';
    let insig_digit = ascii_digits[prec] - b'0';

    let rounded_digit =
        RoundingMode::default().round_pair(Sign::NoSign, (sig_digit, insig_digit), trailing_zeros);

    // remove insignificant digits
    ascii_digits.truncate(prec - 1);

    // push rounded value
    if rounded_digit < 10 {
        ascii_digits.push(rounded_digit + b'0');
        return;
    }

    debug_assert_eq!(rounded_digit, 10);

    // push zero and carry-the-one
    ascii_digits.push(b'0');

    // loop through digits in reverse order (skip the 0 we just pushed)
    let digits = ascii_digits.iter_mut().rev().skip(1);
    for digit in digits {
        if *digit < b'9' {
            // we've carried the one as far as it will go
            *digit += 1;
            return;
        }

        debug_assert_eq!(*digit, b'9');

        // digit was a 9, set to zero and carry the one
        // to the next digit
        *digit = b'0';
    }

    // at this point all digits have become zero
    // just set significant digit to 1 and increase exponent
    //
    // eg: 9999e2 ~> 0000e2 ~> 1000e3
    //
    ascii_digits[0] = b'1';
    *exp += 1;
}
