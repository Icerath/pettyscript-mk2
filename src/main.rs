#![allow(incomplete_features)]
#![feature(specialization)]

mod core_types;
mod object;
mod prelude;
mod vm;

use prelude::*;

fn main() {
    let value = 1i64;
    let obj = Obj::new(value).cast_petty();
    let str = obj.cast_ref::<i64>();
    println!("{str:?}");
}
