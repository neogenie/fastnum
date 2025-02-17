use crate::decimal::{
    dec::math::{add::add, ln::ln, mul::mul, sqrt::sqrt, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn acosh<const N: usize>(x: D<N>) -> D<N> {
    if x.is_nan() {
        return x.op_invalid();
    }

    if x.is_zero() {
        return D::ZERO.with_ctx(x.context());
    }

    if x.is_infinite() {
        return x.signaling_nan();
    }

    if x.le(&D::ONE) {
        return x.signaling_nan();
    }

    ln(add(x, sqrt(sub(mul(x, x), D::ONE))))
}
