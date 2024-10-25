use std::str::FromStr;

use crate::decimal::{parse::from_str, Decimal, ParseError};

impl FromStr for Decimal {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Decimal, ParseError> {
        from_str(s)
    }
}
