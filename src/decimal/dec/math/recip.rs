use crate::decimal::{
    dec::{
        convert::to_f64,
        math::{add::add, mul::mul, sub::sub},
        parse::from_f64,
        scale,
        scale::extend_scale_to,
    },
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn recip<const N: usize>(mut d: D<N>) -> D<N> {
    if d.is_nan() {
        return d.raise_op_invalid();
    }

    if d.is_zero() {
        return D::INFINITY.compound(&d).raise_op_div_by_zero();
    }

    if d.is_infinite() {
        return D::ZERO.compound(&d);
    }

    let scale = d.cb.get_scale();

    let approx_f64 = to_f64(d);
    let approx_result = 1.0 / approx_f64;

    let mut result = from_f64(approx_result).compound(&d);

    let mut result_next;

    while result.is_ok() {
        result_next = add(result, mul(result, sub(D::ONE, mul(result, d))));

        if result.eq_with_extra_precision(&result_next) {
            break;
        }

        result = result_next;
    }
    
    extend_scale_to(scale::reduce(result), scale)
        .raise_op_inexact()
        .raise_op_clamped()
        .raise_op_rounded()
}
