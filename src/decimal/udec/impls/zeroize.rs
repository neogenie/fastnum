use crate::decimal::UnsignedDecimal;

impl<const N: usize> zeroize::DefaultIsZeroes for UnsignedDecimal<N> {}
