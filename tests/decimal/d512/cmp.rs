use std::cmp::{max, min};

use rstest::*;

use fastnum::{dec512, D512};

use crate::decimal::common::cmp::{
    test_impl, test_impl_256, test_impl_512, test_impl_signed, test_impl_signed_256, test_impl_signed_512,
};

test_impl!(dec512, D512);
test_impl_256!(dec512, D512);
test_impl_512!(dec512, D512);

test_impl_signed!(dec512, D512);
test_impl_signed_256!(dec512, D512);
test_impl_signed_512!(dec512, D512);
