use core::cmp::Ordering;

use crate::decimal::{
    dec::{scale::rescale, ExtraPrecision},
    Decimal, Signal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn rem<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    if lhs.is_nan() {
        return lhs.compound_and_raise(&rhs, Signal::OP_INVALID);
    }

    if rhs.is_nan() {
        return rhs.compound_and_raise(&lhs, Signal::OP_INVALID);
    }

    if lhs.abs().lt(&rhs.abs()) {
        return lhs;
    }

    match lhs.scale_cmp(&rhs) {
        Ordering::Equal => {}
        Ordering::Less => {
            lhs = rescale(lhs, rhs.scale);
        }
        Ordering::Greater => {
            rhs = rescale(rhs, lhs.scale);
        }
    }

    D::new(
        lhs.digits.rem(rhs.digits),
        lhs.scale,
        lhs.cb.combine(rhs.cb.abs()),
        ExtraPrecision::new(),
    )
}
