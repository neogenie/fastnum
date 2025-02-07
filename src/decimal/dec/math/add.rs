use core::cmp::Ordering;

use crate::decimal::{
    dec::{
        math::{sub::sub_abs, utils::magnitude_inc},
        scale::{extend_scale_to, rescale},
    },
    Decimal, Signal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn add<const N: usize>(lhs: D<N>, rhs: D<N>) -> D<N> {
    if lhs.is_nan() {
        return lhs.compound_and_raise(&rhs, Signal::OP_INVALID);
    }

    if rhs.is_nan() {
        return rhs.compound_and_raise(&lhs, Signal::OP_INVALID);
    }

    match (lhs.is_negative(), rhs.is_negative()) {
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
        return extend_scale_to(lhs, rhs.scale).compound(&rhs);
    }

    if lhs.is_zero() {
        return extend_scale_to(rhs, lhs.scale).compound(&lhs);
    }

    match lhs.scale_cmp(&rhs) {
        Ordering::Less => add_rescale(lhs, rhs),
        Ordering::Equal => add_aligned(lhs, rhs),
        Ordering::Greater => add_rescale(rhs, lhs),
    }
}

#[inline]
const fn add_rescale<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    lhs = rescale(lhs, rhs.scale);

    if lhs.is_op_clamped() {
        rhs = rescale(rhs, lhs.scale);
        add_aligned(lhs, rhs)
    } else {
        add_aligned(lhs, rhs)
    }
}

#[inline]
const fn add_aligned<const N: usize>(mut lhs: D<N>, rhs: D<N>) -> D<N> {
    debug_assert!(lhs.scale == rhs.scale);

    let mut overflow;
    let digits;

    (digits, overflow) = lhs.digits.overflowing_add(rhs.digits);

    if !overflow {
        lhs.digits = digits;

        (lhs.extra_precision, overflow) = lhs.extra_precision.overflowing_add(rhs.extra_precision);

        if overflow {
            lhs = magnitude_inc(lhs);
        }

        lhs.compound(&rhs)
    } else if let (scale, false) = lhs.scale.overflowing_sub(1) {
        add_aligned(rescale(lhs, scale), rescale(rhs, scale))
    } else {
        lhs.compound_and_raise(&rhs, Signal::OP_OVERFLOW)
    }
}
