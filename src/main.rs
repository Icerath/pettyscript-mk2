#![allow(incomplete_features)]
#![feature(specialization)]

pub mod bytecode;
pub mod core_types;
pub mod object;
pub mod prelude;
pub mod vm;

use prelude::*;

fn main() {
    let value = String::from("Hello, World!");
    let obj = Obj::new(value).cast_petty();
    let value = obj.cast_ref::<String>().unwrap();
    println!("{}", value.value());
}
