use crate::{int::UInt, utils::err_msg};

pub type Digit = u64;
pub type DoubleDigit = u128;
pub type ExpType = u32;

pub type Digits<const N: usize> = [Digit; N];

pub const POWER: u32 = 19;
pub const BITS: ExpType = Digit::BITS;
pub const BITS_MINUS_1: ExpType = BITS - 1;
pub const BIT_SHIFT: ExpType = BITS.trailing_zeros();

#[repr(transparent)]
pub struct PowersOf10<const N: usize>([[UInt<N>; POWER as usize + 1]; N]);

impl<const N: usize> PowersOf10<N> {
    #[inline]
    const fn new() -> Self {
        debug_assert!(N > 0);
        debug_assert!((UInt::<N>::MAX.ilog10()) < (POWER + 1) * (N as u32));

        let mut res = [[UInt::ZERO; POWER as usize + 1]; N];
        res[0][0] = UInt::ONE;

        let mut v;
        let mut j = 0;
        let mut i = 1;
        v = UInt::ONE;
        while v.le(&UInt::<N>::MAX.div(UInt::TEN)) {
            v = v.strict_mul(UInt::TEN);
            res[j][i] = v;
            i += 1;

            if i == POWER as usize + 1 {
                i = 0;
                j += 1;
            }
        }

        Self(res)
    }

    #[inline(always)]
    pub const fn lookup(&self, power: u32) -> UInt<N> {
        let j = (power / (POWER + 1)) as usize;

        if j >= N {
            panic!(err_msg!("power is too large"));
        }

        let i = (power % (POWER + 1)) as usize;
        self.0[j][i]
    }
}

pub struct Intrinsics<const N: usize>;

impl<const N: usize> Intrinsics<N> {
    pub const MAX_POWER_OF_TEN: u32 = (POWER + 1) * (N as u32);
    pub const POWERS_OF_TEN: PowersOf10<N> = PowersOf10::new();
}
