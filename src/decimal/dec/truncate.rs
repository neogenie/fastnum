use crate::{
    bint::{intrinsics::ExpType, UInt},
    decimal::{dec::scale, Decimal, Sign},
};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn truncate<const N: usize>(mut d: D<N>, scale: i16) -> D<N> {
    // TODO: performance optimization
    // We do not need to keep the extra precision
    scale::rescale(&mut d, scale);
    d.cb.reset_extra_precision();
    d
}

#[inline]
pub(crate) const fn fract<const N: usize>(mut d: D<N>) -> D<N> {
    if d.cb.is_special() {
        return d.signaling_nan();
    }

    let scale = d.cb.get_scale();

    if scale <= 0 {
        d.digits = UInt::ZERO;
        d.cb.set_scale(0);
        d.cb.reset_extra_precision();
    } else if let Some(unit) = UInt::checked_power_of_ten(scale as ExpType) {
        d.digits = d.digits.rem(unit);
    }

    // `self - self.trunc()` is `+0` when `self` is integral.
    if d.is_zero() {
        d.cb.set_sign(Sign::Plus);
    }

    d
}
