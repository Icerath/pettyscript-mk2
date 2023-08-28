#![allow(incomplete_features)]
#![feature(specialization)]

mod vm;
use vm::prelude::*;

fn main() {
    let obj = Obj::new(10i64).cast_petty();
    let res = obj.get_item("key");
    println!("{res:?}");
}
