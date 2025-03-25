use core::cmp::Ordering;

use crate::decimal::{
    dec::math::{add::add, asin::asin, consts::Consts, div::div, mul::mul, sqrt::sqrt},
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
        return x.signaling_nan();
    }

    match x.cmp(&D::ONE.neg()) {
        Some(Ordering::Less) => {
            return x.signaling_nan();
        }
        Some(Ordering::Equal) => return Consts::FRAC_PI_4.neg(),
        Some(Ordering::Greater) => {},
        None => return x.signaling_nan(),
    }

    match x.cmp(&D::ONE) {
        Some(Ordering::Less) => {}
        Some(Ordering::Equal) => {
            return Consts::FRAC_PI_4;
        }
        Some(Ordering::Greater) => {
            return x.signaling_nan();
        }
        None => return x.signaling_nan(),
    }

    let x2 = mul(x, x);
    asin(div(x, sqrt(add(x2, D::<N>::ONE))))
}
