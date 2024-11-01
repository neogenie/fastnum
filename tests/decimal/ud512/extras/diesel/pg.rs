use diesel::pg::data_types::PgNumeric;

use rstest::*;

use fastnum::{udec512, UD512};

use crate::decimal::common::extras::diesel::pg::{test_impl, test_impl_unsigned};

test_impl!(udec512, UD512);
test_impl_unsigned!(udec512, UD512);
