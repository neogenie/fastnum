use core::ops::Neg;

use crate::decimal::Decimal;

impl<const N: usize> Neg for Decimal<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        self.neg()
    }
}
