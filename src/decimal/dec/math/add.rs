use core::cmp::Ordering;

use crate::decimal::{
    dec::{
        math::sub::sub_abs,
        scale::{extend_scale_to, rescale},
        utils,
    },
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn add<const N: usize>(lhs: D<N>, rhs: D<N>) -> D<N> {
    if lhs.is_nan() {
        return lhs.compound(&rhs).op_invalid();
    }

    if rhs.is_nan() {
        return rhs.compound(&lhs).op_invalid();
    }

    match (lhs.cb.is_negative(), rhs.cb.is_negative()) {
        (false, false) => add_abs(lhs, rhs),
        (true, true) => add_abs(rhs.neg(), lhs.neg()).neg(),
        (false, true) => sub_abs(lhs, rhs.neg()),
        (true, false) => sub_abs(rhs, lhs.neg()),
    }
}

#[inline]
pub(crate) const fn add_abs<const N: usize>(lhs: D<N>, rhs: D<N>) -> D<N> {
    debug_assert!(!lhs.is_negative() && !rhs.is_negative());

    if lhs.is_infinite() {
        return lhs.compound(&rhs);
    }

    if rhs.is_infinite() {
        return rhs.compound(&lhs);
    }

    if rhs.is_zero() {
        return extend_scale_to(lhs, rhs.cb.get_scale()).compound(&rhs);
    }

    if lhs.is_zero() {
        return extend_scale_to(rhs, lhs.cb.get_scale()).compound(&lhs);
    }

    match lhs.cb.scale_cmp(&rhs.cb) {
        Ordering::Less => add_rescale(lhs, rhs),
        Ordering::Equal => add_aligned(lhs, rhs),
        Ordering::Greater => add_rescale(rhs, lhs),
    }
}

#[inline]
const fn add_rescale<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    rescale(&mut lhs, rhs.cb.get_scale());

    if lhs.is_op_clamped() {
        rescale(&mut rhs, lhs.cb.get_scale());
        add_aligned(lhs, rhs)
    } else {
        add_aligned(lhs, rhs)
    }
}

#[inline]
const fn add_aligned<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    debug_assert!(lhs.cb.get_scale() == rhs.cb.get_scale());

    let (digits, overflow) = lhs.digits.overflowing_add(rhs.digits);

    if !overflow {
        lhs.digits = digits;
        utils::add_extra_precision(&mut lhs, &rhs);
        lhs.compound(&rhs)
    } else if let (scale, false) = lhs.cb.get_scale().overflowing_sub(1) {
        rescale(&mut lhs, scale);
        rescale(&mut rhs, scale);
        add_aligned(lhs, rhs)
    } else {
        lhs.compound(&rhs).op_overflow()
    }
}
