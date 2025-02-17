use crate::{
    decimal::{
        dec::{math::add::add, scale::reduce},
        signals::Signals,
        Context, Decimal, Sign,
    },
    signals,
};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn overflow<const N: usize>(sign: Sign, signals: Signals, ctx: Context) -> D<N> {
    const OVERFLOW: Signals = signals![!OFW, !INEXACT, !ROUND];

    D::INFINITY
        .raise_signals(signals.combine(OVERFLOW))
        .set_ctx(ctx)
        .set_sign(sign)
}

#[inline(always)]
pub(crate) const fn underflow<const N: usize>(sign: Sign, signals: Signals, ctx: Context) -> D<N> {
    const UNDERFLOW: Signals = signals![!UFW, !INEXACT, !ROUND, !SN];

    D::ZERO
        .raise_signals(signals.combine(UNDERFLOW))
        .set_ctx(ctx)
        .set_sign(sign)
}

#[inline(always)]
pub(crate) const fn overflow_exp<const N: usize>(
    exp: i32,
    sign: Sign,
    signals: Signals,
    ctx: Context,
) -> D<N> {
    if exp > 0 {
        underflow(sign, signals, ctx)
    } else {
        overflow(sign, signals, ctx)
    }
}

#[inline(always)]
pub(crate) const fn is_even<const N: usize>(d: &D<N>) -> bool {
    if d.cb.get_scale() < 0 {
        true
    } else {
        d.digits.digits()[0] & 1 == 0
    }
}

#[inline(always)]
pub(crate) const fn is_odd<const N: usize>(d: &D<N>) -> bool {
    if d.cb.get_scale() < 0 {
        false
    } else {
        d.digits.digits()[0] & 1 == 1
    }
}

#[inline(always)]
pub(crate) const fn is_integral<const N: usize>(d: &D<N>) -> bool {
    reduce(*d).cb.get_scale() <= 0
}

#[inline(always)]
pub(crate) const fn correct<const N: usize>(d: D<N>, correction: D<N>) -> D<N> {
    if correction.is_zero() || correction.is_op_underflow() {
        return d;
    }

    let result = add(d, correction);

    if result.is_op_underflow() {
        d
    } else {
        result
    }
}
