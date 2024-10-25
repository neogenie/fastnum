//! Fast Numbers library
//!
//! 
//! 
//! 
//! 

mod parse;
mod error;

pub use parse::parse_str;
pub use error::ParseError;

#[macro_use]
mod macros;

/// U256
pub type U256 = bnum::types::U256;