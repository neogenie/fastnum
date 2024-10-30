use crate::decimal::signed::Decimal;
use crate::decimal::unsigned::UnsignedDecimal;

impl<UINT> PartialEq for Decimal<UINT>
where
    UnsignedDecimal<UINT>: PartialEq,
{
    #[inline]
    fn eq(&self, rhs: &Decimal<UINT>) -> bool {
        (self.sign == rhs.sign) && (self.value == rhs.value)
    }
}

impl<UINT> Eq for Decimal<UINT> where UnsignedDecimal<UINT>: PartialEq {}
