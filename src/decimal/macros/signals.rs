/// Macro helper for fast initializing of signals set.
///
/// # Examples
///
/// ```
/// use fastnum::{*, decimal::*};
///
/// let signals = signals![!OFW, !CP];
///
/// assert_eq!(signals, Signals::OP_OVERFLOW.combine(Signals::OP_CLAMPED));
/// ```
#[macro_export]
macro_rules! signals {
    [ $(! $tts: tt),* ] => {{
        const __SIGNALS: $crate::decimal::Signals = signals!(@ [$($tts),*]);
        __SIGNALS
    }};
    (@ []) => {
        $crate::decimal::Signals::EMPTY
    };
    (@ CP) => {
        $crate::decimal::Signals::OP_CLAMPED
    };
    (@ DBZ) => {
        $crate::decimal::Signals::OP_DIV_BY_ZERO
    };
    (@ INEXACT) => {
        $crate::decimal::Signals::OP_INEXACT
    };
    (@ INV) => {
        $crate::decimal::Signals::OP_INVALID
    };
    (@ OFW) => {
        $crate::decimal::Signals::OP_OVERFLOW
    };
    (@ ROUND) => {
        $crate::decimal::Signals::OP_ROUNDED
    };
    (@ SN) => {
        $crate::decimal::Signals::OP_SUBNORMAL
    };
    (@ UFW) => {
        $crate::decimal::Signals::OP_UNDERFLOW
    };
    (@ [$t:tt]) => {
        signals!(@ $t)
    };
    (@ [$t:tt, $($tts:tt),*]) => {
        signals!(@ $t).combine(signals!(@ [$($tts),*]))
    };
}
