use core::fmt::{self, Debug, Display, Formatter, LowerExp, UpperExp};

use crate::decimal::{utils, UnsignedDecimal};

impl<const N: usize> Display for UnsignedDecimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<const N: usize> LowerExp for UnsignedDecimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        LowerExp::fmt(&self.0, f)
    }
}

impl<const N: usize> UpperExp for UnsignedDecimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        UpperExp::fmt(&self.0, f)
    }
}

impl<const N: usize> Debug for UnsignedDecimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        utils::fmt::debug_print(
            &self.0.digits(),
            &self.0.control_block(),
            Self::type_name(),
            f,
        )
    }
}
