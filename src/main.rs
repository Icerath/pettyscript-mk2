#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(specialization)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

pub mod bytecode;
pub mod core_types;
pub mod object;
pub mod prelude;
pub mod vm;

use prelude::*;

fn main() {
    let obj = Obj::new(String::from("Hello, World!")).cast_petty();
    let value = obj.cast_ref::<String>().unwrap();
    println!("{:?}", value.value());
}
