use rstest::*;

use fastnum::{udec512, UD512};

use crate::decimal::common::scale::test_impl;

test_impl!(udec512, UD512);
