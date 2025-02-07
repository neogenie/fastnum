macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);

        // #[rstest(::trace)]
        // #[case($dec!(1),   $D::E)]
        // #[case($dec!(1.5), $dec!(4.481689070338064822602055460119275819005749868369667056772650082785936674466713772981053831382453391388616350651830195768962746477220408606961759644973694))]
        // #[case($dec!(2),   $dec!(7.389056098930650227230427460575007813180315570551847324087127822522573796079057763384312485079121794773753161265478865861071847256222554755523560777944330))]
        // #[case($dec!(2.5), $dec!(12.18249396070347343807017595116796618318276779006316131156039834183818512614331441006025552300629578874164976170442788368712917648129599496753477915843652))]
        // #[case($dec!(3),   $dec!(20.08553692318766774092852965458171789698790783855415014437893422969884587809197373120449716025301770215360761585194900287021042684967986039597191508837625))]
        // #[case($dec!(4),   $dec!(54.59815003314423907811026120286087840279073703861406872582659395855366209993586948167698056194473414278411544577108743407044337947263363895012944697812306))]
        // #[case($dec!(5),   $dec!(148.4131591025766034211155800405522796234876675938789890467528451109120648209585760796884094598990211412928082706663260540549216427294321775515519658609825))]
        // #[case($dec!(10),  $dec!(22026.46579480671651695790064528424436635351261855678107423542635522520281857079257519912096816452589545155550109245783662818616607875444034355001832255666))]
        // #[case($dec!(15),  $dec!(3269017.372472110639301855046091721315505738543820034206629562773242021332748879132969874112289258912407319374255608741017119159966673029004641938241846981))]
        // #[case($dec!(100), $dec!(2.688117141816135448412625551580013587361111877374192241519160861528028703490956491415887109721984571081167087919057606869990399296290782991524699985243168e+43))]
        // fn test_exp_512(#[case] d: $D, #[case] expected: $D) {
        //     let res = d.exp();
        //     assert_eq!(res, expected);
        //     assert!(res.is_op_inexact());
        //     assert!(res.is_op_rounded());
        // }
    };


    (COMMON:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
    };
    (COMMON:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(COMMON:: 128, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(1),   $D::E)]
        #[case($dec!(1.5), $dec!(4.4816890703380648226020554601192758190057498683696670567726500827859366744667))]
        #[case($dec!(2),   $dec!(7.3890560989306502272304274605750078131803155705518473240871278225225737960791))]
        #[case($dec!(2.5), $dec!(12.182493960703473438070175951167966183182767790063161311560398341838185126143))]
        #[case($dec!(3),   $dec!(20.085536923187667740928529654581717896987907838554150144378934229698845878092))]
        #[case($dec!(4),   $dec!(54.598150033144239078110261202860878402790737038614068725826593958553662099936))]
        #[case($dec!(5),   $dec!(148.41315910257660342111558004055227962348766759387898904675284511091206482096))]
        #[case($dec!(10),  $dec!(22026.465794806716516957900645284244366353512618556781074235426355225202818571))]
        #[case($dec!(15),  $dec!(3269017.3724721106393018550460917213155057385438200342066295627732420213327489))]
        #[case($dec!(100), $dec!(2.6881171418161354484126255515800135873611118773741922415191608615280287034909e+43))]
        fn test_exp_256(#[case] d: $D, #[case] expected: $D) {
            let res = d.exp();
            assert_eq!(res, expected);
            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };

    (COMMON:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(1),   $D::E)]
        #[case($dec!(1.5), $dec!(4.4816890703380648226020554601192758190))]
        #[case($dec!(2),   $dec!(7.3890560989306502272304274605750078132))]
        #[case($dec!(2.5), $dec!(12.1824939607034734380701759511679661832))]
        #[case($dec!(3),   $dec!(20.0855369231876677409285296545817178970))]
        #[case($dec!(4),   $dec!(54.598150033144239078110261202860878403))]
        #[case($dec!(5),   $dec!(148.413159102576603421115580040552279623))]
        #[case($dec!(10),  $dec!(22026.4657948067165169579006452842443663))]
        #[case($dec!(15),  $dec!(3269017.37247211063930185504609172131551))]
        #[case($dec!(100), $dec!(2.68811714181613544841262555158001358736e+43))]
        fn test_exp_128(#[case] d: $D, #[case] expected: $D) {
            let res = d.exp();
            assert_eq!(res, expected);
            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {

        #[rstest(::trace)]
        #[case($dec!(0), $D::ONE)]
        #[case($D::INFINITY, $D::INFINITY)]
        fn test_exp_special(#[case] d: $D, #[case] expected: $D) {
            let res = d.exp();
            assert_eq!(res, expected);
            assert_eq!(res.op_signals(), signals![]);
        }

        #[rstest(::trace)]
        #[case($dec!(1), $D::E)]
        fn test_exp(#[case] d: $D, #[case] expected: $D) {
            let res = d.exp();
            assert_eq!(res, expected);
            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }

        #[rstest(::trace)]
        #[case($D::NAN)]
        fn test_exp_nan(#[case] d: $D) {
            let ctx = Context::default().without_traps();
            let res = d.with_ctx(ctx).exp();
            assert!(res.is_nan());
            assert!(res.is_op_invalid());
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {

    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(-1),   $dec!(0.36787944117144232159552377016146086744))]
        // #[case($dec!(-1.5), $dec!(4.4816890703380648226020554601192758187))]
        // #[case($dec!(-2),   $dec!(7.3890560989306502272304274605750078133))]
        // #[case($dec!(-2.5), $dec!(12.1824939607034734380701759511679661832))]
        // #[case($dec!(-3),   $dec!(20.0855369231876677409285296545817178969))]
        // #[case($dec!(-4),   $dec!(54.598150033144239078110261202860878402))]
        // #[case($dec!(-5),   $dec!(148.413159102576603421115580040552279621))]
        // #[case($dec!(-10),  $dec!(22026.4657948067165169579006452842443660))]
        // #[case($dec!(-15),  $dec!(3269017.37247211063930185504609172131533))]
        // #[case($dec!(-100), $dec!(3.720076E-44))]
        fn test_exp_128_signed(#[case] d: $D, #[case] expected: $D) {
            let res = d.exp();
            assert_eq!(res, expected);
            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-0), $D::ONE)]
        #[case($D::NEG_INFINITY, $D::ZERO)]
        fn test_exp_special_signed(#[case] d: $D, #[case] expected: $D) {
            let res = d.exp();
            assert_eq!(res, expected);
            assert_eq!(res.op_signals(), signals![]);
        }
    };
}

pub(crate) use test_impl;
