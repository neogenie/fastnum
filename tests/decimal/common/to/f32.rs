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
            use fastnum::*;
            use num_traits::float::FloatCore;

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::*;
            use num_traits::float::FloatCore;

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);

        #[rstest(::trace)]
        #[case(1e-13,                   $dec!(9.99999982451670044181213370393379591405391693115234375e-14))]
        #[case(7.2e-14,                 $dec!(7.20000028022783900016889901962713338434696197509765625e-14))]
        #[case(1e-16,                   $dec!(1.0000000168623835263871646450439811815158464014530181884765625e-16))]
        #[case(1e-32,                   $dec!(1.00000002374222799036108273658815415520405083747275818881715403474430559072061441838741302490234375e-32))]
        #[case(1.1754943508e-38,        $dec!(1.1754943508222875079687365372222456778186655567720875215087517062784172594547271728515625e-38))]
        #[case(1.1754947011469036e-38,  $dec!(1.175494771211826805413857814341120664793504940850670084463483233398902426392051978609742945991456508636474609375e-38))]
        #[case(317e-40,                 $dec!(3.1700000098946435501119816090716154772221806896649747100732700841687651538425285480116144753992557525634765625E-38))]
        #[case(2.35098744048e-38,       $dec!(2.350987440475957123602109243087866394712812961308427354153308831195379018097479928428583662025630474090576171875e-38))]
        #[case(2.3509889819e-38,        $dec!(2.35098898190426788090088725919040801362055736959656341832065776397049129686767088287524529732763767242431640625e-38))]
        fn test_to_f32_512(#[case] expected: f32, #[case] d: $D) {
            let f = f32::try_from(d).unwrap();
            assert_eq!(f.integer_decode(), expected.integer_decode());
        }

        #[rstest(::trace)]
        #[case(1.0e-39,             $dec!(1.00000021530533325742087560014568310926874564800968669110436609702256827159061458587530069053173065185546875000e-39))]
        #[case(3.92e-39,            $dec!(3.91999933059456489828739575494312783522406115751507460249208160269472102366083987590172910131514072418212890625e-39))]
        #[case(1.0e-40,             $dec!(0.9999946101114759581525919052273499496042205269619191850412790687494327124262838424328947439789772033691406250e-40))]
        #[case(1e-42,               $dec!(1.0005271035279193886395429224690001177341070264998322610345467546973108330377044694614596664905548095703125e-42))]
        #[case(1.40129846432e-45,   $dec!(1.40129846432481707092372958328991613128026194187651577175706828388979108268586060148663818836212158203125e-45))]
        fn test_to_f32_subnormal_512(#[case] expected: f32, #[case] d: $D) {
            let f = f32::try_from(d).unwrap();
            assert_eq!(f.integer_decode(), expected.integer_decode());
        }
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);

        #[rstest(::trace)]
        #[case(-1.0e-40, $dec!(-0.9999946101114759581525919052273499496042205269619191850412790687494327124262838424328947439789772033691406250e-40))]
        #[case(-1.0e-39, $dec!(-1.00000021530533325742087560014568310926874564800968669110436609702256827159061458587530069053173065185546875000e-39))]
        #[case(-3.92e-39, $dec!(-3.91999933059456489828739575494312783522406115751507460249208160269472102366083987590172910131514072418212890625e-39))]
        #[case(-1e-42, $dec!(-1.0005271035279193886395429224690001177341070264998322610345467546973108330377044694614596664905548095703125e-42))]
        #[case(-1.40129846432e-45, $dec!(-1.40129846432481707092372958328991613128026194187651577175706828388979108268586060148663818836212158203125e-45))]
        fn test_to_f32_subnormal_signed_512(#[case] expected: f32, #[case] d: $D) {
            let f = f32::try_from(d).unwrap();
            assert_eq!(f.integer_decode(), expected.integer_decode());
        }
    };


    (COMMON:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);

        #[rstest(::trace)]
        #[case(1e-13,                   $dec!(9.99999982451670044181213370393379591405391693115234375e-14))]
        #[case(7.2e-14,                 $dec!(7.20000028022783900016889901962713338434696197509765625e-14))]
        #[case(1e-16,                   $dec!(1.0000000168623835263871646450439811815158464014530181884765625e-16))]
        #[case(1e-32,                   $dec!(1.00000002374222799036108273658815415520405083747275818881715403474430559072062e-32))]
        #[case(1.1754943508e-38,        $dec!(1.1754943508222875079687365372222456778186655567720875215087517062784172594548e-38))]
        #[case(1.1754947011469036e-38,  $dec!(1.1754947712118268054138578143411206647935049408506700844634832333989024263921e-38))]
        #[case(2.3509874e-38,           $dec!(2.3509874404759571236021092430878663947128129613084273541533088311953790182e-38))]
        #[case(2.3509889819e-38,        $dec!(2.3509889819042678809008872591904080136205573695965634183206577639704912968677e-38))]
        #[case(317e-40,                 $dec!(3.170000009894643550111981609071615477222180689664974710073270084168765153843e-38))]
        fn test_to_f32_256(#[case] expected: f32, #[case] d: $D) {
            let f = f32::try_from(d).unwrap();
            assert_eq!(f.integer_decode(), expected.integer_decode());
        }

        #[rstest(::trace)]
        #[case(1.0e-39,             $dec!(1.00000021530533325742087560014568310926874564800968669110436609702256827159062e-39))]
        #[case(3.92e-39,            $dec!(3.9199993305945648982873957549431278352240611575150746024920816026947210236608e-39))]
        #[case(9.9999e-41,          $dec!(0.99999461011147595815259190522734994960422052696191918504127906874943271242628e-40))]
        #[case(1e-42,               $dec!(1.00052710352791938863954292246900011773410702649983226103454675469731083303771e-42))]
        #[case(1.40129846432e-45,   $dec!(1.4012984643248170709237295832899161312802619418765157717570682838897910826859e-45))]
        fn test_to_f32_subnormal_256(#[case] expected: f32, #[case] d: $D) {
            let f = f32::try_from(d).unwrap();
            assert_eq!(f.integer_decode(), expected.integer_decode());
        }
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
        #[case(-9.9999e-41, $dec!(-0.99999461011147595815259190522734994960422052696191918504127906874943271242628e-40))]
        #[case(-1.0e-39, $dec!(-1.00000021530533325742087560014568310926874564800968669110436609702256827159062e-39))]
        #[case(-3.92e-39, $dec!(-3.9199993305945648982873957549431278352240611575150746024920816026947210236608e-39))]
        #[case(-1e-42, $dec!(-1.00052710352791938863954292246900011773410702649983226103454675469731083303771e-42))]
        #[case(-1.40129846432e-45, $dec!(-1.4012984643248170709237295832899161312802619418765157717570682838897910826859e-45))]
        fn test_to_f32_subnormal_signed_256(#[case] expected: f32, #[case] d: $D) {
            let f = f32::try_from(d).unwrap();
            assert_eq!(f.integer_decode(), expected.integer_decode());
        }
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };

    (COMMON:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case(1e-13,                   $dec!(0.9999999999999999999999999999999999999e-13))]
        #[case(7.2e-14,                 $dec!(7.200000290000000000000000000000000000e-14))]
        #[case(1e-16,                   $dec!(1.0000000168623835263871646450439811816e-16))]
        #[case(1e-32,                   $dec!(1.0000000237422279903610827365881542e-32))]
        #[case(1.1754943508e-38,        $dec!(1.17549435082228750796873653722224567782e-38))]
        #[case(1.1754947011469036e-38,  $dec!(1.17549477121182680541385781434112066480e-38))]
        #[case(2.3509889819e-38,        $dec!(2.35098898190426788090088725919040801363e-38))]
        #[case(317e-40,                 $dec!(3.17000000989464355011198160907161547723e-38))]
        fn test_to_f32_128(#[case] expected: f32, #[case] d: $D) {
            let f = f32::try_from(d).unwrap();
            assert_eq!(f.integer_decode(), expected.integer_decode());
        }

        #[rstest(::trace)]
        #[case(1.1754942e-38,       $dec!(1.1754942e-38))]
        #[case(1.0e-39,             $dec!(1.00000021530533325742087560014568310927e-39))]
        #[case(3.92e-39,            $dec!(3.92e-39))]
        #[case(1.0e-40,             $dec!(1.0e-40))]
        #[case(1e-42,               $dec!(1.0006e-42))]
        #[case(2e-45,               $dec!(2e-45))]
        fn test_to_f32_subnormal_128(#[case] expected: f32, #[case] d: $D) {
            let f = f32::try_from(d).unwrap();
            assert_eq!(f.integer_decode(), expected.integer_decode());
        }
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        $crate::decimal::common::to::float::to_float!(TO f32, $dec, $D);
        $crate::decimal::common::to::float::to_float!(TO INF f32, $dec, $D);
        $crate::decimal::common::to::float::to_float!(TO NAN f32, $dec, $D);

        #[rstest(::trace)]
        #[case(f32::MIN_POSITIVE,       $dec!(1.17549435082228750796873653722224567782e-38))]
        #[case(2.3509876e-38,           $dec!(2.35098758e-38))]
        #[case(2.3283064e-10,           $dec!(0.00000000023283064365386962890625))]
        #[case(1e-5,                    $dec!(0.00000999999974737875163555145263671875))]
        #[case(0.00013746770127909258,  $dec!(0.0001374676940031349658966064453125))]
        #[case(0.00036393293703440577,  $dec!(0.00036393295158632099628448486328125))]
        #[case(0.0008602388261351734,   $dec!(0.000860238797031342983245849609375))]
        #[case(0.001,                   $dec!(0.001000000047497451305389404296875))]
        #[case(0.0015924838953651488, $dec!(0.001592483953572809696197509765625))]
        #[case(0.002153817447833717, $dec!(0.0021538175642490386962890625))]
        #[case(0.004221370676532388, $dec!(0.0042213709093630313873291015625))]
        #[case(0.012114629615098238, $dec!(0.012114630080759525299072265625))]
        #[case(0.028068351559340954, $dec!(0.0280683524906635284423828125))]
        #[case(0.03706067614257336, $dec!(0.037060678005218505859375))]
        #[case(0.09289376810193062, $dec!(0.092893771827220916748046875))]
        #[case(0.01, $dec!(0.00999999977648258209228515625))]
        #[case(0.07155292, $dec!(0.07155291736125946044921875))]
        #[case(1e-1, $dec!(0.100000001490116119384765625))]
        #[case(0.14693861798803098, $dec!(0.146938621997833251953125))]
        #[case(0.1760912590558, $dec!(0.176091253757476806640625))]
        #[case(0.176091259055681, $dec!(0.176091253757476806640625))]
        #[case(2e-1, $dec!(0.20000000298023223876953125))]
        #[case(0.21791061013936996, $dec!(0.21791060268878936767578125))]
        #[case(0.289529654602168, $dec!(0.2895296514034271240234375))]
        #[case(0.301029995663981, $dec!(0.3010300099849700927734375))]
        #[case(0.30103, $dec!(0.3010300099849700927734375))]
        #[case(0.30531780421733856, $dec!(0.3053177893161773681640625))]
        #[case(0.3333333, $dec!(0.333333313465118408203125))]
        #[case(0.333333333333333333333333333333, $dec!(0.3333333432674407958984375))]
        #[case(0.4999999, $dec!(0.4999999105930328369140625))]
        #[case(0.5000001, $dec!(0.50000011920928955078125))]
        #[case(0.7622503340244293, $dec!(0.76225030422210693359375))]
        #[case(1.0 / 3.0, $dec!(0.3333333432674407958984375))]
        #[case(1.000006, $dec!(1.0000059604644775390625))]
        #[case(1.01, $dec!(1.0099999904632568359375))]
        #[case(1.14, $dec!(1.13999998569488525390625))]
        #[case(1.1877630352973938, $dec!(1.18776309490203857421875))]
        #[case(1.30113, $dec!(1.3011300563812255859375))]
        #[case(1.38, $dec!(1.37999999523162841796875))]
        #[case(1.4, $dec!(1.39999997615814208984375))]
        #[case(1.7, $dec!(1.7000000476837158203125))]
        #[case(1.8, $dec!(1.7999999523162841796875))]
        #[case(2.09808, $dec!(2.0980799198150634765625))]
        #[case(2.4, $dec!(2.400000095367431640625))]
        #[case(2.687217116355896, $dec!(2.687217235565185546875))]
        #[case(2.718281828459045, $dec!(2.71828174591064453125))]
        #[case(3.0000000000000004, $dec!(3.0))]
        #[case(3.11, $dec!(3.1099998950958251953125))]
        #[case(3.14159265359, $dec!(3.1415927410125732421875))]
        #[case(3.141592653589793, $dec!(3.1415927410125732421875))]
        #[case(3.1416, $dec!(3.1415998935699462890625))]
        #[case(3.2, $dec!(3.2000000476837158203125))]
        #[case(3.8, $dec!(3.7999999523162841796875))]
        #[case(3.9, $dec!(3.900000095367431640625))]
        #[case(4.12, $dec!(4.11999988555908203125))]
        #[case(6e0, $dec!(6))]
        #[case(6.99999952316, $dec!(6.999999523162841796875))]
        #[case(7.5464513301849365, $dec!(7.546451091766357421875))]
        #[case(1e0000001, $dec!(10))]
        #[case(12.34, $dec!(12.340000152587890625))]
        #[case(13.91745138168335, $dec!(13.9174518585205078125))]
        #[case(14.03, $dec!(14.02999973297119140625))]
        #[case(17.486443519592285, $dec!(17.48644256591796875))]
        #[case(18.04, $dec!(18.04000091552734375))]
        #[case(20.04, $dec!(20.04000091552734375))]
        #[case(24.86, $dec!(24.8600006103515625))]
        #[case(50.61, $dec!(50.6100006103515625))]
        #[case(50.811574935913086, $dec!(50.81157684326171875))]
        #[case(68.225, $dec!(68.22499847412109375))]
        #[case(86.53, $dec!(86.529998779296875))]
        #[case(124.16878890991211, $dec!(124.168792724609375))]
        #[case(206.50310516357422, $dec!(206.50311279296875))]
        #[case(294.33, $dec!(294.329986572265625))]
        #[case(411.88682556152344, $dec!(411.8868408203125))]
        #[case(521.66, $dec!(521.65997314453125))]
        #[case(936.3702087402344, $dec!(936.3702392578125))]
        #[case(1061.86, $dec!(1061.8599853515625))]
        #[case(1370.9265747070312, $dec!(1370.926513671875))]
        #[case(2018.04, $dec!(2018.0400390625))]
        #[case(2020.04, $dec!(2020.0400390625))]
        #[case(2525.2840576171875, $dec!(2525.2841796875))]
        #[case(6318.580322265625, $dec!(6318.580078125))]
        #[case(15498.36376953125, $dec!(15498.36328125))]
        #[case(16407.9462890625, $dec!(16407.9453125))]
        #[case(17419.6494140625, $dec!(17419.6484375))]
        #[case(21509.2, $dec!(21509.19921875))]
        #[case(30219.0830078125, $dec!(30219.08203125))]
        #[case(85003.24609375, $dec!(85003.25))]
        #[case(156782.0703125, $dec!(156782.0625))]
        #[case(328381.484375, $dec!(328381.5))]
        #[case(782262.28125, $dec!(782262.25))]
        #[case(1510988.3125, $dec!(1510988.25))]
        // ----------------
        #[case(31184683.0, $dec!(31184684))]
        #[case(31904423.0, $dec!(31904424))]
        #[case(32721165.0, $dec!(32721164))]
        #[case(33228241.0, $dec!(33228240))]
        #[case(33333333.0, $dec!(33333332))]
        #[case(34404238.0, $dec!(34404240))]
        #[case(37188395.0, $dec!(37188396))]
        #[case(37826145.0, $dec!(37826144))]
        #[case(38544571.0, $dec!(38544572))]
        #[case(38562139.0, $dec!(38562140))]
        #[case(38598533.0, $dec!(38598532))]
        #[case(38803710.0, $dec!(38803712))]
        #[case(38832042.0, $dec!(38832040))]
        #[case(39735126.0, $dec!(39735128))]
        #[case(39740229.0, $dec!(39740228))]
        #[case(39988247.0, $dec!(39988248))]
        #[case(40405643.0, $dec!(40405644))]
        #[case(42078409.0, $dec!(42078408))]
        #[case(42092201.0, $dec!(42092200))]
        #[case(42423473.0, $dec!(42423472))]
        #[case(42667174.0, $dec!(42667176))]
        #[case(42669063.0, $dec!(42669064))]
        #[case(42717713.0, $dec!(42717712))]
        #[case(43288506.0, $dec!(43288504))]
        #[case(43340539.0, $dec!(43340540))]
        #[case(43527578.0, $dec!(43527576))]
        #[case(43641062.0, $dec!(43641064))]
        #[case(44295939.0, $dec!(44295940))]
        #[case(44421934.0, $dec!(44421936))]
        #[case(45035996.273704985, $dec!(45035996))]
        #[case(45035996.273704995, $dec!(45035996))]
        #[case(45877585.0, $dec!(45877584))]
        #[case(46116538.0, $dec!(46116536))]
        #[case(47939822.0, $dec!(47939824))]
        #[case(48094147.0, $dec!(48094148))]
        #[case(48111209.0, $dec!(48111208))]
        #[case(48113823.0, $dec!(48113824))]
        #[case(48250926.0, $dec!(48250928))]
        #[case(48361110.0, $dec!(48361112))]
        #[case(48633003.0, $dec!(48633004))]
        #[case(48828125.0, $dec!(48828124))]
        #[case(49194195.0, $dec!(49194196))]
        #[case(53304714.0, $dec!(53304712))]
        #[case(54467535.0, $dec!(54467536))]
        #[case(54571890.0, $dec!(54571888))]
        #[case(54656606.0, $dec!(54656608))]
        #[case(54774735.0, $dec!(54774736))]
        #[case(55078143.0, $dec!(55078144))]
        #[case(55555555.0, $dec!(55555556))]
        #[case(55680090.0, $dec!(55680088))]
        #[case(55990879.0, $dec!(55990880))]
        #[case(56550310.0, $dec!(56550312))]
        #[case(57245637.0, $dec!(57245636))]
        #[case(57304222.0, $dec!(57304224))]
        #[case(57706234.0, $dec!(57706232))]
        #[case(57773302.0, $dec!(57773304))]
        #[case(58169007.0, $dec!(58169008))]
        #[case(58188878.0, $dec!(58188880))]
        #[case(58416654.0, $dec!(58416656))]
        #[case(59062754.0, $dec!(59062752))]
        #[case(61832742.0, $dec!(61832744))]
        #[case(62397437.0, $dec!(62397436))]
        #[case(62864002.0, $dec!(62864000))]
        #[case(62944073.0, $dec!(62944072))]
        #[case(63168350.0, $dec!(63168352))]
        #[case(63649610.0, $dec!(63649608))]
        #[case(63830875.0, $dec!(63830876))]
        #[case(64344821.0, $dec!(64344820))]
        #[case(64419642.0, $dec!(64419640))]
        #[case(64501793.0, $dec!(64501792))]
        #[case(64957599.0, $dec!(64957600))]
        #[case(66666666.0, $dec!(66666664))]
        #[case(67301713.0, $dec!(67301712))]
        #[case(68105580.0, $dec!(68105584))]
        #[case(68183731.0, $dec!(68183728))]
        #[case(68517318.0, $dec!(68517320))]
        #[case(68873971.0, $dec!(68873968))]
        #[case(69450330.0, $dec!(69450328))]
        #[case(69685606.0, $dec!(69685608))]
        #[case(69686659.0, $dec!(69686656))]
        #[case(69731732.0, $dec!(69731728))]
        #[case(69956135.0, $dec!(69956136))]
        #[case(70972830.0, $dec!(70972832))]
        #[case(72376922.0, $dec!(72376920))]
        #[case(73166703.0, $dec!(73166704))]
        #[case(73433906.0, $dec!(73433904))]
        #[case(74071854.0, $dec!(74071856))]
        #[case(74396490.0, $dec!(74396488))]
        #[case(75232413.0, $dec!(75232416))]
        #[case(75773725.0, $dec!(75773728))]
        #[case(76247763.0, $dec!(76247760))]
        #[case(76707543.0, $dec!(76707544))]
        #[case(76722281.0, $dec!(76722280))]
        #[case(76756590.0, $dec!(76756592))]
        #[case(77118461.0, $dec!(77118464))]
        #[case(77529147.0, $dec!(77529144))]
        #[case(77777777.0, $dec!(77777776))]
        #[case(79042286.0, $dec!(79042288))]
        #[case(79813406.0, $dec!(79813408))]
        #[case(79823479.0, $dec!(79823480))]
        #[case(80000197e0, $dec!(80000200))]
        #[case(80222122.0, $dec!(80222120))]
        #[case(82018330.0, $dec!(82018328))]
        #[case(82740809.0, $dec!(82740808))]
        #[case(82746620.0, $dec!(82746624))]
        #[case(82838342.0, $dec!(82838344))]
        #[case(83233730.0, $dec!(83233728))]
        #[case(83711047.0, $dec!(83711048))]
        #[case(83725573.0, $dec!(83725576))]
        #[case(83775423.0, $dec!(83775424))]
        #[case(84595161.0, $dec!(84595160))]
        #[case(85084788.0, $dec!(85084784))]
        #[case(85342511.0, $dec!(85342512))]
        #[case(85777219.0, $dec!(85777216))]
        #[case(85785631.0, $dec!(85785632))]
        #[case(85904404.0, $dec!(85904400))]
        #[case(86091465.0, $dec!(86091464))]
        #[case(86484437.0, $dec!(86484440))]
        #[case(86777268.0, $dec!(86777264))]
        #[case(86962508.0, $dec!(86962512))]
        #[case(87736852.0, $dec!(87736848))]
        #[case(87791158.0, $dec!(87791160))]
        #[case(88658996.0, $dec!(88658992))]
        #[case(88961903.0, $dec!(88961904))]
        #[case(89139509.0, $dec!(89139512))]
        #[case(89629465.0, $dec!(89629464))]
        #[case(90012929.0, $dec!(90012928))]
        #[case(91360377.0, $dec!(91360376))]
        #[case(91419389.0, $dec!(91419392))]
        #[case(92038121.0, $dec!(92038120))]
        #[case(92125366.0, $dec!(92125368))]
        #[case(92239407.0, $dec!(92239408))]
        #[case(92267121.0, $dec!(92267120))]
        #[case(93327838.0, $dec!(93327840))]
        #[case(94470938.0, $dec!(94470936))]
        #[case(95053418.0, $dec!(95053416))]
        #[case(95221044.0, $dec!(95221040))]
        #[case(95817703.0, $dec!(95817704))]
        #[case(96505557.0, $dec!(96505560))]
        #[case(96706114.0, $dec!(96706112))]
        #[case(96769950.0, $dec!(96769952))]
        #[case(96882881.0, $dec!(96882880))]
        #[case(96898389.0, $dec!(96898392))]
        #[case(97633526.0, $dec!(97633528))]
        #[case(98182244.0, $dec!(98182240))]
        #[case(98955522.0, $dec!(98955520))]
        #[case(99171105.0, $dec!(99171104))]
        #[case(99343471.0, $dec!(99343472))]
        #[case(99999999.0, $dec!(100000000))]
        #[case(111111111.0, $dec!(111111112))]
        #[case(123456789.0, $dec!(123456792))]
        #[case(126597973.0, $dec!(126597976))]
        #[case(141006042.0, $dec!(141006048))]
        #[case(145877585.0, $dec!(145877584))]
        #[case(216765690.0, $dec!(216765696))]
        #[case(222222222.0, $dec!(222222224))]
        #[case(244140625.0, $dec!(244140624))]
        #[case(272467e3, $dec!(272467008))]
        #[case(314325637.0, $dec!(314325632))]
        #[case(333333333.0, $dec!(333333344))]
        #[case(380406926.0, $dec!(380406912))]
        #[case(444444444.0, $dec!(444444448))]
        #[case(555555555.0, $dec!(555555584))]
        #[case(590723948.0, $dec!(590723968))]
        #[case(596706114.0, $dec!(596706112))]
        #[case(605083704.0, $dec!(605083712))]
        #[case(627050305.0, $dec!(627050304))]
        #[case(633293366.0, $dec!(633293376))]
        #[case(666666666.0, $dec!(666666688))]
        #[case(728900802.0, $dec!(728900800))]
        #[case(756174393.0, $dec!(756174400))]
        #[case(760414536.0, $dec!(760414528))]
        #[case(777777777.0, $dec!(777777792))]
        #[case(802221226.0, $dec!(802221248))]
        #[case(805720085.0, $dec!(805720064))]
        #[case(848405530.0, $dec!(848405504))]
        #[case(856334878.0, $dec!(856334848))]
        #[case(870516656.0, $dec!(870516672))]
        #[case(878678326.0, $dec!(878678336))]
        #[case(888888888.0, $dec!(888888896))]
        #[case(891185938.0, $dec!(891185920))]
        #[case(897478238.0, $dec!(897478208))]
        #[case(905079926.0, $dec!(905079936))]
        #[case(913179899.0, $dec!(913179904))]
        #[case(952429603.0, $dec!(952429632))]
        #[case(987434744.0, $dec!(987434752))]
        #[case(991711052.0, $dec!(991711040))]
        #[case(999999999.0, $dec!(1000000000))]
        #[case(1.00000006e+09, $dec!(1000000064))]
        #[case(1220703125.0, $dec!(1220703104))]
        #[case(1234567890.0, $dec!(1234567936))]
        #[case(2147483314.0, $dec!(2147483264))]
        #[case(2147483315.0, $dec!(2147483264))]
        #[case(2147483351.0, $dec!(2147483392))]
        #[case(2147483352.0, $dec!(2147483392))]
        #[case(2147483388.0, $dec!(2147483392))]
        #[case(2147483389.0, $dec!(2147483392))]
        #[case(2147483425.0, $dec!(2147483392))]
        #[case(2147483426.0, $dec!(2147483392))]
        #[case(2147483462.0, $dec!(2147483520))]
        #[case(2147483463.0, $dec!(2147483520))]
        #[case(2147483499.0, $dec!(2147483520))]
        #[case(2147483500.0, $dec!(2147483520))]
        #[case(2147483536.0, $dec!(2147483520))]
        #[case(2147483537.0, $dec!(2147483520))]
        #[case(2147483573.0, $dec!(2147483520))]
        #[case(2147483574.0, $dec!(2147483520))]
        #[case(2147483610.0, $dec!(2147483648))]
        #[case(2147483611.0, $dec!(2147483648))]
        #[case(2147483647.0, $dec!(2147483648))]
        #[case(2289620000.0, $dec!(2289619968))]
        #[case(2555653131.0, $dec!(2555653120))]
        #[case(2798182244.0, $dec!(2798182144))]
        #[case(3200069671.0, $dec!(3200069632))]
        #[case(3243650005.0, $dec!(3243650048))]
        #[case(3303674053.0, $dec!(3303674112))]
        #[case(3537826145.0, $dec!(3537826048))]
        #[case(4045812296.0, $dec!(4045812224))]
        #[case(4294967295.0, $dec!(4294967296))]
        #[case(4363804324.0, $dec!(4363804160))]
        #[case(4899877186.0, $dec!(4899877376))]
        #[case(5004981478.0, $dec!(5004981248))]
        #[case(5086492111.0, $dec!(5086492160))]
        #[case(5138519684.0, $dec!(5138519552))]
        #[case(5371912364.0, $dec!(5371912192))]
        #[case(5635246428.0, $dec!(5635246592))]
        #[case(5746577930.0, $dec!(5746577920))]
        #[case(6103515625.0, $dec!(6103515648))]
        #[case(63e8, $dec!(6300000256))]
        #[case(7383725573.0, $dec!(7383725568))]
        #[case(7642717713.0, $dec!(7642717696))]
        #[case(791e07, $dec!(7910000128))]
        #[case(8637627989.0, $dec!(8637628416))]
        #[case(8865899617.0, $dec!(8865899520))]
        #[case(8974836059.0, $dec!(8974835712))]
        #[case(9e9, $dec!(8999999488))]
        #[case(9226712162.0, $dec!(9226712064))]
        #[case(94e8, $dec!(9400000512))]
        #[case(942042e4, $dec!(9420420096))]
        #[case(9672793580.0, $dec!(9672793088))]
        #[case(9772470297.0, $dec!(9772470272))]
        #[case(1.3e10, $dec!(12999999488))]
        #[case(1662e7, $dec!(16620000256))]
        #[case(24661173473.0, $dec!(24661174272))]
        #[case(276e8, $dec!(27599998976))]
        #[case(2867e7, $dec!(28669999104))]
        #[case(30517578125.0, $dec!(30517577728))]
        #[case(31e9, $dec!(31000000512))]
        #[case(32907604691.0, $dec!(32907603968))]
        #[case(36837130890.0, $dec!(36837130240))]
        #[case(412e08, $dec!(41200001024))]
        #[case(47879823479.0, $dec!(47879823360))]
        #[case(49914078536.0, $dec!(49914077184))]
        #[case(51000000000.0, $dec!(51000000512))]
        #[case(54e9, $dec!(54000001024))]
        #[case(59e9, $dec!(59000000512))]
        #[case(72E9, $dec!(71999995904))]
        #[case(74000000000.0, $dec!(73999998976))]
        #[case(83e9, $dec!(83000000512))]
        #[case(90000000000.0, $dec!(89999998976))]
        #[case(100000000000.0, $dec!(99999997952))]
        #[case(1e11, $dec!(99999997952))]
        #[case(101e9, $dec!(101000003584))]
        #[case(1463e8, $dec!(146299994112))]
        #[case(152587890625.0, $dec!(152587894784))]
        #[case(200000000000.0, $dec!(199999995904))]
        #[case(242499697392.0, $dec!(242499698688))]
        #[case(280000000000.0, $dec!(280000004096))]
        #[case(3e11, $dec!(299999985664))]
        #[case(400000000000.0, $dec!(399999991808))]
        #[case(539e9, $dec!(539000012800))]
        #[case(612062576589.0, $dec!(612062593024))]
        #[case(612641865679.0, $dec!(612641865728))]
        #[case(757596946075.0, $dec!(757596946432))]
        #[case(762939453125.0, $dec!(762939441152))]
        #[case(819992132456.0, $dec!(819992133632))]
        #[case(1e12, $dec!(999999995904))]
        #[case(2e12, $dec!(1999999991808))]
        #[case(3e12, $dec!(3000000053248))]
        #[case(3814697265625.0, $dec!(3814697205760))]
        #[case(5e012, $dec!(4999999913984))]
        #[case(5354e9, $dec!(5354000220160))]
        #[case(59157491e5, $dec!(5915749122048))]
        #[case(7056562757456.0, $dec!(7056562585600))]
        #[case(8955e9, $dec!(8955000520704))]
        #[case(92829494e5, $dec!(9282948956160))]
        #[case(9641e9, $dec!(9640999911424))]
        #[case(10000000000000.0, $dec!(9999999827968))]
        #[case(1e13, $dec!(9999999827968))]
        #[case(19073486328125.0, $dec!(19073486290944))]
        #[case(20000000000000.0, $dec!(19999999655936))]
        #[case(28000000000000.0, $dec!(27999999098880))]
        #[case(40000000000000.0, $dec!(39999999311872))]
        #[case(42949672960001.0, $dec!(42949672960000))]
        #[case(84595161401484.0, $dec!(84595160645632))]
        #[case(89523386091465.0, $dec!(89523383959552))]
        #[case(95367431640625.0, $dec!(95367433551872))]
        #[case(1e14, $dec!(100000000376832))]
        #[case(290123e9, $dec!(290122994024448))]
        #[case(350000000000000.0, $dec!(350000005513216))]
        #[case(400000000000000.0, $dec!(400000001507328))]
        #[case(40e13, $dec!(400000001507328))]
        #[case(424000000000000.0, $dec!(424000010321920))]
        #[case(468107100525890.0, $dec!(468107109859328))]
        #[case(476837158203125.0, $dec!(476837167759360))]
        #[case(5e14, $dec!(499999993495552))]
        #[case(606060606060606.0, $dec!(606060620546048))]
        #[case(732000000000000.0, $dec!(732000000409600))]
        #[case(800000000000000.0, $dec!(800000003014656))]
        #[case(8e14, $dec!(800000003014656))]
        #[case(1000000000000000.0, $dec!(999999986991104))]
        #[case(1e15, $dec!(999999986991104))]
        #[case(1125899906842624.125, $dec!(1125899906842624))]
        #[case(1125899906842901.875, $dec!(1125899906842624))]
        #[case(1314448000000000.0, $dec!(1314448031612928))]
        #[case(2251799813685248.25, $dec!(2251799813685248))]
        #[case(2251799813685803.75, $dec!(2251799813685248))]
        #[case(2384185791015625.0, $dec!(2384185738133504))]
        #[case(3333333333333333.0, $dec!(3333333245231104))]
        #[case(4000000000000000.0, $dec!(3999999947964416))]
        #[case(4503599627370496.5, $dec!(4503599627370496))]
        #[case(4503599627370497.5, $dec!(4503599627370496))]
        #[case(4503599627475352.5, $dec!(4503599627370496))]
        #[case(4503599627475353.5, $dec!(4503599627370496))]
        #[case(5000000000000000.0, $dec!(5000000136282112))]
        #[case(75e14, $dec!(7500000204423168))]
        #[case(97e14, $dec!(9700000028164096))]
        #[case(8000000000000000.0, $dec!(7999999895928832))]
        #[case(9007199254740993.0, $dec!(9007199254740992))]
        #[case(9007199254740994.0, $dec!(9007199254740992))]
        #[case(9007199254740995.0, $dec!(9007199254740992))]
        #[case(642e14, $dec!(64199998614536192))]
        #[case(465e15, $dec!(464999994420625408))]
        #[case(1e16, $dec!(10000000272564224))]
        #[case(1e17, $dec!(99999998430674944))]
        #[case(96e017, $dec!(9599999849344794624))]
        #[case(943e17, $dec!(94300001784301617152))]
        #[case(1e18, $dec!(999999984306749440))]
        #[case(9e18, $dec!(9000000202358128640))]
        #[case(74e18, $dec!(74000000075650039808))]
        #[case(773886e18, $dec!(773886021945983433179136))]
        #[case(9.862818194192001e18, $dec!(9862818312755347456))]
        #[case(1e19, $dec!(9999999980506447872))]
        #[case(9e19, $dec!(89999996526023147520))]
        #[case(1e20, $dec!(100000002004087734272))]
        #[case(743e20, $dec!(74299999342790489145344))]
        #[case(1e21, $dec!(1000000020040877342720))]
        #[case(1e22, $dec!(9999999778196308361216))]
        #[case(9.999999999999999e22, $dec!(99999997781963083612160))]
        #[case(1e23, $dec!(99999997781963083612160))]
        #[case(1e24, $dec!(1000000013848427855085568))]
        #[case(737e24, $dec!(737000035858794806700408832))]
        #[case(3e27, $dec!(2999999891665487966366597120))]
        #[case(67902e27, $dec!(67901997638271913615623523926016))]
        #[case(124e28, $dec!(1239999973324139877098500128768))]
        #[case(1.58456325029e+29, $dec!(158456325028528675187087900672))]
        #[case(62e29, $dec!(6200000244410018015064117739520))]
        #[case(29620e29, $dec!(2961999943927520460356873594339328))]
        #[case(1e30, $dec!(1000000015047466219876688855040))]
        #[case(86e30, $dec!(86000004014165189042310884622336))]
        #[case(4e31, $dec!(39999999392972829180438379495424))]
        #[case(5e33, $dec!(4999999895107383976803697243979776))]
        #[case(1e32, $dec!(100000003318135351409612647563264))]
        #[case(7e32, $dec!(700000003884134346033221737644032))]
        #[case(5e34, $dec!(50000002045923937981487659687608320))]
        #[case(317e36, $dec!(317000006395220278118691742155288870912))]
        #[case(1e38, $dec!(99999996802856924650656260769173209088))]
        // ----------------
        #[case(7450580596923828125.0, $dec!(7450580511124094976))]
        #[case(92666518056446206563e3, $dec!(92666516292738062745600))]
        #[case(1090544144181609348835077142190.0, $dec!(1090544176580505842031423127552))]
        #[case(1490116119384765625.0, $dec!(1490116157200400384))]
        #[case(2402844368454405395.2, $dec!(2402844398917255168))]
        #[case(298023223876953125.0, $dec!(298023210824237056))]
        #[case(9896800000000000.0, $dec!(9896799724634112))]
        #[case(11920928955078125.0, $dec!(11920928690667520))]
        #[case(59604644775390625.0, $dec!(59604645600821248))]
        // ----------------
        #[case(1.7014118346046923e+38, $dec!(170141183460469231731687303715884105728))]
        #[case(3.4028234664e38, $dec!(340282346638528859811704183484516925440))]
        #[case(3.4028234665e38, $dec!(340282346638528859811704183484516925440))]
        #[case(3.4028234666e38, $dec!(340282346638528859811704183484516925440))]
        // ----------------
        #[case(f32::MAX, $dec!(3.40282346638528859811704183484516925440e+38))]
        // ----------------
        #[case(core::f32::consts::PI, $dec!(3.1415927410125732421875))]
        #[case(core::f32::consts::PI * 10000.0, $dec!(31415.927734375))]
        #[case(core::f32::consts::PI * 30000.0, $dec!(94247.78125))]
        #[case(core::f32::consts::E, $dec!(2.71828174591064453125))]
        #[case(f32::EPSILON, $dec!(1.1920928955078125E-7))]
        fn test_to_f32(#[case] expected: f32, #[case] d: $D) {
            let f = f32::try_from(d).unwrap();
            assert_eq!(f.integer_decode(), expected.integer_decode());
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {};
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case(-1.0e-40, $dec!(-1e-40))]
        #[case(-1.0e-39, $dec!(-1.00000021530533325742087560014568310927e-39))]
        #[case(-3.92e-39, $dec!(-3.92e-39))]
        #[case(-1e-42, $dec!(-1.0006e-42))]
        #[case(-2e-45, $dec!(-2e-45))]
        fn test_to_f32_subnormal_signed_128(#[case] expected: f32, #[case] d: $D) {
            let f = f32::try_from(d).unwrap();
            assert_eq!(f.integer_decode(), expected.integer_decode());
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        $crate::decimal::common::to::float::to_float!(TO S f32, $dec, $D);
        $crate::decimal::common::to::float::to_float!(TO NEG INF f32, $dec, $D);

        #[rstest(::trace)]
        #[case(f32::MIN, $dec!(-3.40282346638528859811704183484516925440e+38))]
        fn test_to_f32_signed(#[case] expected: f32, #[case] d: $D) {
            let f = f32::try_from(d).unwrap();
            assert_eq!(f.integer_decode(), expected.integer_decode());
        }
    };
}

pub(crate) use test_impl;
