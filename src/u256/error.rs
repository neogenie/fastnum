use std::fmt;
use std::fmt::Display;
use std::num::{IntErrorKind, ParseIntError};

/// Enum to store the various types of errors that can cause parsing [crate::decimal::Decimal] to fail.
///
/// # Example
///
/// ```
/// use fastnum::u256::U256;
///
/// # fn main() {
/// if let Err(e) = U256::from_str_radix("a12", 10) {
///     println!("Failed conversion to U256: {}", e);
/// }
/// # }
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParseError {
    /// Value being parsed is empty.
    ///
    /// This variant will be constructed when parsing an empty string.
    Empty,

    /// Contains an invalid digit in its context.
    ///
    /// Among other causes, this variant will be constructed when parsing a string that
    /// contains a non-ASCII char.
    ///
    /// This variant is also constructed when a `+` or `-` is misplaced within a string
    /// either on its own or in the middle of a number.
    InvalidDigit,

    /// Integer is too large to store in target integer type.
    PosOverflow,

    /// Integer is too small to store in target integer type.
    NegOverflow,

    /// Value was Zero
    ///
    /// This variant will be emitted when the parsing string has a value of zero, which
    /// would be illegal for non-zero types.
    Zero,

    Unknown,
}

impl ParseError {
    pub(crate) const fn description(&self) -> &str {
        use ParseError::*;
        match self {
            Empty => "attempt to parse integer from empty string",
            InvalidDigit => "attempt to parse integer from string containing invalid digit",
            PosOverflow => {
                "attempt to parse integer too large to be represented by the target type"
            }
            NegOverflow => {
                "attempt to parse integer too small to be represented by the target type"
            }
            Zero => {
                "attempt to parse the integer `0` which cannot be represented by the target type"
            }
            Unknown => panic!("unknown error occurred"),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> ParseError {
        from_int_error_kind(e.kind())
    }
}

impl From<bnum::errors::ParseIntError> for ParseError {
    fn from(e: bnum::errors::ParseIntError) -> Self {
        from_int_error_kind(e.kind())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        self.description()
    }
}

pub(crate) const fn from_int_error_kind(e: &IntErrorKind) -> ParseError {
    match e {
        IntErrorKind::Empty => ParseError::Empty,
        IntErrorKind::InvalidDigit => ParseError::InvalidDigit,
        IntErrorKind::PosOverflow => ParseError::PosOverflow,
        IntErrorKind::NegOverflow => ParseError::NegOverflow,
        IntErrorKind::Zero => ParseError::Zero,
        _ => ParseError::Unknown,
    }
}
