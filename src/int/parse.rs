macro_rules! macro_impl {
    ($sign: ident, $Ty: ident) => {
        pub mod $sign {
            use core::str::from_utf8_unchecked;

            use crate::int::{error::from_int_error_kind, $Ty, ParseError};

            #[inline]
            pub const fn parse_str<const N: usize>(s: &str) -> $Ty<N> {
                match from_str(s) {
                    Ok(n) => n,
                    Err(e) => panic!("{}", e.description()),
                }
            }

            #[inline]
            const fn from_str<const N: usize>(s: &str) -> Result<$Ty<N>, ParseError> {
                let buf = s.as_bytes();
                if buf.len() > 1 && buf[0] == b'0' && buf[1] == b'x' {
                    #[allow(unsafe_code)]
                    let s = unsafe { from_utf8_unchecked(buf.split_at(2).1) };
                    from_str_radix(s, 16)
                } else {
                    from_str_radix(s, 10)
                }
            }

            #[inline]
            const fn from_str_radix<const N: usize>(
                s: &str,
                radix: u32,
            ) -> Result<$Ty<N>, ParseError> {
                match $Ty::<N>::from_str_radix(s, radix) {
                    Ok(val) => Ok(val),
                    Err(e) => Err(from_int_error_kind(e.kind())),
                }
            }
        }
    };
}

macro_impl!(unsigned, UInt);
macro_impl!(signed, Int);
