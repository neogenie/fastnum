use rstest::*;

use fastnum::{dec512, D512};

use crate::decimal::common::scale::test_impl;

test_impl!(dec512, D512);
