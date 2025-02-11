use crate::decimal::{
    dec::math::{asin::asin, sub::sub},
    Decimal,
};
use crate::decimal::dec::math::consts::Consts;

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn acos<const N: usize>(x: D<N>) -> D<N> {
    sub(Consts::FRAC_PI_2, asin(x))
}
