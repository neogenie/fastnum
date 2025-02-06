use core::cmp::Ordering;

use crate::{
    decimal::{dec::scale::reduce, Decimal},
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn eq<const N: usize>(lhs: &D<N>, rhs: &D<N>) -> bool {
    if lhs.is_nan() || rhs.is_nan() {
        return false;
    }

    if lhs.is_negative() ^ rhs.is_negative() {
        return false;
    }

    match (lhs.is_infinite(), rhs.is_infinite()) {
        (false, true) => {
            return false;
        }
        (true, false) => {
            return false;
        }
        (true, true) => {
            return true;
        }
        (false, false) => {}
    }

    let lhs = reduce(*lhs);
    let rhs = reduce(*rhs);

    (lhs.scale == rhs.scale) && (lhs.digits.eq(&rhs.digits))
}

#[inline]
pub(crate) const fn ne<const N: usize>(lhs: &D<N>, rhs: &D<N>) -> bool {
    !lhs.eq(rhs)
}

#[inline]
pub(crate) const fn cmp<const N: usize>(lhs: &D<N>, rhs: &D<N>) -> Ordering {
    match (lhs.is_negative(), rhs.is_negative()) {
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        (true, true) => cmp_magnitude(lhs, rhs).reverse(),
        (false, false) => cmp_magnitude(lhs, rhs),
    }
}

#[inline]
const fn cmp_magnitude<const N: usize>(lhs: &D<N>, rhs: &D<N>) -> Ordering {
    match (lhs.is_zero(), rhs.is_zero()) {
        (true, true) => {
            return Ordering::Equal;
        }
        (true, false) => {
            return Ordering::Less;
        }
        (false, true) => {
            return Ordering::Greater;
        }
        (_, _) => {}
    }

    let a = reduce(*lhs);
    let b = reduce(*rhs);

    if a.scale == b.scale {
        return a.digits.cmp(&b.digits);
    }

    let a_exp = a.power();
    let b_exp = b.power();

    if a_exp == b_exp {
        if a.scale > b.scale {
            let (mul, false) = UInt::TEN.overflowing_pow((a.scale - b.scale) as u32) else {
                return Ordering::Less;
            };

            let (value, false) = b.digits.overflowing_mul(mul) else {
                return Ordering::Less;
            };

            a.digits.cmp(&value)
        } else {
            let (mul, false) = UInt::TEN.overflowing_pow((b.scale - a.scale) as u32) else {
                return Ordering::Less;
            };

            let (value, false) = a.digits.overflowing_mul(mul) else {
                return Ordering::Less;
            };

            value.cmp(&b.digits)
        }
    } else if a_exp > b_exp {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}
