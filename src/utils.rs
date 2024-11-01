macro_rules! err_prefix {
    () => {
        "(fastnum)"
    };
}

macro_rules! err_msg {
    ($msg: expr) => {
        concat!($crate::utils::err_prefix!(), " ", $msg)
    };
}

pub(crate) use err_prefix;
pub(crate) use err_msg;