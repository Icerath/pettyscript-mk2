pub use crate::bytecode::Instruction;
pub use crate::core_types::*;
pub use crate::object::*;
pub use crate::vm::Vm;
pub use core::fmt;
pub use std::ptr::NonNull;
pub use std::rc::Rc;

#[inline]
pub fn alloc<T: fmt::Debug>(val: T) -> NonNull<T> {
    // Safety: Box::into_raw is guaranteed to be non-null
    let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(val))) };
    println!("Alloc ({ptr:?})");
    ptr
}

#[inline]
/// # Safety
/// It is up to the caller to guarantee that `Box::from_raw(ptr.as_ptr())` is safe.
pub unsafe fn dealloc<T: fmt::Debug>(ptr: NonNull<T>) {
    println!("Dealloc ({ptr:?})");
    // Safety: It is up to the caller to determine if this is safe.
    let _ = unsafe { Box::from_raw(ptr.as_ptr()) };
}
