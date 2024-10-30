use std::str::FromStr;

use crate::decimal::{ParseError, unsigned::{UnsignedDecimal, parse}};

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal, $name: ident) => {
        impl FromStr for UnsignedDecimal<crate::$UINT> {
            type Err = ParseError;
        
            #[inline]
            fn from_str(s: &str) -> Result<UnsignedDecimal<crate::$UINT>, ParseError> {
                parse::$name::from_str(s)
            }
        }
    }
}

macro_impl!(U128, 128, d128);
macro_impl!(U256, 256, d256);
macro_impl!(U512, 512, d512);