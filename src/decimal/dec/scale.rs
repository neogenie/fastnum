use crate::{
    decimal::{
        dec::{
            construct::construct,
            intrinsics::{clength, Intrinsics},
            math::add::add,
            ExtraPrecision,
        },
        signals::Signals,
        Context, Decimal, Sign,
    },
    int::{
        math::{div_rem, div_rem_digit, strict_mul10},
        UInt,
    },
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn extend_scale_to<const N: usize>(mut d: D<N>, new_scale: i16) -> D<N> {
    if new_scale > d.cb.get_scale() {
        rescale(&mut d, new_scale)
    }
    d
}

#[inline]
pub(crate) const fn rescale<const N: usize>(d: &mut D<N>, new_scale: i16) {
    if d.cb.is_special() {
        d.cb.signaling_nan();
        return;
    }

    let scale = d.cb.get_scale();

    if new_scale > scale {
        // Re-scale up. Multiply Coefficient by 10, increase scale: 10 -> 100e-1
        rescale_up(d, new_scale);
    } else if new_scale < scale {
        // Re-scale down. Divide Coefficient by 10 with rounding, decrease scale: 1234
        // -> 123e+1
        rescale_down(d, new_scale);
    }
}

#[inline]
pub(crate) const fn quantum<const N: usize>(exp: i32, ctx: Context) -> D<N> {
    construct(
        UInt::ONE,
        exp,
        Sign::Plus,
        Signals::empty(),
        ctx,
        ExtraPrecision::new(),
    )
}

#[inline]
pub(crate) const fn reduce<const N: usize>(mut d: D<N>) -> D<N> {
    if d.cb.is_special() {
        return d.signaling_nan();
    }

    if d.has_extra_precision() {
        return d;
    }

    if d.digits.is_zero() {
        d.cb.set_scale(0);
    } else {
        let mut digits;
        let mut remainder;
        while !d.digits.is_zero() {
            (digits, remainder) = div_rem(d.digits, UInt::TEN);
            if remainder.is_zero() {
                if d.cb.get_scale() > i16::MIN {
                    d.digits = digits;
                    d.cb.dec_scale(1);
                } else {
                    return d.raise_signals(Signals::OP_SUBNORMAL);
                }
            } else {
                break;
            }
        }
    }

    d
}

#[inline]
pub(crate) const fn quantize<const N: usize>(mut d: D<N>, other: D<N>) -> D<N> {
    if d.is_infinite() && other.is_infinite() {
        d
    } else if d.cb.is_special() || other.cb.is_special() {
        d.signaling_nan()
    } else {
        let scale = other.cb.get_scale();

        rescale(&mut d, scale);
        d.cb.quiet_signals(Signals::OP_UNDERFLOW);

        if d.cb.get_scale() != scale {
            d.signaling_nan()
        } else {
            d
        }
    }
}

#[inline]
const fn rescale_up<const N: usize>(d: &mut D<N>, new_scale: i16) {
    debug_assert!(new_scale > d.cb.get_scale());

    let mpower = (new_scale as i32 - d.cb.get_scale() as i32) as u32;

    if d.digits.is_zero() {
        d.cb.set_scale(new_scale);

        let mut extra_precision = d.cb.get_extra_precision();

        if let Some(correction) = extra_precision.scale_up(mpower) {
            correct(d, correction);
        }

        return d.cb.set_extra_precision(extra_precision);
    }

    let clength = clength(d.digits);
    let max_gap = Intrinsics::<N>::MAX_CLENGTH - clength;

    if max_gap < 1 {
        return d.cb.raise_signals(Signals::OP_CLAMPED);
    }

    if mpower < max_gap {
        return rescale_up_unchecked(d, mpower);
    }

    if max_gap >= 2 {
        rescale_up_unchecked(d, max_gap - 1);
    }

    while new_scale > d.cb.get_scale() {
        if d.digits.gt(&Intrinsics::<N>::COEFF_MEDIUM) {
            return d.cb.raise_signals(Signals::OP_CLAMPED);
        } else {
            rescale_up_unchecked(d, 1);
        }
    }
}

#[inline]
const fn rescale_down<const N: usize>(d: &mut D<N>, new_scale: i16) {
    debug_assert!(new_scale < d.cb.get_scale());
    d.cb.raise_signals(Signals::OP_ROUNDED);

    let mut extra_precision = d.cb.get_extra_precision();

    // TODO: performance optimization
    // - fast zero scale extra precision
    // - replace iterative division with faster 10^N
    let mut extra_digit;
    while new_scale < d.cb.get_scale() {
        if !d.digits.is_zero() {
            (d.digits, extra_digit) = div_rem_digit(d.digits, 10);

            if extra_digit != 0 {
                d.cb.raise_signals(Signals::OP_INEXACT);
            }

            extra_precision.push_digit(extra_digit);
        } else {
            extra_precision.push_digit(0);
        }

        d.cb.dec_scale(1);
    }

    d.cb.set_extra_precision(extra_precision);
}

#[inline]
const fn rescale_up_unchecked<const N: usize>(d: &mut D<N>, gap: u32) {
    if gap == 1 {
        d.digits = d.digits.strict_mul(UInt::<N>::TEN);
    } else {
        d.digits = strict_mul10(d.digits, gap);
    }

    d.cb.inc_scale(gap as i16);

    let mut extra_precision = d.cb.get_extra_precision();

    if let Some(correction) = extra_precision.scale_up(gap) {
        correct(d, correction);
    }

    d.cb.set_extra_precision(extra_precision);
}

#[inline]
const fn correct<const N: usize>(d: &mut D<N>, mut correction: D<N>) {
    correction.set_sign(d.sign());
    correction.cb.inc_scale(d.cb.get_scale());
    *d = add(*d, correction);
}
