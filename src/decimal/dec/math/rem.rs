use core::cmp::Ordering;

use crate::decimal::{dec::scale::rescale, Decimal};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn rem<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    if lhs.is_nan() {
        return lhs.compound(&rhs).op_invalid();
    }

    if rhs.is_nan() {
        return rhs.compound(&lhs).op_invalid();
    }

    if lhs.abs().lt(&rhs.abs()) {
        return lhs;
    }

    match lhs.cb.scale_cmp(&rhs.cb) {
        Ordering::Equal => {}
        Ordering::Less => {
            rescale(&mut lhs, rhs.cb.get_scale());
        }
        Ordering::Greater => {
            rescale(&mut rhs, lhs.cb.get_scale());
        }
    }

    rhs.cb.abs();
    lhs.cb.compound(&rhs.cb);

    D::new(lhs.digits.rem(rhs.digits), lhs.cb)
}
