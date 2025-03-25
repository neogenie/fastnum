use core::cmp::Ordering;

use crate::decimal::Decimal;

impl<const N: usize> PartialOrd for Decimal<N> {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.cmp(rhs)
    }
}
