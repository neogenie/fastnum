use num_traits::{ConstOne, ConstZero, FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
use std::ops::Neg;

use crate::decimal::Decimal;

impl Zero for Decimal {
    #[inline]
    fn zero() -> Decimal {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl ConstZero for Decimal {
    const ZERO: Self = Self::ZERO;
}

impl One for Decimal {
    #[inline]
    fn one() -> Decimal {
        Self::ONE
    }
}

impl ConstOne for Decimal {
    const ONE: Self = Self::ONE;
}

