use diesel::pg::data_types::PgNumeric;

use rstest::*;

use fastnum::{udec128, UD128};

use crate::decimal::common::extras::diesel::pg::{test_impl, test_impl_unsigned};

test_impl!(udec128, UD128);
test_impl_unsigned!(udec128, UD128);
