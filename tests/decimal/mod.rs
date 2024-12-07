#![allow(clippy::excessive_precision)]
#![allow(clippy::approx_constant)]
#![allow(clippy::zero_prefixed_literal)]

pub(crate) mod common;

mod math;
mod round;
mod smoke;
mod parse;
mod extras;

#[cfg(feature = "numtraits")]
pub(crate) mod numtraits;

mod d128;
mod d256;
mod d512;

mod ud128;
mod ud256;
mod ud512;
