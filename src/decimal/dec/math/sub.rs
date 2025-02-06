use core::cmp::Ordering;

use crate::{
    decimal::{
        dec::{
            math::{add::add_abs, utils::magnitude_dec},
            scale::{extend_scale_to, rescale},
        },
        Decimal, Signal,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn sub<const N: usize>(lhs: D<N>, rhs: D<N>) -> D<N> {
    if lhs.is_nan() {
        return lhs.compound_and_raise(&rhs, Signal::OP_INVALID);
    }

    if rhs.is_nan() {
        return rhs.compound_and_raise(&lhs, Signal::OP_INVALID);
    }

    match (lhs.is_negative(), rhs.is_negative()) {
        (false, false) => sub_abs(lhs, rhs),
        (true, true) => sub_abs(rhs.neg(), lhs.neg()),
        (false, true) => add_abs(lhs, rhs.neg()),
        (true, false) => add_abs(lhs.neg(), rhs).neg(),
    }
}

#[inline]
pub(crate) const fn sub_abs<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    debug_assert!(!lhs.is_negative() && !rhs.is_negative());

    if lhs.is_infinite() && rhs.is_infinite() {
        let cb = lhs.cb.combine(rhs.cb);
        return D::NAN.with_cb(cb.raise_signal(Signal::OP_INVALID));
    } else if lhs.is_infinite() {
        return lhs.compound(&rhs);
    } else if rhs.is_infinite() {
        return rhs.compound(&lhs).neg();
    }

    if rhs.is_zero() {
        return extend_scale_to(lhs.compound(&rhs), rhs.scale);
    }

    if lhs.is_zero() {
        return extend_scale_to(rhs.compound(&lhs), lhs.scale).neg();
    }

    match lhs.scale_cmp(&rhs) {
        Ordering::Equal => sub_aligned(lhs, rhs),
        Ordering::Less => {
            lhs = rescale(lhs, rhs.scale);

            if lhs.is_op_clamped() {
                rhs = rescale(rhs, lhs.scale);
                sub_aligned(lhs, rhs)
            } else {
                sub_aligned(lhs, rhs)
            }
        }
        Ordering::Greater => {
            rhs = rescale(rhs, lhs.scale);

            if rhs.is_op_clamped() {
                lhs = rescale(lhs, rhs.scale);
                sub_aligned(lhs, rhs)
            } else {
                sub_aligned(lhs, rhs)
            }
        }
    }
}

#[inline]
const fn sub_aligned<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    debug_assert!(lhs.scale == rhs.scale);

    let overflow;

    match lhs.digits.cmp(&rhs.digits) {
        Ordering::Less => {
            rhs.digits = rhs.digits.strict_sub(lhs.digits);

            (rhs.extra_precision, overflow) =
                rhs.extra_precision.overflowing_sub(lhs.extra_precision);

            if overflow {
                rhs = magnitude_dec(rhs);
            }

            rhs.compound(&lhs).neg()
        }
        Ordering::Equal => {
            (lhs.extra_precision, overflow) =
                lhs.extra_precision.overflowing_sub(rhs.extra_precision);

            let mut d = D::new(
                UInt::ZERO,
                lhs.scale,
                lhs.cb.compound(rhs.cb),
                lhs.extra_precision,
            );

            if overflow {
                d = magnitude_dec(d);
            }

            d
        }
        Ordering::Greater => {
            lhs.digits = lhs.digits.strict_sub(rhs.digits);

            (lhs.extra_precision, overflow) =
                lhs.extra_precision.overflowing_sub(rhs.extra_precision);

            if overflow {
                lhs = magnitude_dec(lhs);
            }

            lhs.compound(&rhs)
        }
    }
}
