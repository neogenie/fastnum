use core::hash::{Hash, Hasher};

use crate::decimal::Decimal;

impl<const N: usize> Hash for Decimal<N> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let normalized = self.reduce();
        normalized.digits.hash(state);
        normalized.cb.hash(state);
    }
}
