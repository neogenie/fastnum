use rstest::*;

use fastnum::{decimal::*, *};

// Domain handling of the transcendental / root functions. Several returned a signaling NaN
// (or, for acosh(0), a bogus value) for inputs whose result is well defined. Reference values
// checked against mpmath at 185-digit precision; irrational results are compared after round().
const PREC: i16 = 55;

fn d(s: &str) -> D512 {
    s.parse::<D512>().unwrap()
}

// Attach a non-trapping context so an out-of-domain input yields a quiet NaN instead of trapping.
fn quiet(x: D512) -> D512 {
    x.with_ctx(Context::default().without_traps())
}

// atan is total on ℝ; |x| > 1 previously signaled "invalid operation".
#[rstest]
#[case(dec512!(2), dec512!(1.107148717794090503017065460178537040070047645401432646676539207433710))]
#[case(dec512!(5), dec512!(1.373400766945015860861271926444961148650999595899700808969783355912874))]
#[case(dec512!(-2), dec512!(-1.107148717794090503017065460178537040070047645401432646676539207433710))]
#[case(dec512!(10), dec512!(1.471127674303734591852875571761730851855306377183238262471963519343880))]
#[case(dec512!(100), dec512!(1.560796660108231381024981575430471893537215347143176270859532877957452))]
fn test_atan_gt_one(#[case] x: D512, #[case] expected: D512) {
    let r = x.atan();
    assert!(!r.is_nan());
    assert_eq!(r.round(PREC), expected.round(PREC));
}

// The reduction identity keeps full precision for very large arguments.
#[test]
fn test_atan_large() {
    assert_eq!(
        d("1e12").atan().round(PREC),
        d("1.570796326793896619231321691639751442431918033020886243820805429487242").round(PREC)
    );
    assert_eq!(
        d("1e30").atan().round(PREC),
        d("1.570796326794896619231321691638751442098584699687552910487472296153908").round(PREC)
    );
}

// atan(±∞) = ±π/2.
#[test]
fn test_atan_infinity() {
    assert_eq!(D512::INFINITY.atan(), D512::FRAC_PI_2);
    assert_eq!(D512::NEG_INFINITY.atan(), D512::FRAC_PI_2.neg());
}

// The in-domain path (|x| ≤ 1) is unchanged.
#[rstest]
#[case(dec512!(0), dec512!(0))]
#[case(dec512!(0.5), dec512!(0.463647609000806116214256231461214402028537054286120263810933088720198))]
#[case(dec512!(1), D512::FRAC_PI_4)]
#[case(dec512!(-1), D512::FRAC_PI_4.neg())]
fn test_atan_in_domain(#[case] x: D512, #[case] expected: D512) {
    assert_eq!(x.atan().round(PREC), expected.round(PREC));
}

// atan2 delegates to atan, so every steep quadrant (|y| > |x|) inherited the NaN.
#[rstest]
#[case(dec512!(2), dec512!(1), dec512!(1.107148717794090503017065460178537040070047645401432646676539207433710))]
#[case(dec512!(-3), dec512!(1), dec512!(-1.249045772398254425829917077281090123077829404129896719054669236797152))]
#[case(dec512!(2), dec512!(-1), dec512!(2.034443935795702735445577923100965844127121753973673174298405384874106))]
#[case(dec512!(-5), dec512!(-2), dec512!(-1.951302703907261505534909608450184546595990413345653748063777918478109))]
fn test_atan2_steep(#[case] y: D512, #[case] x: D512, #[case] expected: D512) {
    let r = y.atan2(x);
    assert!(!r.is_nan());
    assert_eq!(r.round(PREC), expected.round(PREC));
}

#[test]
fn test_atan2_origin_is_nan() {
    assert!(quiet(dec512!(0)).atan2(quiet(dec512!(0))).is_nan());
}

// Odd roots of a negative are real: ⁿ√(−x) = −ⁿ√x.
#[rstest]
#[case(dec512!(-8), 3, dec512!(-2))]
#[case(dec512!(-27), 3, dec512!(-3))]
#[case(dec512!(-1000), 3, dec512!(-10))]
#[case(dec512!(-3.375), 3, dec512!(-1.5))]
#[case(dec512!(-32), 5, dec512!(-2))]
#[case(dec512!(-128), 7, dec512!(-2))]
fn test_odd_root_negative(#[case] x: D512, #[case] n: u32, #[case] expected: D512) {
    assert_eq!(x.nth_root(n), expected);
}

#[test]
fn test_cbrt_negative() {
    assert_eq!(dec512!(-8).cbrt(), dec512!(-2));
    assert_eq!(
        dec512!(-2).cbrt().round(PREC),
        d("-1.259921049894873164767210607278228350570251464701507980081975112155300").round(PREC)
    );
}

// Even roots of a negative have no real value and must stay NaN.
#[rstest]
#[case(dec512!(-4), 2)]
#[case(dec512!(-16), 4)]
#[case(dec512!(-64), 6)]
fn test_even_root_negative_is_nan(#[case] x: D512, #[case] n: u32) {
    assert!(quiet(x).nth_root(n).is_nan());
}

// acosh(1) = 0, acosh(+∞) = +∞, and x < 1 (incl. 0 and negatives) has no real value.
#[test]
fn test_acosh_domain() {
    assert_eq!(dec512!(1).acosh(), dec512!(0));
    assert_eq!(
        dec512!(2).acosh().round(PREC),
        d("1.316957896924816708625046347307968444026981971467516479768472256920460").round(PREC)
    );
    assert_eq!(
        dec512!(10).acosh().round(PREC),
        d("2.993222846126380897912667713774182913083660451180980642685145600977499").round(PREC)
    );

    let inf = D512::INFINITY.acosh();
    assert!(inf.is_infinite() && !inf.is_negative());

    for x in [dec512!(0), dec512!(0.5), dec512!(-2)] {
        assert!(quiet(x).acosh().is_nan());
    }
    assert!(quiet(D512::NEG_INFINITY).acosh().is_nan());
}

// Genuine out-of-domain inputs that must keep signaling (boundary guard for the fixes above).
#[test]
fn test_kept_domain_errors() {
    assert!(quiet(D512::INFINITY).asin().is_nan());
    assert!(quiet(dec512!(2)).asin().is_nan());
    assert!(quiet(dec512!(2)).atanh().is_nan());
    assert!(quiet(D512::INFINITY).sin().is_nan());
}
