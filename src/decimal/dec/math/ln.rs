use core::cmp::Ordering;

use crate::decimal::{
    dec::{
        intrinsics::Intrinsics,
        math::{add::add, div::div, mul::mul, sqrt::sqrt, sub::sub},
        parse::from_u32,
    },
    Decimal, Signal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn ln<const N: usize>(x: D<N>) -> D<N> {
    if x.is_nan() {
        return x.raise_signal(Signal::OP_INVALID);
    }

    if x.is_zero() {
        return D::NEG_INFINITY
            .raise_signal(Signal::OP_INVALID)
            .with_ctx(x.context());
    }

    if x.is_negative() {
        return x.signaling_nan();
    }

    if x.is_infinite() {
        return x;
    }

    if x.is_one() {
        return D::ZERO.with_ctx(x.context());
    }

    argument_reduction(x)
}

#[inline]
pub(crate) const fn ln_1p<const N: usize>(x: D<N>) -> D<N> {
    ln(add(D::ONE, x))
}

#[inline]
const fn argument_reduction<const N: usize>(x: D<N>) -> D<N> {
    match x.cmp(&D::TWO) {
        Ordering::Less => taylor_series(x),
        Ordering::Equal => D::LN_2,
        Ordering::Greater => mul(D::TWO, argument_reduction(sqrt(x))),
    }
}

#[inline]
const fn taylor_series<const N: usize>(x: D<N>) -> D<N> {
    let mut result = D::ZERO;
    let mut result_next;

    let mut base = div(sub(x, D::ONE), add(x, D::ONE));
    let mut item = base;

    base = mul(base, base);

    let mut i = 1;

    while i < Intrinsics::<N>::SERIES_MAX_ITERATIONS * 2 {
        result_next = add(result, div(item, from_u32(i)));

        if result.eq_with_extra_precision(&result_next) {
            break;
        }

        item = mul(item, base);

        result = result_next;
        i += 2;
    }

    mul(result, D::TWO)
}
