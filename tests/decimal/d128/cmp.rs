use std::cmp::{max, min};

use rstest::*;

use fastnum::{dec128, D128};

use crate::decimal::common::cmp::{test_impl, test_impl_signed};

test_impl!(dec128, D128);
test_impl_signed!(dec128, D128);