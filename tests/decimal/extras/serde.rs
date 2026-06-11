use crate::decimal::common::extras::serde::test_impl;

test_impl!(D, 128);
test_impl!(D, 256);
test_impl!(D, 512);

test_impl!(UD, 128);
test_impl!(UD, 256);
test_impl!(UD, 512);

#[cfg(feature = "serde-arbitrary-precision")]
mod arbitrary_precision;
