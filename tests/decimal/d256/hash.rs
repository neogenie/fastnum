use rstest::*;

use fastnum::{dec256, D256};

use crate::decimal::common::hash::{test_impl, test_impl_signed};

test_impl!(dec256, D256);
test_impl_signed!(dec256, D256);