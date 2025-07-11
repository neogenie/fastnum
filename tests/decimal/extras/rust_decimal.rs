use crate::decimal::common::to::rust_decimal::test_impl;

test_impl!(D, 64);
test_impl!(UD, 64);
test_impl!(D, 128);
test_impl!(UD, 128);
test_impl!(D, 256);
test_impl!(UD, 256);
test_impl!(D, 512);
test_impl!(UD, 512);
test_impl!(D, 1024);
test_impl!(UD, 1024);
test_impl!(D, 2048);
test_impl!(UD, 2048);
// 4096 and 8192 bit widths are causing compilation timeout issues with long constant evaluation
