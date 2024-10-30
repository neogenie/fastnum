//! Fast Numbers library
//!
//!
//!
//!
//!

mod error;

#[macro_use]
mod macros;

pub mod parse;

pub use error::ParseError;
pub use bnum::types::*;
