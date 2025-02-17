use crate::decimal::{
    dec::{
        convert::to_f64,
        math::{add::add, div::div, mul::mul},
        parse::{from_f64, from_u32},
    },
    utils::types,
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn nth_root<const N: usize>(d: D<N>, n: u32) -> D<N> {
    if d.is_nan() {
        return d.op_invalid();
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

    nth_root_newton(d, n)
}

#[inline]
const fn nth_root_newton<const N: usize>(d: D<N>, n: u32) -> D<N> {
    let approx_f64 = to_f64(d);
    let guess = types::f64::sqrt(approx_f64);

    let mut result = from_f64(guess).compound(&d);

    let mut result_next;

    let n_minus_one = from_u32(n - 1);
    let one_div_n = D::ONE.div(from_u32(n));
    let mut x_n;
    let mut i;

    while result.is_ok() {
        x_n = result;
        i = n - 2;

        while i > 0 {
            x_n = mul(x_n, result);
            i -= 1;
        }

        result_next = mul(one_div_n, add(mul(n_minus_one, result), div(d, x_n)));

        if result.eq(&result_next) {
            break;
        }

        result = result_next;
    }

    result
}
