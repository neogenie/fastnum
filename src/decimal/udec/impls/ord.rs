use core::cmp::Ordering;

use crate::decimal::UnsignedDecimal;

impl<const N: usize> PartialOrd for UnsignedDecimal<N> {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.cmp(rhs)
    }
}
