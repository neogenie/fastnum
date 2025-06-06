use fastnum::*;

fn main() {
    assert_eq!(
        D128::E * D128::E,
        dec128!(7.3890560989306502272304274605750078132)
    );
    assert_eq!(
        D128::E.powi(2),
        dec128!(7.3890560989306502272304274605750078132)
    );
    assert_eq!(
        D128::E.pow(dec128!(2)),
        dec128!(7.3890560989306502272304274605750078132)
    );
    assert_eq!(
        D128::E.pow(dec128!(2.0)),
        dec128!(7.3890560989306502272304274605750078132)
    );
    assert_eq!(
        D128::PI.powi(2),
        dec128!(9.8696044010893586188344909998761511353)
    );
}
