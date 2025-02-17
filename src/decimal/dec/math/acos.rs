use crate::decimal::{
    dec::math::{asin::asin, consts::Consts, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn acos<const N: usize>(x: D<N>) -> D<N> {
    sub(Consts::FRAC_PI_2, asin(x))
}
