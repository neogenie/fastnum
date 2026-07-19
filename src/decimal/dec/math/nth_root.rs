use crate::decimal::{
    dec::{
        convert::to_f64,
        intrinsics::Intrinsics,
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
        // Odd roots of a negative are real: ⁿ√(−d) = −ⁿ√d. Even roots have no real value.
        if n % 2 == 1 {
            return nth_root(d.abs(), n).neg();
        }
        return d.signaling_nan();
    }

    if d.is_infinite() {
        return d;
    }

    nth_root_newton(d, n).set_ctx(d.context())
}

#[inline]
const fn nth_root_newton<const N: usize>(d: D<N>, n: u32) -> D<N> {
    let approx_f64 = to_f64(d);
    let guess = types::f64::sqrt(approx_f64);

    let mut result = from_f64(guess).compound(&d);

    let mut result_next;

    let n_minus_one = from_u32(n - 1);
    let one_div_n = div(D::ONE, from_u32(n));
    let mut x_n;
    let mut j;
    let mut i = 1;

    while result.is_ok() && i < Intrinsics::<N>::SERIES_MAX_ITERATIONS {
        x_n = result;
        j = n - 2;

        while j > 0 {
            x_n = mul(x_n, result);
            j -= 1;
        }

        result_next = mul(one_div_n, add(mul(n_minus_one, result), div(d, x_n)));

        if result.eq(&result_next) {
            break;
        }

        result = result_next;
        i += 1;
    }

    result
}
