use crate::decimal::{
    dec::math::{add::add, div::div, ln::ln, mul::mul, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn atanh<const N: usize>(x: D<N>) -> D<N> {
    if x.is_nan() {
        return x.op_invalid();
    }

    if x.is_zero() {
        return D::ZERO.with_ctx(x.context());
    }

    if x.is_infinite() {
        return x.signaling_nan();
    }

    if x.abs().gt(&D::ONE) {
        return x.signaling_nan();
    }

    mul(D::HALF, ln(div(add(D::ONE, x), sub(D::ONE, x))))
}
