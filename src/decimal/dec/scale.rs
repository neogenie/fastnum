use crate::{
    decimal::{
        dec::{
            construct::construct,
            intrinsics::{clength, Intrinsics},
            math::add::add,
            ControlBlock, ExtraPrecision,
        },
        Context, Decimal, Signal,
    },
    int::{
        math::{div_rem, div_rem_digit, strict_mul10},
        UInt,
    },
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn extend_scale_to<const N: usize>(d: D<N>, new_scale: i16) -> D<N> {
    if new_scale > d.scale {
        rescale(d, new_scale)
    } else {
        d
    }
}

#[inline]
pub(crate) const fn rescale<const N: usize>(mut d: D<N>, new_scale: i16) -> D<N> {
    if d.cb.is_special() {
        return d.signaling_nan();
    }

    // FIXME: extra precision <->
    if d.digits.is_zero() {
        d.scale = new_scale;
        return d;
    }

    if new_scale == d.scale {
        d
    } else if new_scale > d.scale {
        // Re-scale up. Multiply Coefficient by 10, increase scale: 10 -> 100e-1
        rescale_up(d, new_scale)
    } else {
        // Re-scale down. Divide Coefficient by 10 with rounding, decrease scale: 1234
        // -> 123e+1
        rescale_down(d, new_scale)
    }
}

#[inline]
pub(crate) const fn quantum<const N: usize>(exp: i32, ctx: Context) -> D<N> {
    let cb = ControlBlock::default().set_context(ctx);
    construct(UInt::ONE, exp, cb, ExtraPrecision::new())
}

#[inline]
pub(crate) const fn reduce<const N: usize>(mut d: D<N>) -> D<N> {
    if d.cb.is_special() {
        return d.raise_signal(Signal::OP_INVALID);
    }

    if d.digits.is_zero() {
        d.scale = 0;
    } else {
        let mut digits;
        let mut remainder;
        while !d.digits.is_zero() {
            (digits, remainder) = div_rem(d.digits, UInt::TEN);
            if remainder.is_zero() {
                if d.scale > i16::MIN {
                    d.digits = digits;
                    d.scale -= 1;
                } else {
                    return d.raise_signal(Signal::OP_SUBNORMAL);
                }
            } else {
                break;
            }
        }
    }

    d
}

#[inline]
pub(crate) const fn quantize<const N: usize>(d: D<N>, other: D<N>) -> D<N> {
    if d.is_infinite() && other.is_infinite() {
        d
    } else if d.cb.is_special() || other.cb.is_special() {
        d.signaling_nan()
    } else {
        let res = rescale(d, other.scale).quiet_signal(Signal::OP_UNDERFLOW);

        if res.scale != other.scale {
            d.signaling_nan()
        } else {
            res
        }
    }
}

#[inline]
const fn rescale_up<const N: usize>(mut d: D<N>, new_scale: i16) -> D<N> {
    debug_assert!(new_scale > d.scale);

    let mpower = (new_scale as i32 - d.scale as i32) as u32;
    let clength = clength(d.digits);
    let max_gap = Intrinsics::<N>::MAX_CLENGTH - clength;

    if max_gap < 1 {
        return d.raise_signal(Signal::OP_CLAMPED);
    }

    let mut correction;

    if mpower < max_gap {
        d.digits = strict_mul10(d.digits, mpower);
        d.scale += mpower as i16;

        (d.extra_precision, correction) = d.extra_precision.overflowing_scale(mpower as i16);
        correction.scale += d.scale;
        return add(d, correction);
    }

    if max_gap >= 2 {
        d.digits = strict_mul10(d.digits, max_gap - 1);
        d.scale += (max_gap - 1) as i16;

        (d.extra_precision, correction) = d.extra_precision.overflowing_scale((max_gap - 1) as i16);
        correction.scale += d.scale;
        d = add(d, correction);
    }

    while new_scale > d.scale {
        if d.digits.gt(&Intrinsics::<N>::COEFF_MEDIUM) {
            return d.raise_signal(Signal::OP_CLAMPED);
        } else {
            d.digits = d.digits.strict_mul(UInt::<N>::TEN);
            d.scale += 1;

            (d.extra_precision, correction) = d.extra_precision.overflowing_scale(1);
            correction.scale += d.scale;
            d = add(d, correction);
        }
    }
    d
}

#[inline]
const fn rescale_down<const N: usize>(mut d: D<N>, new_scale: i16) -> D<N> {
    debug_assert!(new_scale < d.scale);
    d.cb = d.cb.raise_signal(Signal::OP_ROUNDED);

    // TODO: performance optimization
    let mut extra_digit;
    while new_scale < d.scale {
        if !d.digits.is_zero() {
            (d.digits, extra_digit) = div_rem_digit(d.digits, 10);

            if extra_digit != 0 {
                d.cb = d.cb.raise_signal(Signal::OP_INEXACT);
            }

            d.extra_precision = d.extra_precision.push(extra_digit);
        } else {
            d.extra_precision = d.extra_precision.push(0);
        }

        d.scale -= 1;
    }

    d
}
