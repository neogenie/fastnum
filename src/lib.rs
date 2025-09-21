#![doc = include_str!("../doc/LIB.md")]
#![deny(unsafe_code, missing_docs, clippy::all, clippy::cargo)]
#![cfg_attr(not(feature = "std"), no_std)]
// #![cfg_attr(nightly, feature(generic_const_exprs))]

extern crate alloc;
extern crate core;

pub mod bint;
pub mod decimal;

mod cast;
mod doc;
mod utils;

pub use cast::{Cast, TryCast};

// pub use bint::{I1024, I128, I2048, I256, I4096, I512, I64, I8192};
pub use bint::{I1024, I128, I256, I512, I64};

// pub use bint::{U1024, U128, U2048, U256, U4096, U512, U64, U8192};
pub use bint::{U1024, U128, U256, U512, U64};

// pub use decimal::{UD1024, UD128, UD2048, UD256, UD4096, UD512, UD64, UD8192};
pub use decimal::{UD1024, UD128, UD256, UD512, UD64};

// pub use decimal::{D1024, D128, D2048, D256, D4096, D512, D64, D8192};
pub use decimal::{D1024, D128, D256, D512, D64};
