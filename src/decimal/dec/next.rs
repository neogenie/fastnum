use crate::{
    bint::{intrinsics::ExpType, UInt},
    decimal::{Decimal, Sign},
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn next_up<const N: usize>(d: D<N>) -> D<N> {
    if d.is_nan() {
        d
    } else if d.is_zero() {
        smallest(d, Sign::Plus)
    } else if d.is_negative() {
        toward_zero(d)
    } else {
        away_from_zero(d)
    }
}

#[inline]
pub(crate) const fn next_down<const N: usize>(d: D<N>) -> D<N> {
    if d.is_nan() {
        d
    } else if d.is_zero() {
        smallest(d, Sign::Minus)
    } else if d.is_negative() {
        away_from_zero(d)
    } else {
        toward_zero(d)
    }
}

/// The smallest non-zero magnitude with the given sign, keeping the context
/// and signals of `d`.
#[inline(always)]
const fn smallest<const N: usize>(d: D<N>, sign: Sign) -> D<N> {
    like(D::MIN_POSITIVE, &d).set_sign(sign)
}

/// `r` with the sign, context and signals of `d`.
#[inline(always)]
const fn like<const N: usize>(r: D<N>, d: &D<N>) -> D<N> {
    r.set_sign(d.sign())
        .set_ctx(d.context())
        .raise_signals(d.signals())
}

/// How many places finer the coefficient of a non-zero finite `d` can be
/// widened: as many as it holds, down to the smallest exponent. One unit in
/// the last place of the widened coefficient is the finest step from `d`.
#[inline(always)]
const fn widening<const N: usize>(d: &D<N>) -> ExpType {
    let digits = d.digits.remaining_decimal_digits();
    let room = (i16::MAX as i32 - d.cb.get_scale() as i32) as ExpType;

    if digits < room {
        digits
    } else {
        room
    }
}

/// `MAX / 10 + 1`: the coefficient just above `MAX` one place coarser.
#[inline(always)]
const fn above_tenth_of_max<const N: usize>() -> UInt<N> {
    UInt::<N>::MAX.div_digit(10).add(UInt::ONE)
}

/// The neighbour of a non-zero `d` one step further from zero.
#[inline]
const fn away_from_zero<const N: usize>(mut d: D<N>) -> D<N> {
    if d.is_infinite() {
        return d;
    }

    let scale = d.cb.get_scale();

    if d.digits.eq(&UInt::MAX) {
        // The coefficient cannot grow: step one place coarser, where the next
        // multiple of the coarser unit lies.
        if scale == i16::MIN {
            return like(D::INFINITY, &d);
        }
        d.digits = above_tenth_of_max();
        d.cb.set_scale(scale - 1);
    } else {
        let w = widening(&d);
        d.digits = d.digits.mul(UInt::power_of_ten(w)).add(UInt::ONE);
        d.cb.inc_scale(w);
    }

    d.cb.reset_extra_precision();
    d
}

/// The neighbour of a non-zero `d` one step closer to zero. A zero result
/// keeps the sign of `d`.
#[inline]
const fn toward_zero<const N: usize>(mut d: D<N>) -> D<N> {
    if d.is_infinite() {
        return like(D::MAX, &d);
    }

    let w = widening(&d);
    let widened = d.digits.mul(UInt::power_of_ten(w));
    let scale = d.cb.get_scale() as i32 + w as i32;

    // `MAX`, one place finer than the widened coefficient, lies between it
    // and that coefficient minus one, when the widened coefficient is
    // `MAX / 10 + 1`: in D64, `1844674407370955161.5` lies below
    // `1844674407370955162`.
    if scale < i16::MAX as i32 && widened.eq(&above_tenth_of_max()) {
        d.digits = UInt::MAX;
        d.cb.set_scale((scale + 1) as i16);
    } else {
        d.digits = widened.sub(UInt::ONE);
        d.cb.set_scale(scale as i16);
    }

    d.cb.reset_extra_precision();
    d
}
