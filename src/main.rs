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
    let mut sum = Obj::new(0i64).cast_petty();
    for i in 0..100_000_000 {
        let i = Obj::new(i).cast_petty();
        let temp_sum =
            sum.cast_ref::<i64>().unwrap().value() + i.cast_ref::<i64>().unwrap().value();
        sum = Obj::new(temp_sum).cast_petty();
    }
    println!("{:?}", sum.cast::<i64>());
}
