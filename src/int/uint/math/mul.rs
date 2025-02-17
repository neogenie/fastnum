use crate::int::{intrinsics::Intrinsics, UInt};

#[inline(always)]
pub const fn strict_mul10<const N: usize>(n: UInt<N>, power: u32) -> UInt<N> {
    debug_assert!(power <= Intrinsics::<N>::MAX_POWER_OF_TEN);
    n.strict_mul(Intrinsics::<N>::POWERS_OF_TEN.lookup(power))
}

#[inline(always)]
pub const fn overflowing_mul10<const N: usize>(n: UInt<N>, power: u32) -> (UInt<N>, bool) {
    n.overflowing_mul(Intrinsics::<N>::POWERS_OF_TEN.lookup(power))
}

//  10  * x = 5*x * 2 = 5*x << 1 = (4*x + x) * 2 = ((x << 2) + x) << 1
// 100  * x = 10 * 10 = 5 * 2 * 5 * 2 = 25 * 4 = (16x + 9x) << 2 = (16x + 8x +
// x) << 2 = ((x << 4) + (x << 3) + x) << 2 10*x = (x << 3) + (x << 1);
