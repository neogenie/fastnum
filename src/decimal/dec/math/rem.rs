use core::cmp::Ordering;

use crate::{
    bint::{intrinsics::min, UInt},
    decimal::{
        dec::{scale::rescale, ExtraPrecision},
        Decimal,
    },
};

type D<const N: usize> = Decimal<N>;
type U<const N: usize> = UInt<N>;

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
            let power = (rhs.cb.get_scale() as i32 - lhs.cb.get_scale() as i32) as u32;

            if lhs.cb.is_special()
                || rhs.digits.is_zero()
                || power <= lhs.digits.remaining_decimal_digits()
            {
                rescale(&mut lhs, rhs.cb.get_scale());
            } else {
                // The dividend's coefficient, rescaled to the divisor's exponent, would
                // overflow: take the remainder by long division instead.
                lhs.digits = shifted_rem(lhs.digits, power, rhs.digits);
                lhs.cb.set_scale(rhs.cb.get_scale());
                lhs.cb.set_extra_precision(ExtraPrecision::new());
            }
        }
        Ordering::Greater => {
            rescale(&mut rhs, lhs.cb.get_scale());
        }
    }

    rhs.cb.abs();
    lhs.cb.compound(&rhs.cb);

    D::new(lhs.digits.rem(rhs.digits), lhs.cb)
}

/// `(digits × 10^power) mod divisor`, exactly, without overflow.
///
/// Long division: the partial remainder stays below the divisor, and each step
/// brings down as many zero digits as a power of ten in `UInt<N>` holds.
#[inline]
const fn shifted_rem<const N: usize>(digits: U<N>, mut power: u32, divisor: U<N>) -> U<N> {
    let mut remainder = digits.rem(divisor);

    while power > 0 && !remainder.is_zero() {
        let step = min(power, U::<N>::MAX_POWER_OF_TEN);
        (_, remainder) = remainder.mul_div_rem(U::<N>::power_of_ten(step), divisor);
        power -= step;
    }

    remainder
}
