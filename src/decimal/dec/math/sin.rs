use core::cmp::Ordering;

use crate::decimal::{
    dec::{
        intrinsics::Intrinsics,
        math::{add::add, consts::Consts, div::div, mul::mul, rem::rem, sub::sub},
        parse::from_u32,
    },
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn sin<const N: usize>(d: D<N>) -> D<N> {
    if d.is_nan() {
        return d.op_invalid();
    }

    if d.is_zero() {
        return D::ZERO.with_ctx(d.context());
    }

    if d.is_infinite() {
        return d.signaling_nan();
    }

    if d.is_negative() {
        sin_abs(d.neg()).neg()
    } else {
        sin_abs(d)
    }
}

#[inline]
const fn sin_abs<const N: usize>(d: D<N>) -> D<N> {
    debug_assert!(!d.is_negative());

    // We notice that sin(x) is cyclic with a period of 2π so we can quickly
    // reduce any argument > 2π so it falls between zero and 2π by simply taking
    // x modulo 2π.
    let x = rem(d, Consts::TAU);
    debug_assert!(x.lt(&Consts::TAU));

    match x.cmp(&Consts::PI) {
        Ordering::Less => sin_less_pi(x),
        Ordering::Equal => D::ZERO.with_ctx(d.context()),
        Ordering::Greater => {
            // We can further reduce x, so it is between 0..π using the identity:
            // sin(x)=-sin(x-π) for x≥π.
            sin_less_pi(sub(x, Consts::PI)).neg()
        }
    }
}

#[inline]
const fn sin_less_pi<const N: usize>(x: D<N>) -> D<N> {
    debug_assert!(!x.is_negative());
    debug_assert!(x.lt(&Consts::PI));

    if x.eq(&Consts::FRAC_PI_2) {
        D::ONE.with_ctx(x.context())
    } else {
        taylor_series(x)
    }
}

#[inline]
const fn taylor_series<const N: usize>(x: D<N>) -> D<N> {
    debug_assert!(!x.is_negative());
    debug_assert!(x.lt(&Consts::PI));

    let mut result = D::ZERO;
    let mut result_next;
    let mut item = x;

    let x2 = mul(x, x);

    let mut i = 1;

    while i < Intrinsics::<N>::SERIES_MAX_ITERATIONS * 2 {
        result_next = add(result, item);

        if result.eq(&result_next) {
            break;
        }

        item = div(mul(item, x2), from_u32((i + 1) * (i + 2))).neg();

        result = result_next;
        i += 2;
    }

    if result.gt(&D::ONE) {
        result = D::ONE.compound(&result);
    }

    result.with_ctx(x.context())
}
