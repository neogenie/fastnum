use core::cmp::Ordering;

use crate::decimal::{
    dec::math::{add::add, consts::Consts, div::div, ln::ln},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn log2<const N: usize>(x: D<N>) -> D<N> {
    if x.is_nan() {
        return x.op_invalid();
    }

    if x.is_zero() {
        return D::NEG_INFINITY.op_invalid().with_ctx(x.context());
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

    // TODO: Integral special cases
    log2_reduce(x)
}

#[inline]
const fn log2_reduce<const N: usize>(x: D<N>) -> D<N> {
    // TODO: remove iteration by direct 2^N
    match x.cmp(&D::TWO) {
        Ordering::Less => log2_impl(x),
        Ordering::Equal => D::ONE,
        Ordering::Greater => add(log2_reduce(div(x, D::TWO)), D::ONE),
    }
}

#[inline]
const fn log2_impl<const N: usize>(d: D<N>) -> D<N> {
    div(ln(d), Consts::LN_2)
}
