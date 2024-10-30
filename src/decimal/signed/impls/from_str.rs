use std::str::FromStr;

use crate::decimal::{ParseError, signed::{Decimal, parse}};

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal, $name: ident) => {
        impl FromStr for Decimal<crate::$UINT> {
            type Err = ParseError;
        
            #[inline]
            fn from_str(s: &str) -> Result<Decimal<crate::$UINT>, ParseError> {
                parse::$name::from_str(s)
            }
        }
    }
}

macro_impl!(U128, 128, d128);
macro_impl!(U256, 256, d256);
macro_impl!(U512, 512, d512);