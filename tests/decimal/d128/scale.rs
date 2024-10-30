use rstest::*;

use fastnum::{dec128, D128};

use crate::decimal::common::scale::test_impl;

test_impl!(dec128, D128);
