#![feature(convert)]
#![feature(read_exact)]

extern crate rustc_serialize;

use set1::challenge4;

mod set1;

fn main() {
    println!("result: {}", challenge4().unwrap());
}
