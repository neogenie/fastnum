use crate::decimal::unsigned::UnsignedDecimal;
use crate::{U128, U256, U512};

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal) => {
        impl PartialEq for UnsignedDecimal<$UINT> {
            #[inline]
            fn eq(&self, rhs: &UnsignedDecimal<$UINT>) -> bool {
                let a = self.normalized();
                let b = rhs.normalized();
                (a.scale == b.scale) && (a.value == b.value)
            }
        }

        impl Eq for UnsignedDecimal<$UINT> {}
    };
}

macro_impl!(U128, 128);
macro_impl!(U256, 256);
macro_impl!(U512, 512);
