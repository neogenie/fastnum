use crate::{
    decimal::{
        dec::{
            math::{add::add, sub::sub},
            scale::reduce,
            ControlBlock, ExtraPrecision,
        },
        Decimal,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn overflow<const N: usize>(cb: ControlBlock) -> D<N> {
    D::INFINITY
        .with_ctx(cb.context())
        .with_cb(cb.raise_signal(Signal::overflow()))
}

#[inline(always)]
pub(crate) const fn underflow<const N: usize>(cb: ControlBlock) -> D<N> {
    D::ZERO
        .with_ctx(cb.context())
        .with_cb(cb.raise_signal(Signal::underflow()))
}

#[inline]
pub(crate) const fn overflow_exp<const N: usize>(exp: i32, cb: ControlBlock) -> D<N> {
    if exp > 0 {
        underflow(cb)
    } else {
        overflow(cb)
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
