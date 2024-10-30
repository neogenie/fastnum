macro_rules! err_prefix {
    () => {
        "(fastnum)"
    };
}

pub(crate) use err_prefix;

macro_rules! err_msg {
    ($msg: expr) => {
        concat!(err_prefix!(), " ", $msg)
    };
}

pub(crate) use err_msg;