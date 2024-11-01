use std::fmt::{self, Debug, Display, Formatter};

use crate::decimal::signed::Decimal;
use crate::decimal::unsigned::UnsignedDecimal;

impl<UINT> Display for Decimal<UINT>
where
    UnsignedDecimal<UINT>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.sign, self.value)
    }
}

impl<UINT> Debug for Decimal<UINT>
where
    UnsignedDecimal<UINT>: Debug,
    Decimal<UINT>: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}", self)
        } else {
            write!(
                f,
                "Decimal(sign = {}, value = {:?})",
                self.sign, self.value
            )
        }
    }
}
