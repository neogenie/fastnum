macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(T: [< dec $bits >], [< u $bits >], [<D $bits>], [<U $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(T: [< udec $bits >], [< u $bits >], [<UD $bits>], [<U $bits>]); }
    };
    (T: $dec: ident, $uint: ident, $D: ident, $U: ident) => {
        mod $dec {
            use fastnum::{decimal::*, *};
            use rstest::*;

            #[rstest(::trace)]
            #[case(0, $uint!(1), 0, Signal::empty())]
            #[case(-0, $uint!(1), 0, Signal::empty())]
            #[case(-1, $uint!(1), 1, Signal::empty())]
            #[case(1, $uint!(1), -1, Signal::empty())]
            #[case(-2, $uint!(1), 2, Signal::empty())]
            #[case(2, $uint!(1), -2, Signal::empty())]
            #[case(-3, $uint!(1), 3, Signal::empty())]
            #[case(3, $uint!(1), -3, Signal::empty())]
            // -----------
            #[case(1000, $uint!(1), -1000, Signal::empty())]
            #[case(32767, $uint!(1), -32767, Signal::empty())]
            #[case(32768, $uint!(1), -32768, Signal::empty())]
            #[case(32769, $uint!(10), -32768, signals![!CP, !ROUND])]
            #[case(32770, $uint!(100), -32768, signals![!CP, !ROUND])]
            #[case(32771, $uint!(1000), -32768, signals![!CP, !ROUND])]
            // -----------
            #[case(-1000, $uint!(1), 1000, Signal::empty())]
            #[case(-32765, $uint!(1), 32765, Signal::empty())]
            #[case(-32766, $uint!(1), 32766, Signal::empty())]
            #[case(-32767, $uint!(1), 32767, Signal::empty())]
            fn test_quantum(
                #[case] exp: i32,
                #[case] digits: $U,
                #[case] scale: i16,
                #[case] signals: Signal,
            ) {
                let d = $D::quantum(exp, Context::default());

                assert_eq!(d.digits(), digits);
                assert_eq!(d.fractional_digits_count(), scale);
                assert_eq!(d.op_signals(), signals);
            }

            #[rstest(::trace)]
            #[case(65536)]
            fn test_quantum_overflow(#[case] exp: i32) {
                let d = $D::quantum(
                    exp,
                    Context::default().with_signal_traps(SignalsTraps::empty()),
                );
                assert!(d.is_infinite());
                assert_eq!(d.op_signals(), signals![!OFW, !INEXACT, !ROUND]);
            }

            #[rstest(::trace)]
            #[case(-65536)]
            #[case(-32768)]
            fn test_quantum_underflow(#[case] exp: i32) {
                let d = $D::quantum(
                    exp,
                    Context::default().with_signal_traps(SignalsTraps::empty()),
                );
                assert!(d.is_zero());
                assert_eq!(d.op_signals(), signals![!UFW, !INEXACT, !ROUND, !SN]);
            }
        }
    };
}

pub(crate) use test_impl;
