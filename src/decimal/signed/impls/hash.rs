use std::hash::{Hash, Hasher};

use crate::decimal::signed::{Decimal};
use crate::decimal::unsigned::UnsignedDecimal;

impl<UINT> Hash for Decimal<UINT>
where
    UnsignedDecimal<UINT>: Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.sign.hash(state);
    }
}