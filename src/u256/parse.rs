use crate::u256::error::from_int_error_kind;
use crate::u256::{ParseError, U256};

/// AAAA
#[inline]
pub const fn parse_str(s: &str) -> U256 {
    match from_str(s) {
        Ok(n) => n,
        Err(e) => panic!("{}", e.description()),
    }
}

const fn from_str(s: &str) -> Result<U256, ParseError> {
    if let Some(val) = const_str::strip_prefix!(s, "0x") {
        from_str_radix(val, 16)
    } else {
        from_str_radix(s, 10)
    }
}

const fn from_str_radix(s: &str, radix: u32) -> Result<U256, ParseError> {
    match U256::from_str_radix(s, radix) {
        Ok(val) => Ok(val),
        Err(e) => Err(from_int_error_kind(e.kind())),
    }
}
