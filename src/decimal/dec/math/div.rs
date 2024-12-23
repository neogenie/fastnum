use crate::{
    decimal::{
        dec::{construct::construct, math::utils::overflow_exp},
        round::scale_round,
        Decimal, Signal,
    },
    int::{math::div_rem, UInt},
};
use crate::decimal::dec::math::utils::overflow_coeff;

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn div<const N: usize>(dividend: D<N>, divisor: D<N>) -> D<N> {
    if dividend.is_nan() {
        return dividend.compound_and_raise(&divisor, Signal::OP_INVALID);
    }

    if divisor.is_nan() {
        return divisor.compound_and_raise(&dividend, Signal::OP_INVALID);
    }

    let cb = dividend.cb.combine_mul(divisor.cb);

    if dividend.is_infinite() && divisor.is_infinite() {
        return D::NAN.with_cb(cb.raise_signal(Signal::OP_INVALID));
    }

    if divisor.is_zero() {
        D::INFINITY.with_cb(cb.raise_signal(Signal::div_by_zero()))
    } else if dividend.is_zero() || divisor.is_one() {
        dividend.compound(&divisor)
    } else if dividend.is_infinite() {
        D::INFINITY.with_cb(cb)
    } else if divisor.is_infinite() {
        D::ZERO.with_cb(cb)
    } else {
        let (mut exp, mut overflow) =
            (dividend.scale as i32 - divisor.scale as i32).overflowing_neg();

        if overflow {
            return overflow_exp(exp, cb);
        }

        let (mut digits, mut remainder) = div_rem(dividend.digits, divisor.digits);

        if !remainder.is_zero() {
            let mut quotient;

            while !remainder.is_zero() {
                (remainder, overflow) = remainder.overflowing_mul(UInt::TEN);

                if overflow {
                    return overflow_coeff(exp, cb);
                }

                (quotient, remainder) = div_rem(remainder, divisor.digits);

                if digits.gt(&D::<N>::COEFF_MEDIUM) {
                    // TODO: performance optimizations
                    let (digit, _) = scale_round(quotient, cb.context());

                    if digit.is_one() {
                        digits = digits.saturating_add(digit);
                    }

                    return construct(
                        digits,
                        exp,
                        cb.raise_signal(Signal::OP_INEXACT)
                            .raise_signal(Signal::OP_ROUNDED),
                    );
                }

                digits = digits.strict_mul(UInt::TEN);

                (exp, overflow) = exp.overflowing_sub(1);
                if overflow {
                    return overflow_exp(exp, cb);
                }

                if digits.gt(&UInt::MAX.strict_sub(quotient)) {
                    return construct(
                        UInt::MAX,
                        exp,
                        cb.raise_signal(Signal::OP_INEXACT)
                            .raise_signal(Signal::OP_ROUNDED),
                    );
                }

                digits = digits.strict_add(quotient);
            }
        }

        construct(digits, exp, cb)
    }
}
