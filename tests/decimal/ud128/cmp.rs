use std::cmp::{max, min};

use rstest::*;

use fastnum::{udec128, UD128};

use crate::decimal::common::cmp::test_impl;

test_impl!(udec128, UD128);