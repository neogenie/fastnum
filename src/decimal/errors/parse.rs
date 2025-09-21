use core::{
    fmt,
    fmt::{Debug, Display, Formatter},
    num::{IntErrorKind, ParseIntError},
};

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

use crate::utils::err_prefix;

/// Enum to store the various types of errors that can cause parsing decimal to
/// fail.
///
/// # Example
///
/// ```
/// use fastnum::decimal::Context;
/// use fastnum::UD256;
///
/// if let Err(e) = UD256::from_str("e12", Context::default()) {
///     println!("Failed conversion to Decimal: {e}");
/// }
/// ```
#[derive(Copy, Clone, PartialEq)]
pub enum ParseError {
    /// Value being parsed is empty.
    ///
    /// This variant will be constructed when parsing an empty string.
    Empty,

    /// Contains an invalid digit in its context.
    ///
    /// Among other causes, this variant will be constructed when parsing a
    /// string that contains a non-ASCII char.
    ///
    /// This variant is also constructed when a `+` or `-` is misplaced within a
    /// string either on its own or in the middle of a number.
    InvalidLiteral,

    /// The number is too large to store in target decimal type.
    PosOverflow,

    /// The number is too small to store in target decimal type.
    NegOverflow,

    /// Exponent is too large to store in decimal type.
    ExponentOverflow,

    /// Value was Signed
    ///
    /// This variant will be emitted when the parsing string has a sign literal,
    /// which would be illegal for unsigned types.
    Signed,

    /// Invalid radix.
    InvalidRadix,

    /// Unknown error
    Unknown,
}

impl ParseError {
    #[inline(always)]
    pub(crate) const fn description(&self) -> &str {
        use ParseError::*;
        match self {
            Empty => "cannot parse decimal from empty string",
            InvalidLiteral => "invalid literal found in string",
            PosOverflow => "number too large to fit in target type",
            NegOverflow => "number too small to fit in target type",
            Signed => "number would be signed for unsigned type",
            InvalidRadix => "radix for decimal must be 10",
            ExponentOverflow => "exponent is too large to fit in target type",
            Unknown => "unknown error",
        }
    }

    #[inline(always)]
    pub(crate) const fn from_int_error_kind(e: &IntErrorKind) -> ParseError {
        match e {
            IntErrorKind::Empty => ParseError::Empty,
            IntErrorKind::InvalidDigit => ParseError::InvalidLiteral,
            IntErrorKind::PosOverflow => ParseError::PosOverflow,
            IntErrorKind::NegOverflow => ParseError::NegOverflow,
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
    #[inline]
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

impl core::error::Error for ParseError {
    #[inline]
    fn description(&self) -> &str {
        self.description()
    }
}

#[allow(dead_code)]
#[inline]
pub(crate) fn pretty_error_msg(ty: &str, e: ParseError) -> String {
    use ParseError::*;
    let msg = match e {
        Empty => "cannot be constructed from an empty string",
        InvalidLiteral => "string contains invalid characters",
        PosOverflow => "overflow",
        NegOverflow => "negative overflow",
        Signed => "does not support negative values",
        InvalidRadix => "radix MUST be 10",
        ExponentOverflow => "exponent overflow",
        Unknown => "decimal unknown error",
    };

    format!("{} {ty} {msg}", err_prefix!())
}
