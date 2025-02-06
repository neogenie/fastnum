use crate::{
    decimal::{
        dec::{
            construct::construct,
            math::{add::add, utils::correct},
            scale::extend_scale_to,
            ExtraPrecision,
        },
        Decimal, Signal,
    },
    int::{math::div_rem_wide, UInt},
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn mul<const N: usize>(lhs: D<N>, rhs: D<N>) -> D<N> {
    if lhs.is_nan() {
        return lhs.compound_and_raise(&rhs, Signal::OP_INVALID);
    }

    if rhs.is_nan() {
        return rhs.compound_and_raise(&lhs, Signal::OP_INVALID);
    }

    let mut cb = lhs.cb.combine_mul(rhs.cb);

    if lhs.is_infinite() || rhs.is_infinite() {
        return if lhs.is_zero() || rhs.is_zero() {
            lhs.with_cb(cb).signaling_nan()
        } else {
            D::INFINITY.with_cb(cb)
        };
    }

    if lhs.is_zero() {
        return extend_scale_to(lhs.with_cb(cb), rhs.scale.saturating_add(lhs.scale));
    }

    if rhs.is_zero() {
        return extend_scale_to(rhs.with_cb(cb), lhs.scale.saturating_add(rhs.scale));
    }

    let mut exp = lhs.exponent() + rhs.exponent();

    let correction = mul_correction(&lhs, &rhs);

    let (mut low, mut high) = lhs.digits.widening_mul(rhs.digits);

    let mut extra_precision = ExtraPrecision::new();

    if high.is_zero() {
        return correct(construct(low, exp, cb, extra_precision), correction);
    }

    cb = cb.raise_signal(Signal::OP_ROUNDED);

    let mut out;
    let mut rem;

    while !high.is_zero() {
        exp += 1;

        out = [0; N];
        rem = 0;

        let mut i = N;
        while i > 0 {
            i -= 1;
            let (q, r) = div_rem_wide(high.digits()[i], rem, 10);
            rem = r;
            out[i] = q;
        }

        high = UInt::from_digits(out);

        i = N;
        out = [0; N];

        while i > 0 {
            i -= 1;
            let (q, r) = div_rem_wide(low.digits()[i], rem, 10);
            rem = r;
            out[i] = q;
        }

        low = UInt::from_digits(out);

        if rem != 0 {
            cb = cb.raise_signal(Signal::OP_INEXACT);
        }

        extra_precision = extra_precision.push(rem);
    }

    let result = construct(low, exp, cb, extra_precision);
    correct(result, correction)
}

#[inline]
const fn mul_correction<const N: usize>(lhs: &D<N>, rhs: &D<N>) -> D<N> {
    let xi_lhs = lhs.extra_digits();
    let xi_rhs = rhs.extra_digits();

    if xi_lhs.is_zero() && xi_rhs.is_zero() {
        D::ZERO
    } else if xi_lhs.is_zero() {
        mul(lhs.without_extra_digits(), xi_rhs)
    } else if xi_rhs.is_zero() {
        mul(rhs.without_extra_digits(), xi_lhs)
    } else {
        add(
            mul(lhs.without_extra_digits(), xi_rhs),
            add(mul(rhs.without_extra_digits(), xi_lhs), mul(xi_rhs, xi_lhs)),
        )
    }
}
