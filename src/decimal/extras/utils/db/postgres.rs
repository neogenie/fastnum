macro_rules! macro_impl {
    ($UINT: ident, $bits: literal, $module: ident) => {
        pub(crate) mod $module {
            
            use crate::$UINT;
            
            use std::cmp::Ordering;
            use num_integer::Integer;
            use num_traits::ToPrimitive;
            
            use crate::decimal::ParseError;
            use crate::decimal::unsigned::UnsignedDecimal;
            
            macro_rules! checked {
                ($a: ident *= $b: expr) => {
                    $a = $a.checked_mul($b).ok_or(ParseError::PosOverflow)?;
                };
                ($a: ident += $b: expr) => {
                    $a = $a.checked_add($b).ok_or(ParseError::PosOverflow)?;
                };
                ($a: ident /= $b: expr) => {
                    $a = $a.checked_div($b).ok_or(ParseError::PosOverflow)?;
                };
            }
            
            pub fn from_nbase(weight: i16, scale: u16, digits: &Vec<i16>) -> Result<UnsignedDecimal<$UINT>, ParseError> {
                let count = i64::try_from(digits.len()).map_err(|_| ParseError::PosOverflow)?;
                let weight = i64::from(weight);
                let scale = i64::from(scale);
    
                const TEN: $UINT = $UINT::TEN;
                const NBASE: $UINT = $UINT::from_digit(10_000);
    
                let mut uint = $UINT::ZERO;
    
                for digit in digits {
                    let d = u64::try_from(*digit).map_err(|_| ParseError::InvalidLiteral)?;
                    checked!(uint *= NBASE);
                    checked!(uint += $UINT::from_digit(d));
                }
    
                let correction_exp = - (4 * (weight - (count - 1)));
    
                match scale.cmp(&correction_exp) {
                    Ordering::Greater => {
                        let scale_diff = u32::try_from(scale - correction_exp).map_err(|_| ParseError::PosOverflow)?;
                        let correction = TEN.checked_pow(scale_diff).ok_or(ParseError::PosOverflow)?;
                        checked!(uint *= correction);
                    },
                    Ordering::Less => {
                        let scale_diff = u32::try_from(correction_exp - scale).map_err(|_| ParseError::PosOverflow)?;
                        let correction = TEN.checked_pow(scale_diff).ok_or(ParseError::PosOverflow)?;
                        checked!(uint /= correction);
                    },
                    Ordering::Equal => {},
                }
    
                Ok(UnsignedDecimal::new(uint, scale))
            }
            
            pub fn to_nbase(decimal: &UnsignedDecimal<$UINT>) -> Result<(i16, u16, Vec<i16>), ParseError> {
                
                const TEN: $UINT = $UINT::TEN;
                const NBASE: $UINT = $UINT::from_digit(10_000);
                
                let mut uint = decimal.significant_digits();
                
                if uint.is_zero() {
                    return Ok((0, 0, vec![]));
                }
                
                let mut scale = decimal.fractional_digit_count();
                
                while scale <= -1 {
                   checked!(uint *= TEN);
                   scale += 1;
                }
                
                let scale = scale.try_into().map_err(|_| ParseError::ExponentOverflow)?;
                
                // Ensure that the decimal will always lie on a digit boundary
                for _ in 0..(4 - scale % 4) {
                    checked!(uint *= TEN);
                }
                
                let mut digits = Vec::with_capacity(0);

                let mut weight = 0;
                
                while !uint.is_zero() {
                    let (div, rem) = uint.div_rem(&NBASE);
                    
                    if !digits.is_empty() || !rem.is_zero() {
                        digits.push(rem.to_i16().expect("10000 always fits in an i16"));
                    }
                    
                    uint = div;
                    weight += 1;
                }

                digits.reverse();
                
                let weight = weight - (scale as i16 / 4 + 1) - 1;
                
                Ok((weight, scale, digits))
            }
        }
    }
}

macro_impl!(U128, 128, u128);
macro_impl!(U256, 256, u256);
macro_impl!(U512, 512, u512);