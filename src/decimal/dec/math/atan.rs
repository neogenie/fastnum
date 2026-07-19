use core::cmp::Ordering;

use crate::decimal::{
    dec::math::{add::add, asin::asin, consts::Consts, div::div, mul::mul, sqrt::sqrt, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn atan<const N: usize>(x: D<N>) -> D<N> {
    if x.is_nan() {
        return x.op_invalid();
    }

    if x.is_zero() {
        return D::ZERO.with_ctx(x.context());
    }

    if x.is_infinite() {
        // atan(±∞) = ±π/2.
        return frac_pi_2_signed(x);
    }

    match x.cmp(&D::ONE.neg()) {
        Ordering::Less => return atan_reduce(x),
        Ordering::Equal => return Consts::FRAC_PI_4.neg(),
        Ordering::Greater => {}
    }

    match x.cmp(&D::ONE) {
        Ordering::Less => {}
        Ordering::Equal => {
            return Consts::FRAC_PI_4;
        }
        Ordering::Greater => return atan_reduce(x),
    }

    let x2 = mul(x, x);
    asin(div(x, sqrt(add(x2, D::<N>::ONE))))
}

// atan(x) = sign(x)·π/2 − atan(1/x) for |x| > 1; |1/x| < 1 reuses the accurate small-argument path.
#[inline]
const fn atan_reduce<const N: usize>(x: D<N>) -> D<N> {
    sub(frac_pi_2_signed(x), atan(div(D::ONE, x)))
}

#[inline]
const fn frac_pi_2_signed<const N: usize>(x: D<N>) -> D<N> {
    if x.is_negative() {
        Consts::FRAC_PI_2.neg()
    } else {
        Consts::FRAC_PI_2
    }
}
