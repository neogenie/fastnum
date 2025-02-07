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

macro_rules! assert_eq_size {
    ($x:ty, $($xs:ty),+ $(,)?) => {
        #[cfg(debug_assertions)]
        const _: fn() = || {
            $(let _ = core::mem::transmute::<$x, $xs>;)+
        };
    };
}

pub(crate) use assert_eq_size;
pub(crate) use err_msg;
pub(crate) use err_prefix;
