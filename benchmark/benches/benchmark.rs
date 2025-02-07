mod add;
mod allocation;
mod div;
mod mul;
mod parse;
mod sub;

use criterion::{criterion_group, criterion_main};

criterion_group!(allocation, allocation::vector);
criterion_group!(parse, parse::from_str);
criterion_group!(math, add::add, mul::mul, div::div, sub::sub);

criterion_main!(allocation, parse, math);
