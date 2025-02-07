//! # Decimal numbers

#[cfg(feature = "test-util")]
#[doc(hidden)]
pub mod extras;

#[cfg(not(feature = "test-util"))]
pub(crate) mod extras;

pub(crate) mod dec;
pub(crate) mod doc;
pub(crate) mod round;
pub(crate) mod udec;

mod context;
mod flags;
mod sign;
mod signals;

#[cfg(debug_assertions)]
mod assertions;

#[allow(dead_code)]
pub(crate) mod utils;

pub(crate) mod errors;

#[macro_use]
mod macros;

pub use context::{Context, RoundingMode, SignalsTraps};
pub use dec::Decimal;
pub use errors::{DecimalError, ParseError};
pub use flags::Flags;
pub use sign::Sign;
pub use signals::Signal;
pub use udec::UnsignedDecimal;

use crate::decimal::doc::decimal_type_doc;

macro_rules! decimal_types {
    ( $($bits: literal $u: ident $s: ident; ) *)  => {
        $(
            #[doc = decimal_type_doc!($bits, "unsigned")]
            pub type $u = UnsignedDecimal::<{$bits / 64}>;

            #[doc = decimal_type_doc!($bits, "signed")]
            pub type $s = Decimal::<{$bits / 64}>;
        )*
    };
}

decimal_types!(
    128 UD128 D128;
    256 UD256 D256;
    512 UD512 D512;
    1024 UD1024 D1024;
    2048 UD2048 D2048;
    4096 UD4096 D4096;
    8192 UD8192 D8192;
);
