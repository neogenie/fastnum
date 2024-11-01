use std::env;
use std::path::{Path, PathBuf};

const DEFAULT_PRECISION: &str = "100";
const DEFAULT_ROUNDING_MODE: &str = "HalfEven";
const FMT_EXPONENTIAL_LOWER_THRESHOLD: &str = "5";
const FMT_EXPONENTIAL_UPPER_THRESHOLD: &str = "15";
const FMT_MAX_INTEGER_PADDING: &str = "1000";

fn main() {
    let out_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    write_default_precision_file(&out_dir);
    write_default_rounding_mode(&out_dir);
    write_exponential_format_threshold_file(&out_dir);
}

macro_rules! load_env {
    ($env:ident, $name:literal, $default:ident) => {{
        println!("cargo:rerun-if-env-changed={}", $name);
        $env::var($name).unwrap_or_else(|_| $default.to_owned())
    }};
}

fn write_default_precision_file(out_dir: &Path) {
    let env_var = load_env!(env, "RUST_FASTNUM_DEFAULT_PRECISION", DEFAULT_PRECISION);
    let rust_file_path = out_dir.join("default_precision.rs");

    let default_prec: u32 = env_var
        .parse::<std::num::NonZeroU32>()
        .expect("$RUST_FASTNUM_DEFAULT_PRECISION must be an integer > 0")
        .into();

    let rust_file_contents = format!("const DEFAULT_PRECISION: u64 = {};", default_prec);

    std::fs::write(rust_file_path, rust_file_contents).unwrap();
}

fn write_default_rounding_mode(out_dir: &Path) {
    let rounding_mode_name = load_env!(env, "RUST_FASTNUM_DEFAULT_ROUNDING_MODE", DEFAULT_ROUNDING_MODE);

    let rust_file_path = out_dir.join("default_rounding_mode.rs");
    let rust_file_contents = format!("const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::{};", rounding_mode_name);

    std::fs::write(rust_file_path, rust_file_contents).unwrap();
}

fn write_exponential_format_threshold_file(out_dir: &Path) {
    let low_value = load_env!(env, "RUST_FASTNUM_FMT_EXPONENTIAL_LOWER_THRESHOLD", FMT_EXPONENTIAL_LOWER_THRESHOLD);
    let high_value = load_env!(env, "RUST_FASTNUM_FMT_EXPONENTIAL_UPPER_THRESHOLD", FMT_EXPONENTIAL_UPPER_THRESHOLD);
    let max_padding = load_env!(env, "RUST_FASTNUM_FMT_MAX_INTEGER_PADDING", FMT_MAX_INTEGER_PADDING);

    let low_value: u32 = low_value
        .parse::<std::num::NonZeroU32>()
        .expect("$RUST_FASTNUM_FMT_EXPONENTIAL_LOWER_THRESHOLD must be an integer > 0")
        .into();

    let high_value: u32 = high_value
        .parse::<u32>()
        .expect("$RUST_FASTNUM_FMT_EXPONENTIAL_UPPER_THRESHOLD must be valid u32");

    let max_padding: u32 = max_padding
        .parse::<u32>()
        .expect("$RUST_FASTNUM_FMT_MAX_INTEGER_PADDING must be valid u32");

    let rust_file_path = out_dir.join("exponential_format_threshold.rs");

    let rust_file_contents = [
        format!("const EXPONENTIAL_FORMAT_LEADING_ZERO_THRESHOLD: usize = {};", low_value),
        format!("const EXPONENTIAL_FORMAT_TRAILING_ZERO_THRESHOLD: usize = {};", high_value),
        format!("const FMT_MAX_INTEGER_PADDING: usize = {};", max_padding),
    ];

    std::fs::write(rust_file_path, rust_file_contents.join("\n")).unwrap();
}