use crate::{
    decimal::{
        dec::{
            construct::construct,
            intrinsics::Intrinsics,
            math::{
                mul::mul,
                sub::sub,
                utils::{correct, overflow_exp},
            },
            ExtraPrecision,
        },
        Decimal, Signal,
    },
    int::{
        math::{div_rem, div_rem_digit},
        UInt,
    },
};

type D<const N: usize> = Decimal<N>;
type U<const N: usize> = UInt<N>;

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

        let correction = div_correction(&dividend, &divisor);

        let (mut digits, mut remainder) = div_rem(dividend.digits, divisor.digits);

        if !remainder.is_zero() {
            let mut quotient;
            let mut digits_prev;

            while !remainder.is_zero() {
                (quotient, remainder) = div_rem_next(remainder, divisor.digits);

                if digits.gt(&Intrinsics::<N>::COEFF_MEDIUM) {
                    let result = construct(
                        digits,
                        exp,
                        cb.raise_signal(Signal::OP_INEXACT)
                            .raise_signal(Signal::OP_ROUNDED),
                        extra_precision(quotient, remainder, divisor.digits),
                    );

                    return correct(result, correction);
                }

                digits_prev = digits.strict_mul(UInt::TEN);

                (exp, overflow) = exp.overflowing_sub(1);
                if overflow {
                    return overflow_exp(exp, cb);
                }

                if digits_prev.gt(&UInt::MAX.strict_sub(quotient)) {
                    let result = construct(
                        UInt::MAX,
                        exp,
                        cb.raise_signal(Signal::OP_INEXACT)
                            .raise_signal(Signal::OP_ROUNDED),
                        extra_precision(quotient, remainder, divisor.digits),
                    );

                    return correct(result, correction);
                }

                digits = digits_prev.strict_add(quotient);
            }
        }

        let result = construct(digits, exp, cb, ExtraPrecision::new());
        correct(result, correction)
    }
}

#[inline]
const fn extra_precision<const N: usize>(
    mut quotient: U<N>,
    mut remainder: U<N>,
    divisor: U<N>,
) -> ExtraPrecision {
    let (mut digits, mut count) = extra_digits(quotient);

    while !remainder.is_zero() && count < 4 {
        (quotient, remainder) = div_rem_next(remainder, divisor);
        let (d, c) = extra_digits(quotient);

        if count == 3 {
            digits += d / 1000;
        } else if count == 2 {
            digits += d / 100;
        } else if count == 1 {
            digits += d / 10;
        } else {
            debug_assert!(count == 0);
            digits = d;
        }

        count += c;
    }

    ExtraPrecision::from_digits(digits)
}

#[inline]
const fn extra_digits<const N: usize>(mut quotient: U<N>) -> (u16, u16) {
    let mut ep = 0;
    let mut count = 0;
    let mut digit;

    (quotient, digit) = div_rem_digit(quotient, 10);
    ep = digit as u16 * 1000 + ep / 10;
    count += 1;

    while !quotient.is_zero() {
        (quotient, digit) = div_rem_digit(quotient, 10);
        ep = digit as u16 * 1000 + ep / 10;
        count += 1;
    }

    (ep, count)
}

#[inline]
const fn div_rem_next<const N: usize>(mut remainder: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    if remainder.gt(&Intrinsics::<N>::COEFF_MEDIUM) {
        mul_div_rem_wide(remainder, UInt::TEN, divisor)
    } else {
        remainder = remainder.strict_mul(UInt::TEN);
        div_rem(remainder, divisor)
    }
}

#[inline]
const fn mul_div_rem_wide<const N: usize>(lhs: U<N>, rhs: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    let (low, high) = lhs.widening_mul(rhs);

    if high.is_zero() {
        div_rem(low, divisor)
    } else {
        div_rem_wide(low, high, divisor)
    }
}

#[inline]
const fn overflow_remainder<const N: usize>(
    mut quotient: U<N>,
    mut remainder: U<N>,
    add: U<N>,
    divisor: U<N>,
) -> (U<N>, U<N>) {
    let overflow;

    (remainder, overflow) = remainder.overflowing_add(add);

    if overflow {
        quotient = quotient.strict_add(UInt::ONE);
        (quotient, remainder) =
            overflow_remainder(quotient, remainder, UInt::MAX.strict_sub(divisor), divisor);
    }

    let q;
    (q, remainder) = div_rem(remainder, divisor);
    quotient = quotient.strict_add(q);

    (quotient, remainder)
}

#[inline]
const fn div_rem_wide<const N: usize>(low: U<N>, high: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    let mut quotient;
    let mut remainder;

    (quotient, remainder) = div_rem(low, divisor);

    let (high_quotient, high_remainder) =
        mul_div_rem_wide(UInt::MAX.shr(1).add(UInt::ONE), high, divisor);

    (quotient, remainder) = overflow_remainder(quotient, remainder, high_remainder, divisor);
    (quotient, remainder) = overflow_remainder(quotient, remainder, high_remainder, divisor);

    quotient = quotient.strict_add(high_quotient).strict_add(high_quotient);

    (quotient, remainder)
}

#[inline]
const fn div_correction<const N: usize>(dividend: &D<N>, divisor: &D<N>) -> D<N> {
    let xi_dividend = dividend.extra_digits();
    let xi_divisor = divisor.extra_digits();

    if xi_dividend.is_zero() && xi_divisor.is_zero() {
        D::ZERO
    } else if xi_divisor.is_zero() {
        let x = div(xi_dividend, divisor.without_extra_digits());

        if x.is_op_underflow() {
            D::ZERO
        } else {
            x
        }
    } else {
        let x = if xi_dividend.is_zero() {
            mul(dividend.without_extra_digits(), xi_divisor)
                .without_extra_digits()
                .neg()
        } else {
            sub(
                mul(divisor.without_extra_digits(), xi_dividend).without_extra_digits(),
                mul(dividend.without_extra_digits(), xi_divisor).without_extra_digits(),
            )
        };

        if x.is_zero() || x.is_op_underflow() {
            D::ZERO
        } else {
            let z = mul(divisor.without_extra_digits(), *divisor).without_extra_digits();
            let y = div(x, z);

            if y.is_op_underflow() {
                D::ZERO
            } else {
                y
            }
        }
    }
}
