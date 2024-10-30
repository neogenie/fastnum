use std::fmt::{self, Debug, Display, Formatter, LowerExp, UpperExp};

use crate::decimal::unsigned::{format, UnsignedDecimal};
use crate::{U128, U256, U512};

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal) => {
        impl Display for UnsignedDecimal<$UINT> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let digits = self.value.to_str_radix(10);
                let scale = self.scale;
                format::format(digits, scale, f)
            }
        }

        impl LowerExp for UnsignedDecimal<$UINT> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                let digits = self.value.to_str_radix(10);
                let scale = self.scale;
                format::format_exponential(digits, scale, f, "e")
            }
        }

        impl UpperExp for UnsignedDecimal<$UINT> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                let digits = self.value.to_str_radix(10);
                let scale = self.scale;
                format::format_exponential(digits, scale, f, "E")
            }
        }

        impl Debug for UnsignedDecimal<$UINT> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                if f.alternate() {
                    write!(f, "UD{}(\"{}e{:}\")", $bits, self.value, -self.scale)
                } else {
                    write!(
                        f,
                        "UD{}(scale={}, digits=[{:?}])",
                        $bits, self.scale, self.value
                    )
                }
            }
        }
    };
}

macro_impl!(U128, 128);
macro_impl!(U256, 256);
macro_impl!(U512, 512);
