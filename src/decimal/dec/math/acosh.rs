use crate::decimal::{
    dec::math::{add::add, log::ln, mul::mul, sqrt::sqrt, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn acosh<const N: usize>(x: D<N>) -> D<N> {
    if x.is_nan() {
        return x.op_invalid();
    }

    if x.is_infinite() {
        // acosh(+∞) = +∞; acosh(−∞) is out of domain.
        return if x.is_negative() {
            x.signaling_nan()
        } else {
            x
        };
    }

    if x.lt(&D::ONE) {
        return x.signaling_nan();
    }

    // acosh(1) = 0 falls out of the formula: ln(1 + √0) = 0.
    ln(add(x, sqrt(sub(mul(x, x), D::ONE))))
}
