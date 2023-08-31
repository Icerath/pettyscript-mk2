pub use crate::bytecode::Instruction;
pub use crate::core_types::*;
pub use crate::object::*;
pub use crate::vm::Vm;
pub use core::fmt;
pub use std::ptr::NonNull;
pub use std::rc::Rc;

#[inline]
/// # Safety
pub unsafe fn alloc<T: fmt::Debug>(val: T) -> NonNull<T> {
    println!("Alloc({val:?})");
    NonNull::new_unchecked(Box::into_raw(Box::new(val)))
}

#[inline]
/// # Safety
pub unsafe fn dealloc<T: fmt::Debug>(ptr: NonNull<T>) {
    let val = Box::from_raw(ptr.as_ptr());
    println!("Dealloc({val:?})");
}
