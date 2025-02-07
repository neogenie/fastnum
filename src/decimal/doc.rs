macro_rules! must_use_op {
    () => {
        "this returns the result of the operation, without modifying the original"
    };
}

pub(crate) use must_use_op;

macro_rules! decimal_type_doc {
    ($bits: literal, $sign: literal) => {
        concat!(
            "Strictly exact precision fixed-size decimal number ",
            $sign,
            " number with ",
            $bits,
            "-bit integer for decimal digits."
        )
    };
}

pub(crate) use decimal_type_doc;

macro_rules! decimal_operation_panics {
    ($op: literal) => {
        concat!(
            "\n\n# Panics:\n\n",
            "### debug mode\n\n",
            "This method will panic if ",
            $op,
            " performs with some [Exceptional condition](crate#signaling-flags-and-trap-enablers) and corresponding [Signal] in the [Context]
            is trapped by trap-enabler.
            \n\n",
            "### release mode\n\n",
            "In release mode panic will not occur and result will be [`NaN`](crate#nan).\n\n"
        )
    };
}

pub(crate) use decimal_operation_panics;

macro_rules! decimal_inexact {
    ($op: literal) => {
        concat!(
            "\n\n# Precision\n\n",
            "Since the result of ",
            $op,
            " is irrational number, it can usually only be computed to some finite precision ",
            "from a series of increasingly accurate approximations.\n\n",
            "The result of this operation is mostly inexact and raises [`OP_INEXACT`](crate#signaling-flags-and-trap-enablers) signal.",
            "\n\n"
        )
    };
}

pub(crate) use decimal_inexact;
