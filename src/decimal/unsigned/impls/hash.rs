use std::hash::{Hash, Hasher};

use crate::decimal::unsigned::UnsignedDecimal;
use crate::{U128, U256, U512};

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal) => {
        impl Hash for UnsignedDecimal<$UINT> {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                let a = self.normalized();
                a.value.hash(state);
                a.scale.hash(state);
            }
        }
    };
}

macro_impl!(U128, 128);
macro_impl!(U256, 256);
macro_impl!(U512, 512);