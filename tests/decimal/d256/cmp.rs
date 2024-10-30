use std::cmp::{max, min};

use rstest::*;

use fastnum::{dec256, D256};

use crate::decimal::common::cmp::{test_impl, test_impl_256, test_impl_signed, test_impl_signed_256};

test_impl!(dec256, D256);
test_impl_256!(dec256, D256);
test_impl_signed!(dec256, D256);
test_impl_signed_256!(dec256, D256);


