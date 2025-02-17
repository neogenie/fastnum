use crate::{
    decimal::{
        dec::{
            construct::construct,
            math::{div::div, utils::overflow_exp},
            ExtraPrecision,
        },
        signals::Signals,
        Context, Decimal, Sign,
    },
    int::{math::div_rem_digit, UInt},
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn powi<const N: usize>(d: D<N>, n: i32) -> D<N> {
    if d.is_nan() {
        return d.op_invalid();
    }

    let sign = if d.is_negative() && (n % 2 != 0) {
        Sign::Minus
    } else {
        Sign::Plus
    };

    if d.is_infinite() {
        return if n > 0 {
            if sign.is_negative() ^ d.is_negative() {
                d.neg()
            } else {
                d
            }
        } else if n == 0 {
            D::ONE
        } else {
            D::ZERO.set_sign(sign)
        };
    }

    if n == 0 {
        return if d.is_zero() {
            d.signaling_nan()
        } else {
            D::ONE
        };
    }

    if d.is_zero() {
        return if n < 0 {
            D::INFINITY.set_ctx(d.context()).set_sign(sign)
        } else {
            D::ZERO.set_ctx(d.context()).set_sign(sign)
        };
    }

    if n < 0 {
        div(
            D::ONE,
            powi_integral(
                d.digits,
                d.cb.get_exponent(),
                sign,
                d.cb.get_signals(),
                d.cb.get_context(),
                n.overflowing_neg().0 as u32,
            ),
        )
    } else {
        powi_integral(
            d.digits,
            d.cb.get_exponent(),
            sign,
            d.cb.get_signals(),
            d.cb.get_context(),
            n as u32,
        )
    }
}

#[inline]
const fn powi_integral<const N: usize>(
    mut digits: UInt<N>,
    mut exp: i32,
    sign: Sign,
    mut signals: Signals,
    ctx: Context,
    n: u32,
) -> D<N> {
    // TODO: special case 2^n

    if n > i32::MAX as u32 {
        return overflow_exp(-1, sign, signals, ctx);
    }

    let (mut out, mut overflow) = digits.overflowing_pow(n);
    let mut extra_precision = ExtraPrecision::new();

    if overflow {
        let mut extra_digit;
        signals.raise(Signals::OP_ROUNDED);

        while overflow {
            (digits, extra_digit) = div_rem_digit(digits, 10);

            if extra_digit != 0 {
                signals.raise(Signals::OP_INEXACT);
            }

            extra_precision.push_digit(extra_digit);

            (exp, overflow) = exp.overflowing_add(1);

            if overflow {
                return overflow_exp(exp, sign, signals, ctx);
            }

            (out, overflow) = digits.overflowing_pow(n);
        }
    }

    (exp, overflow) = exp.overflowing_mul(n as i32);

    if overflow {
        return overflow_exp(exp, sign, signals, ctx);
    }

    construct(out, exp, sign, signals, ctx, extra_precision)
}
