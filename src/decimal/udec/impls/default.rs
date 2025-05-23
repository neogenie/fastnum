use crate::decimal::UnsignedDecimal;

impl<const N: usize> Default for UnsignedDecimal<N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}
