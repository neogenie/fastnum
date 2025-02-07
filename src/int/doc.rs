macro_rules! int_type_doc {
    ($bits: literal, $sign: literal) => {
        concat!($bits, "-bit ", $sign, " integer type.")
    };
}

pub(crate) use int_type_doc;
