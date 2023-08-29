#![allow(incomplete_features)]
#![feature(specialization)]

mod core_types;
mod object;
mod prelude;
mod vm;

use prelude::*;

fn main() {
    let obj = unsafe {
        Obj::new(String::from("Hello, World!"))
            .cast_petty()
            .cast_unchecked::<String>()
    };
    let obj2 = obj.clone();
    drop(obj2);
    println!("{obj:?}");
}
