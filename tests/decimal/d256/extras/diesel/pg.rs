use diesel::pg::data_types::PgNumeric;

use rstest::*;

use fastnum::{dec256, D256};

use crate::decimal::common::extras::diesel::pg::{test_impl, test_impl_signed};

test_impl!(dec256, D256);
test_impl_signed!(dec256, D256);