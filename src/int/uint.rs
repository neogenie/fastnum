pub(crate) mod convert;
pub(crate) mod intrinsics;
pub(crate) mod math;

/// Big unsigned integer generic type, of fixed size which must be known at
/// compile time.
pub type UInt<const N: usize> = bnum::BUint<N>;

use crate::int::doc::int_type_doc;

macro_rules! uint_types {
    ( $($bits: literal $u: ident; ) *)  => {
        $(
            #[doc = int_type_doc!($bits, "unsigned")]
            pub type $u = UInt::<{$bits / 64}>;
        )*
    };
}

uint_types!(
    128 U128;
    256 U256;
    512 U512;
    1024 U1024;
    2048 U2048;
    4096 U4096;
    8192 U8192;
);
