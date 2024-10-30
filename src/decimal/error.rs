use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::num::{IntErrorKind, ParseIntError};

use crate::utils::err_prefix;

/// Enum to store the various types of errors that can cause parsing [crate::decimal::Decimal] to fail.
///
/// # Example
///
/// ```
/// use fastnum::decimal::Decimal;
/// use std::str::FromStr;
///
/// # fn main() {
/// if let Err(e) = Decimal::from_str("e12") {
///     println!("Failed conversion to Decimal: {e}");
/// }
/// # }
/// ```
#[derive(Copy, Clone, PartialEq)]
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
    InvalidLiteral,

    /// Integer is too large to store in target integer type.
    PosOverflow,

    /// Integer is too small to store in target integer type.
    NegOverflow,

    /// Exponent is too large to store in decimal type.
    ExponentOverflow,

    /// Value was Zero
    ///
    /// This variant will be emitted when the parsing string has a value of zero, which
    /// would be illegal for non-zero types.
    Zero,

    Signed,

    Infinite,

    NaN,

    /// Invalid radix.
    InvalidRadix,

    /// Unknown error
    Unknown,
}

impl ParseError {
    pub const fn description(&self) -> &str {
        use ParseError::*;
        match self {
            Empty => "cannot parse decimal from empty string",
            InvalidLiteral => "invalid literal found in string",
            PosOverflow => "number too large to fit in target type",
            NegOverflow => "number too small to fit in target type",
            Zero => "number would be zero for non-zero type",
            Signed => "number would be signed for unsigned type",
            Infinite => "number is infinite",
            NaN => "number is NaN",
            InvalidRadix => "radix for decimal must be 10",
            ExponentOverflow => "exponent is too large to fit in target type",
            Unknown => "unknown error",
        }
    }

    pub(crate) const fn from_int_error_kind(e: &IntErrorKind) -> ParseError {
        match e {
            IntErrorKind::Empty => ParseError::Empty,
            IntErrorKind::InvalidDigit => ParseError::InvalidLiteral,
            IntErrorKind::PosOverflow => ParseError::PosOverflow,
            IntErrorKind::NegOverflow => ParseError::NegOverflow,
            IntErrorKind::Zero => ParseError::Zero,
            _ => ParseError::Unknown,
        }
    }
}

impl Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", err_prefix!(), self.description())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

impl From<ParseIntError> for ParseError {
    #[inline]
    fn from(e: ParseIntError) -> ParseError {
        Self::from_int_error_kind(e.kind())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {
    #[inline]
    fn description(&self) -> &str {
        self.description()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct TryFromIntError;

impl TryFromIntError {
    pub const fn description(&self) -> &'static str {
        "out of range integral type conversion attempted"
    }
}

impl Display for TryFromIntError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Debug for TryFromIntError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromIntError {
    #[inline]
    fn description(&self) -> &str {
        self.description()
    }
}
