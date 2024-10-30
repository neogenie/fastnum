use rstest::*;

use fastnum::{dec512, D512};

use crate::decimal::common::hash::{test_impl, test_impl_signed};

test_impl!(dec512, D512);
test_impl_signed!(dec512, D512);
