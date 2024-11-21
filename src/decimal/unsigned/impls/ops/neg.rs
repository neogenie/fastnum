use core::ops::Neg;

use crate::decimal::{signed::Decimal, unsigned::UnsignedDecimal};

impl<const N: usize> Neg for UnsignedDecimal<N> {
    type Output = Decimal<N>;

    #[inline]
    fn neg(self) -> Decimal<N> {
        self.neg()
    }
}