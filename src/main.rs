#![allow(incomplete_features)]
#![feature(specialization)]

mod core_types;
mod object;
mod prelude;
mod vm;

use prelude::*;

fn main() {
    let obj = Obj::new(10i64).cast_petty();
    let res = obj.get_item("key");
    println!("{res:?}");
}
