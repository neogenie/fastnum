use fastnum::*;

#[test]
fn test_from_i32_sign_extends() {
    assert_eq!(I512::from(-1_i32), I512::NEG_ONE);
    assert_eq!(I512::from(-2_i32), I512::NEG_TWO);
    assert_eq!(I512::from(-100_i32), i512!(-100));

    // Positive values should still work
    assert_eq!(I512::from(100_i32), i512!(100));
}

#[test]
fn test_from_i64_sign_extends() {
    assert_eq!(I512::from(-1_i64), I512::NEG_ONE);
    assert_eq!(I512::from(-2_i64), I512::NEG_TWO);
    assert_eq!(I512::from(-100_i64), i512!(-100));

    // Positive values should still work
    assert_eq!(I512::from(100_i64), i512!(100));
}

#[test]
fn test_from_i8_sign_extends() {
    assert_eq!(I512::from(-1_i8), I512::NEG_ONE);
    assert_eq!(I512::from(-128_i8), i512!(-128));
    assert_eq!(I512::from(127_i8), i512!(127));
}

#[test]
fn test_from_i16_sign_extends() {
    assert_eq!(I512::from(-1_i16), I512::NEG_ONE);
    assert_eq!(I512::from(-32768_i16), i512!(-32768));
    assert_eq!(I512::from(32767_i16), i512!(32767));
}

#[test]
fn test_cast_sign_extends() {
    // Negative values must sign-extend
    let neg_one = I512::NEG_ONE;
    let neg_one_cast: I1024 = <I512 as Cast<I1024>>::cast(neg_one);
    assert_eq!(neg_one_cast, I1024::NEG_ONE);
    assert!(neg_one_cast.is_negative());

    let neg_100 = i512!(-100);
    let neg_100_cast: I1024 = <I512 as Cast<I1024>>::cast(neg_100);
    assert_eq!(neg_100_cast, I1024::from(-100_i64));
    assert!(neg_100_cast.is_negative());

    // Positive values must still work
    let pos = i512!(100);
    let pos_cast: I1024 = <I512 as Cast<I1024>>::cast(pos);
    assert_eq!(pos_cast, I1024::from(100_i64));
}

#[test]
fn test_cast_i256_to_i512() {
    let neg_one = I256::NEG_ONE;
    let neg_one_cast: I512 = <I256 as Cast<I512>>::cast(neg_one);
    assert_eq!(neg_one_cast, I512::NEG_ONE);
    assert!(neg_one_cast.is_negative());
}

#[test]
fn test_cast_i128_to_i256() {
    let neg_one = I128::NEG_ONE;
    let neg_one_cast: I256 = <I128 as Cast<I256>>::cast(neg_one);
    assert_eq!(neg_one_cast, I256::NEG_ONE);
    assert!(neg_one_cast.is_negative());
}

#[test]
fn test_try_cast_sign_extends() {
    // Test TryCast for narrowing that should succeed
    let small_neg = I1024::from(-100_i64);
    let result: I512 = <I1024 as TryCast<I512>>::try_cast(small_neg).unwrap();
    assert_eq!(result, I512::from(-100_i64));
    assert!(result.is_negative());

    // Test narrowing -1
    let neg_one = I1024::NEG_ONE;
    let result: I512 = <I1024 as TryCast<I512>>::try_cast(neg_one).unwrap();
    assert_eq!(result, I512::NEG_ONE);
    assert!(result.is_negative());
}

#[test]
fn test_from_i32_boundary_values() {
    // Test i32::MIN
    let min_i32 = I512::from(i32::MIN);
    assert!(min_i32.is_negative());
    assert_eq!(min_i32, i512!(-2147483648));

    // Test i32::MAX
    let max_i32 = I512::from(i32::MAX);
    assert!(max_i32.is_positive());
    assert_eq!(max_i32, i512!(2147483647));
}

#[test]
fn test_from_i64_boundary_values() {
    // Test i64::MIN
    let min_i64 = I512::from(i64::MIN);
    assert!(min_i64.is_negative());
    assert_eq!(min_i64, i512!(-9223372036854775808));

    // Test i64::MAX
    let max_i64 = I512::from(i64::MAX);
    assert!(max_i64.is_positive());
    assert_eq!(max_i64, i512!(9223372036854775807));
}

#[test]
fn test_sign_extension_preserves_value() {
    // Verify that -1 is actually -1, not a large positive number
    let neg_one_i32 = I512::from(-1_i32);
    let neg_one_i64 = I512::from(-1_i64);

    // Both should be equal to -1
    assert_eq!(neg_one_i32, I512::NEG_ONE);
    assert_eq!(neg_one_i64, I512::NEG_ONE);

    // They should be equal to each other
    assert_eq!(neg_one_i32, neg_one_i64);

    // Adding 1 should give 0
    assert_eq!(neg_one_i32 + I512::ONE, I512::ZERO);
}

#[test]
fn test_cast_chain() {
    // Test multiple casts in a chain
    let val = I128::from(-42_i32);
    let val_256: I256 = <I128 as Cast<I256>>::cast(val);
    let val_512: I512 = <I256 as Cast<I512>>::cast(val_256);
    let val_1024: I1024 = <I512 as Cast<I1024>>::cast(val_512);

    // All should represent -42
    assert_eq!(val, I128::from(-42_i64));
    assert_eq!(val_256, I256::from(-42_i64));
    assert_eq!(val_512, I512::from(-42_i64));
    assert_eq!(val_1024, I1024::from(-42_i64));
}
