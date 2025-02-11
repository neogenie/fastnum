use crate::decimal::{
    dec::math::{consts::Consts, div::div, ln::ln},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn log10<const N: usize>(d: D<N>) -> D<N> {
    div(ln(d), Consts::LN_10)
}
