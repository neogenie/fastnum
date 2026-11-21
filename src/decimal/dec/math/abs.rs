use crate::decimal::Decimal;

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn abs<const N: usize>(mut d: D<N>) -> D<N> {
    d.cb.abs();
    d
}
