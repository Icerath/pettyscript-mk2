#![feature(const_mut_refs)]
#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(const_type_id)]
#![feature(ptr_from_ref)]
//
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![warn(clippy::undocumented_unsafe_blocks)]

pub mod bytecode;
pub mod core_types;
pub mod object;
pub mod prelude;
pub mod vm;

fn main() {}
