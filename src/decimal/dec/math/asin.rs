use core::cmp::Ordering;

use crate::decimal::{
    dec::{
        intrinsics::Intrinsics,
        math::{add::add, consts::Consts, div::div, mul::mul, sqrt::sqrt, sub::sub},
        parse::from_u32,
    },
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn asin<const N: usize>(x: D<N>) -> D<N> {
    if x.is_nan() {
        return x.op_invalid();
    }

    if x.is_zero() {
        return D::ZERO.with_ctx(x.context());
    }

    if x.is_infinite() {
        return x.signaling_nan();
    }

    match x.cmp(&D::ONE.neg()) {
        Ordering::Less => {
            return x.signaling_nan();
        }
        Ordering::Equal => return Consts::FRAC_PI_2.neg(),
        Ordering::Greater => {}
    }

    match x.cmp(&D::ONE) {
        Ordering::Less => {}
        Ordering::Equal => {
            return Consts::FRAC_PI_2;
        }
        Ordering::Greater => {
            return x.signaling_nan();
        }
    }

    asin_reduction(x)
}

struct Reduction<const N: usize>;

impl<const N: usize> Reduction<N> {
    const K: D<N> = D::HALF; // TODO
}

#[inline]
const fn asin_reduction<const N: usize>(x: D<N>) -> D<N> {
    debug_assert!(x.ge(&D::ONE.neg()));
    debug_assert!(x.le(&D::ONE));

    if x.abs().gt(&Reduction::K) {
        let x2 = mul(x, x);
        let y = div(
            x,
            mul(Consts::SQRT_2, sqrt(add(D::ONE, sqrt(sub(D::ONE, x2))))),
        );
        mul(D::TWO, asin_reduction(y))
    } else {
        taylor_series(x)
    }
}

#[inline]
const fn taylor_series<const N: usize>(x: D<N>) -> D<N> {
    debug_assert!(x.ge(&D::ONE.neg()));
    debug_assert!(x.le(&D::ONE));

    let mut result = D::ZERO;
    let mut result_next;
    let mut item = x;

    let x2 = mul(x, x);

    let mut i = 2;

    while i < Intrinsics::<N>::SERIES_MAX_ITERATIONS + 2 {
        result_next = add(result, item);

        if result.eq(&result_next) {
            break;
        }

        item = div(
            mul(mul(item, x2), from_u32((2 * i - 3) * (2 * i - 3))),
            from_u32((2 * i - 1) * (2 * i - 2)),
        );

        result = result_next;
        i += 1;
    }

    result.with_ctx(x.context())
}
