use crate::decimal::{
    dec::{
        convert::to_f64,
        intrinsics::Intrinsics,
        math::{add::add, div::div, mul::mul},
        parse::from_f64,
    },
    utils::types,
    Decimal, Signal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn sqrt<const N: usize>(d: D<N>) -> D<N> {
    if d.is_nan() {
        return d.raise_signal(Signal::OP_INVALID);
    }

    if d.is_zero() || d.is_one() {
        return d;
    }

    if d.is_negative() {
        return d.signaling_nan();
    }

    if d.is_infinite() {
        return d;
    }

    sqrt_heron(d)
}

#[inline]
const fn sqrt_heron<const N: usize>(d: D<N>) -> D<N> {
    let cb = d.cb;

    let approx_f64 = to_f64(d);
    let guess = types::f64::sqrt(approx_f64);

    let mut result = from_f64(guess).with_cb(cb);

    let mut result_next;
    let mut i = 0;

    while result.is_ok() && i < Intrinsics::<N>::SERIES_MAX_ITERATIONS {
        result_next = mul(D::HALF, add(result, div(d, result)));

        if result.eq_with_extra_precision(&result_next) {
            break;
        }

        result = result_next;
        i += 1;
    }

    result
}
