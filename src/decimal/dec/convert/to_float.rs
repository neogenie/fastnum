use crate::{
    decimal::Decimal,
    int::{convert::*, math::div_rem, UInt},
    utils::err_msg,
};

type D<const N: usize> = Decimal<N>;
type U<const N: usize> = UInt<N>;

// @TODO: Performance optimizations
// @TODO: Implement fast Daniel Lemire algorithm: https://arxiv.org/abs/2101.11408.pdf.

#[inline(always)]
const fn mul_log_2_10(n: i32) -> i32 {
    if n == 0 {
        0
    } else {
        (n << 12) / 1233 + 1
    }
}

macro_rules! to_float_impl {
    ($to_f: ident, $to_f_impl: ident, $to_uint: ident, $u: ident, $f: ident) => {
        #[inline]
        pub const fn $to_f<const N: usize>(d: D<N>) -> $f {
            if d.is_nan() {
                return $f::NAN;
            }

            let sign = d.is_negative();
            let res = $to_f_impl(d.abs());

            if sign {
                -res
            } else {
                res
            }
        }

        #[inline]
        const fn $to_f_impl<const N: usize>(d: D<N>) -> $f {
            use crate::decimal::utils::types::$f::*;

            if d.is_infinite() {
                return $f::INFINITY;
            }

            if d.is_zero() {
                return 0.0;
            }

            if d.gt(&FloatConsts::<N>::MAX) {
                return $f::INFINITY;
            }

            let bits = d.digits.bits() as i32;

            let d_exp = d.cb.get_exponent();

            let b_exp_native = if d_exp >= 0 {
                let exp_correction = mul_log_2_10(d_exp);
                bits + exp_correction
            } else {
                let exp_correction = mul_log_2_10(-d_exp);
                bits - exp_correction
            };

            let mut b_exp = b_exp_native - (MANTISSA_DIGITS - 1) as i32;

            let mut coefficient: UInt<19> = d.transmute().digits;
            let rem;
            let mut exp_shift = false;

            // TODO: 2^k multiplication/div replace by shift
            if d_exp >= 0 {
                coefficient = coefficient.mul(UInt::FIVE.pow(d_exp as u32));
                if b_exp >= d_exp {
                    (coefficient, rem) =
                        div_rem(coefficient, UInt::TWO.pow((b_exp - d_exp) as u32));
                    if !rem.is_zero() {
                        coefficient = coefficient.mul(UInt::TWO).add(UInt::ONE);
                        exp_shift = true;
                    }
                } else {
                    coefficient = coefficient.mul(UInt::TWO.pow(-(b_exp - d_exp) as u32));
                }
            } else {
                if b_exp >= d_exp {
                    (coefficient, rem) =
                        div_rem(coefficient, UInt::TWO.pow((b_exp - d_exp) as u32));
                    if !rem.is_zero() {
                        coefficient = coefficient.mul(UInt::TWO).add(UInt::ONE);
                        exp_shift = true;
                    }
                } else {
                    coefficient = coefficient.mul(UInt::TWO.pow(-(b_exp - d_exp) as u32));
                }

                coefficient = coefficient.div(UInt::FIVE.pow(-d_exp as u32));
            };

            if exp_shift && !coefficient.bit(0) {
                coefficient = coefficient.shr(1);
                exp_shift = false;
            }

            let digits = coefficient;
            let bits = digits.bits();

            let mut mant = if U::<N>::BITS > $u::BITS {
                if bits < MANTISSA_DIGITS {
                    b_exp -= 1;
                    if let Ok(m) = $to_uint(digits) {
                        m << (MANTISSA_DIGITS - bits)
                    } else {
                        panic!(err_msg!("mantissa is too large"));
                    }
                } else {
                    if let Ok(m) = $to_uint(digits.shr(bits - MANTISSA_DIGITS)) {
                        m
                    } else {
                        panic!(err_msg!("mantissa is too large"));
                    }
                }
            } else if bits < MANTISSA_DIGITS {
                if let Ok(m) = $to_uint(digits) {
                    m << (MANTISSA_DIGITS - bits)
                } else {
                    panic!(err_msg!("mantissa is too large"));
                }
            } else {
                if let Ok(m) = $to_uint(digits) {
                    m >> (bits - MANTISSA_DIGITS)
                } else {
                    panic!(err_msg!("mantissa is too large"));
                }
            };

            if exp_shift {
                b_exp -= 1;
            }

            let mut exp = b_exp + (MANTISSA_DIGITS - 1) as i32;

            if exp > MAX_EXP - 1 {
                return $f::INFINITY;
            }

            if exp < 1 - MAX_EXP - (MANTISSA_DIGITS - 1) as i32 {
                return 0.0;
            }

            if exp < 1 - MAX_EXP {
                let mant_shift = 2 - MAX_EXP - exp;
                mant >>= mant_shift;
                exp += mant_shift - 1;
            }

            let exp = (exp + (MAX_EXP - 1)) as $u;
            $f::from_bits((exp << (MANTISSA_DIGITS - 1)) | mant & MAN_MASK)
        }
    };
}

to_float_impl!(to_f32, to_f32_impl, to_u32, u32, f32);
to_float_impl!(to_f64, to_f64_impl, to_u64, u64, f64);
