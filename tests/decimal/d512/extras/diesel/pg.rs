use diesel::pg::data_types::PgNumeric;

use rstest::*;

use fastnum::{dec512, D512};

use crate::decimal::common::extras::diesel::pg::{test_impl, test_impl_signed};

test_impl!(dec512, D512);
test_impl_signed!(dec512, D512);
