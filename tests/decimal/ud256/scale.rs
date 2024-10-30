use rstest::*;

use fastnum::{udec256, UD256};

use crate::decimal::common::scale::test_impl;

test_impl!(udec256, UD256);