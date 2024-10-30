use std::cmp::{max, min};

use rstest::*;

use fastnum::{udec256, UD256};

use crate::decimal::common::cmp::{test_impl, test_impl_256};

test_impl!(udec256, UD256);
test_impl_256!(udec256, UD256);
