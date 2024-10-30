use std::cmp::Ordering;

use crate::decimal::signed::{Decimal, Sign};
use crate::decimal::unsigned::UnsignedDecimal;

impl<UINT> PartialOrd for Decimal<UINT>
where
    UnsignedDecimal<UINT>: Ord,
{
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<UINT> Ord for Decimal<UINT>
where
    UnsignedDecimal<UINT>: Ord,
{
    #[inline]
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self.sign, rhs.sign) {
            (Sign::NoSign, Sign::Minus) | (Sign::Plus, Sign::Minus) => Ordering::Greater,
            (Sign::Minus, Sign::NoSign) | (Sign::Minus, Sign::Plus) => Ordering::Less,
            (Sign::Minus, Sign::Minus) => self.value.cmp(&rhs.value).reverse(),
            (Sign::Plus, Sign::NoSign) => self.value.cmp(&rhs.value).then(Ordering::Greater),
            (Sign::NoSign, Sign::Plus) => self.value.cmp(&rhs.value).then(Ordering::Less),
            (Sign::NoSign, Sign::NoSign) | (Sign::Plus, Sign::Plus) => self.value.cmp(&rhs.value),
        }
    }
}
