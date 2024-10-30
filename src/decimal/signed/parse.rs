macro_rules! macro_impl {
    ($UINT: ident, $bits: literal, $name: ident) => {
        pub mod $name {
            use std::str::from_utf8_unchecked;
            
            use crate::decimal::unsigned;
            use crate::decimal::ParseError;
            use crate::decimal::signed::{Decimal, Sign};
            use crate::$UINT;
            
            /// Creates and initializes a Decimal from string.
            #[inline]
            pub const fn parse_str(s: &str) -> Decimal<$UINT> {
                match from_str(s) {
                    Ok(n) => n,
                    Err(e) => {
                        panic!("{}", e.description())
                    },
                }
            }
            
            /// Creates and initializes a Decimal from string.
            pub const fn from_str(s: &str) -> Result<Decimal<$UINT>, ParseError> {
                if s.is_empty() {
                    return Err(ParseError::Empty);
                }
                
                let buf = s.as_bytes();
                
                let (sign, src) = match buf[0] {
                    b'+' => {
                        (Sign::Plus, unsafe { from_utf8_unchecked(buf.split_at(1).1) })
                        
                    }
                    b'-' => {
                        (Sign::Minus, unsafe { from_utf8_unchecked(buf.split_at(1).1) })
                    }
                    _ => {
                        (Sign::NoSign, s)
                    }
                };
                
                match unsigned::parse::$name::from_str(src) {
                    Ok(value) => Ok(Decimal::<$UINT>::new(value, sign)),
                    Err(e) => {
                        Err(match (e, sign) {
                            (ParseError::PosOverflow, Sign::Minus) => ParseError::NegOverflow,
                            _ => e,
                        })
                    }
                }
            }
            
            macro_rules! from_float_impl {
                ($n: ident, $f: ty, $module: ident) => {
                    pub(crate) const fn $n(n: $f) -> Result<Decimal<$UINT>, ParseError> {
                        use crate::decimal::math::$module::*;
                        use crate::decimal::unsigned::parse::$name;
                        
                        if is_nan(n) {
                            return Err(ParseError::NaN);
                        }
                        
                        let b = to_bits(n);
        
                        let sign = b & SIGN_MASK != 0;
                        
                        let (sign, n) = if sign {
                            (Sign::Minus, -n)
                        } else {
                            (Sign::NoSign, n)
                        };
                        
                        match $name::$n(n) {
                            Ok(value) => Ok(Decimal::new(value, sign)),
                            Err(e) => Err(e),
                        }
                    }
                }
            }
            
            from_float_impl!(from_f32, f32, f32);
            from_float_impl!(from_f64, f64, f64);
        }
    }
}

macro_impl!(U128, 128, d128);
macro_impl!(U256, 256, d256);
macro_impl!(U512, 512, d512);
