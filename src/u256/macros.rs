#[macro_export]
/// A macro to construct [crate::U256] from literals.
///
/// # Examples:
/// ```
/// use fastnum::{u256, u256::U256};
/// use num_traits::{Signed, Zero};
///  const N: U256 = u256!(100);
///  let x = u256!(1);
///  assert!(u256!(0).is_zero());
///  assert_eq!(u256!(115792089237316195423570985008687907853269984665640564039457584007913129639935), U256::MAX);
/// ```
macro_rules! u256 {
    ($lit:expr) => {
        {
            const __UINT: $crate::u256::U256 = $crate::u256::parse_str(stringify!($lit));
            __UINT
        }
    };
}