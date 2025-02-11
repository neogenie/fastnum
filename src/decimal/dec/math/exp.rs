use crate::decimal::{
    dec::{
        intrinsics::Intrinsics,
        math::{add::add, div::div, mul::mul, sub::sub},
        parse::from_u32,
    },
    Decimal,
};
use crate::decimal::dec::math::consts::Consts;

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn exp<const N: usize>(x: D<N>) -> D<N> {
    if x.is_nan() {
        return x.raise_op_invalid();
    }

    if x.is_zero() {
        return D::ONE.with_ctx(x.context());
    }

    if x.is_negative() {
        return div(D::ONE, exp_abs(x.abs()));
    }

    exp_abs(x)
}

#[inline]
pub(crate) const fn exp_m1<const N: usize>(x: D<N>) -> D<N> {
    sub(exp(x), D::ONE)
}

#[inline]
const fn exp_abs<const N: usize>(x: D<N>) -> D<N> {
    debug_assert!(!x.is_negative());

    if x.is_infinite() {
        return D::INFINITY.with_ctx(x.context());
    }

    if x.is_one() {
        return Consts::E.with_ctx(x.context());
    }

    argument_reduction(x)
}

#[inline]
const fn argument_reduction<const N: usize>(x: D<N>) -> D<N> {
    if x.ge(&D::ONE) {
        let y = argument_reduction(mul(x, D::HALF));
        mul(y, y)
    } else {
        taylor_series(x)
    }
}

#[inline]
const fn taylor_series<const N: usize>(x: D<N>) -> D<N> {
    let mut result = D::ONE;
    let mut result_next;
    let mut item = x;
    let mut i = 2;

    while i < Intrinsics::<N>::SERIES_MAX_ITERATIONS + 2 {
        result_next = add(result, item);

        if result.eq_with_extra_precision(&result_next) {
            break;
        }

        item = div(mul(item, x), from_u32(i));

        result = result_next;
        i += 1;
    }

    result.with_ctx(x.context())
}
