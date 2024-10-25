#[macro_export]
/// A macro to construct [crate::decimal::Decimal] from literals in compile time.
///
/// # Examples:
/// ```
/// use fastnum::{decimal, decimal::Decimal};
/// use num_traits::{Signed, Zero};
/// 
/// const NUM: Decimal = decimal!(1.23456789);
/// assert!(NUM.is_positive());
/// 
/// let num = decimal!(0);
/// assert!(num.is_zero());
/// 
/// let num = decimal!(-0.1);
/// assert!(num.is_negative());
///
/// ```
macro_rules! decimal {
    ($lit:expr) => {{
        const __DECIMAL: $crate::decimal::Decimal = $crate::decimal::parse_str(stringify!($lit));
        __DECIMAL
    }};
}
