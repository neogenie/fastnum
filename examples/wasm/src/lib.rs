#![no_std]
#![no_main]

extern crate core;
extern crate alloc;

use fastnum::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> alloc::string::String {
    let a = dec128!(1.23456789);
    let b = dec128!(2.34567890);
    let c = a + b;
    alloc::format!("{} + {} = {}", a, b, c)
}