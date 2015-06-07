#![feature(convert)]

extern crate rustc_serialize;

use set1::single_byte_xor_cipher;

mod set1;

fn main() {
    println!("result: {}", single_byte_xor_cipher());
}
