use rstest::*;

use fastnum::{dec256, D256};

use crate::decimal::common::scale::test_impl;

test_impl!(dec256, D256);