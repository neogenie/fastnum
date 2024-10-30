use std::cmp::{max, min};

use rstest::*;

use fastnum::{udec512, UD512};

use crate::decimal::common::cmp::{test_impl, test_impl_256, test_impl_512};

test_impl!(udec512, UD512);
test_impl_256!(udec512, UD512);
test_impl_512!(udec512, UD512);
