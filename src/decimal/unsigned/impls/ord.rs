use std::cmp::Ordering;

use crate::decimal::unsigned::UnsignedDecimal;
use crate::{U128, U256, U512};

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal) => {
        impl PartialOrd for UnsignedDecimal<$UINT> {
            #[inline]
            fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
                Some(self.cmp(rhs))
            }
        }
    
        impl Ord for UnsignedDecimal<$UINT> {
            #[inline]
            fn cmp(&self, rhs: &Self) -> Ordering {
                let a = self.normalized();
                let b = rhs.normalized();
                
                if a.scale == b.scale {
                    return a.value.cmp(&b.value);
                }
                
                let a_exp = a.value.ilog10() as i64 - a.scale;
                let b_exp = b.value.ilog10() as i64 - b.scale;
                
                match a_exp.cmp(&b_exp) {
                    Ordering::Equal => {
                        if a.scale > b.scale {
                            let (mul, false) = $UINT::TEN.overflowing_pow((a.scale - b.scale) as u32) else {
                                return Ordering::Less;
                            };
                            
                            let (value, false) = b.value.overflowing_mul(mul) else {
                                return Ordering::Less;
                            };
                            
                            a.value.cmp(&value)
                        }
                        else {
                            let (mul, false) = $UINT::TEN.overflowing_pow((b.scale - a.scale) as u32) else {
                                return Ordering::Less;
                            };
                            
                            let (value, false) = a.value.overflowing_mul(mul) else {
                                return Ordering::Less;
                            };
                            
                            value.cmp(&b.value)
                        }
                    },
                    Ordering::Less => {
                        Ordering::Less
                    },
                    Ordering::Greater => {
                        Ordering::Greater
                    }
                }
            }
        }
    };
}

macro_impl!(U128, 128);
macro_impl!(U256, 256);
macro_impl!(U512, 512);

