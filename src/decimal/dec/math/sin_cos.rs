use crate::decimal::{
    dec::math::{mul::mul, sin::sin, sqrt::sqrt, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn sin_cos<const N: usize>(x: D<N>) -> (D<N>, D<N>) {
    let sin = sin(x);
    debug_assert!(sin.le(&D::ONE));

    let cos = sqrt(sub(D::ONE, mul(sin, sin)));
    (sin, cos)
}
