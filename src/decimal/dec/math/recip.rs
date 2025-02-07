use crate::decimal::{
    dec::{
        convert::to_f64,
        math::{add::add, mul::mul, sub::sub},
        parse::from_f64,
        scale,
        scale::extend_scale_to,
    },
    Decimal, Signal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn recip<const N: usize>(d: D<N>) -> D<N> {
    if d.is_nan() {
        return d.raise_signal(Signal::OP_INVALID);
    }

    let cb = d.cb;

    if d.is_zero() {
        return D::INFINITY.with_cb(cb.raise_signal(Signal::div_by_zero()));
    }

    if d.is_infinite() {
        return D::ZERO.with_cb(cb);
    }

    let scale = d.scale;

    let approx_f64 = to_f64(d);
    let approx_result = 1.0 / approx_f64;

    let mut result = from_f64(approx_result).with_cb(cb);

    let mut result_next;

    while result.is_ok() {
        result_next = add(result, mul(result, sub(D::ONE, mul(result, d))));

        if result.eq_with_extra_precision(&result_next) {
            break;
        }

        result = result_next;
    }

    extend_scale_to(scale::reduce(result), scale).raise_signal(
        Signal::OP_INEXACT
            .combine(Signal::OP_ROUNDED)
            .combine(Signal::OP_CLAMPED),
    )
}
