//! Fast Decimal arbitrary precision library
//!
//! [Decimal] allows storing real number to arbitrary precision; which
//! avoids common floating point errors (such as 0.1 + 0.2 ≠ 0.3) at the
//! cost of complexity.
//!
//! Internally, `Decimal` uses a unsigned 256-bit integer, paired with a signed 64-bit
//! integer which determines the position of the decimal point. Therefore,
//! the precision *is not* actually arbitrary, but limited to 2<sup>63</sup>
//! decimal places.
//!
//! Common numerical operations are overloaded, so we can treat them
//! the same way we treat other numbers.
//!
//! It is not recommended to convert a floating point number to a decimal
//! directly, as the floating point representation may be unexpected.
//!
//! # Example
//!
//! ```
//! use fastnum::decimal::Decimal;
//! use std::str::FromStr;
//!
//! let input = "0.8";
//! let dec = Decimal::from_str(&input).unwrap();
//! let float = f32::from_str(&input).unwrap();
//!
//! println!("Input ({}) with decimals: {} vs {})", input, dec, float);
//! ```

pub mod signed;
pub mod unsigned;

pub(crate) mod math;

mod error;
mod rounding;

#[macro_use]
mod macros;

pub use error::ParseError;
pub use rounding::RoundingMode;
