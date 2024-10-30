macro_rules! macro_impl {
    ($UINT: ident, $bits: literal, $name: ident) => {
        pub mod $name {
            
            use crate::$UINT;
            use crate::int::error::from_int_error_kind;
            use crate::int::{ParseError};
            
            #[inline]
            pub const fn parse_str(s: &str) -> $UINT {
                match from_str(s) {
                    Ok(n) => n,
                    Err(e) => panic!("{}", e.description()),
                }
            }
            
            #[inline]
            const fn from_str(s: &str) -> Result<$UINT, ParseError> {
                if let Some(val) = const_str::strip_prefix!(s, "0x") {
                    from_str_radix(val, 16)
                } else {
                    from_str_radix(s, 10)
                }
            }
            
            #[inline]
            const fn from_str_radix(s: &str, radix: u32) -> Result<$UINT, ParseError> {
                match $UINT::from_str_radix(s, radix) {
                    Ok(val) => Ok(val),
                    Err(e) => Err(from_int_error_kind(e.kind())),
                }
            }
        }
    }
}

macro_impl!(U256, 256, u256);