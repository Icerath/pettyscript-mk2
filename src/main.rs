#![allow(incomplete_features)]
#![feature(specialization)]

pub mod bytecode;
pub mod core_types;
pub mod object;
pub mod prelude;
pub mod vm;

use prelude::*;

fn main() {
    let value = 1i64;
    let obj = Obj::new(value).cast_petty();
    let str = obj.cast_ref::<i64>();
    println!("{str:?}");
}
