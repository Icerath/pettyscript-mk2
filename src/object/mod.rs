mod ptyptr;
mod vtable;

use crate::prelude::*;
use core::ptr::NonNull;
pub use ptyptr::PtyPtr;
use std::any::type_name;
use std::mem::{size_of, transmute, transmute_copy};
use vtable::Vtable;

#[repr(C)]
pub struct Obj<T: CanObj> {
    type_id: usize,
    value: NonNull<T>,
    ref_count: NonNull<isize>,
    vtable: &'static Vtable,
}

pub trait CanObj: fmt::Debug + Sized {
    fn get_item(vm: &mut Vm, _obj: &Obj<PtyPtr>, _key: &str) -> Obj<PtyPtr> {
        vm.raise_not_implemented();
        Obj::new(Null).cast_petty()
    }
    fn set_item(vm: &mut Vm, _obj: &Obj<PtyPtr>, _key: &str, _val: &Obj<PtyPtr>) {
        vm.raise_not_implemented();
    }
    fn call(vm: &mut Vm, _obj: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        vm.raise_not_implemented();
        Obj::new(Null).cast_petty()
    }
    fn delete(obj: &Obj<PtyPtr>) {
        unsafe { dealloc(obj.cast_ref_unchecked::<Self>().value) };
    }
}

/// Marks a type as an immutable value type
pub trait ValueObj {}

pub trait ObjImpl<T>: private::Seal {
    fn new(value: T) -> Self;
    fn value(&self) -> &T;
}

mod private {
    use super::{CanObj, Obj};
    pub trait Seal {}

    impl<T: CanObj> Seal for Obj<T> {}
}

impl<T: CanObj> ObjImpl<T> for Obj<T> {
    default fn new(value: T) -> Self {
        let value = unsafe { alloc(value) };
        let ref_count = unsafe { alloc(1isize) };
        let vtable = Vtable::new::<T>();
        let type_id = type_id::<T>();

        Self {
            type_id,
            value,
            ref_count,
            vtable,
        }
    }
    default fn value(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}

impl<T: CanObj + ValueObj> ObjImpl<T> for Obj<T> {
    fn new(value: T) -> Self {
        debug_assert_eq!(size_of::<T>(), size_of::<NonNull<T>>());
        let value = unsafe { transmute_copy(&value) };
        let ref_count = unsafe { alloc(isize::MIN + 1) };
        let vtable = &Vtable::new::<T>();
        let type_id = type_id::<T>();

        Self {
            type_id,
            value,
            ref_count,
            vtable,
        }
    }
    fn value(&self) -> &T {
        unsafe { &*std::ptr::addr_of!(self.value).cast::<T>() }
    }
}

impl ObjImpl<PtyPtr> for Obj<PtyPtr> {
    fn value(&self) -> &PtyPtr {
        &PtyPtr
    }
}

impl<T: CanObj> Obj<T> {
    pub fn cast_petty(self) -> Obj<PtyPtr> {
        unsafe { transmute(self) }
    }
    pub fn cast_petty_ref(&self) -> &Obj<PtyPtr> {
        unsafe { &*(self as *const Obj<T>).cast() }
    }
    pub fn get_item(&self, vm: &mut Vm, key: &str) -> Obj<PtyPtr> {
        T::get_item(vm, self.cast_petty_ref(), key)
    }
    pub fn is_value(&self) -> bool {
        unsafe { *self.ref_count.as_ptr() < 0 }
    }
}

impl<T: CanObj> fmt::Display for Obj<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("Obj")
                .field("type_id", &self.type_id)
                .field("ref_count", self.ref_count.as_ref())
                .finish()
        }
    }
}

impl<T: CanObj> Clone for Obj<T> {
    fn clone(&self) -> Self {
        unsafe { *self.ref_count.as_ptr() += 1 };
        Self {
            ref_count: self.ref_count,
            value: self.value,
            type_id: self.type_id,
            vtable: self.vtable,
        }
    }
}

impl<T: CanObj> Drop for Obj<T> {
    fn drop(&mut self) {
        unsafe {
            *self.ref_count.as_ptr() -= 1;
            if *self.ref_count.as_ptr() == 0 {
                T::delete(self.cast_petty_ref());
                dealloc(self.ref_count);
            }
        }
    }
}

// #[allow(clippy::missing_fields_in_debug)]
// impl<T: CanObj> fmt::Debug for Obj<T> {
//     default fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Obj")
//             .field("type_id", &self.type_id)
//             .field("value", &"")
//             .field("ref_count", unsafe { self.ref_count.as_ref() })
//             .finish()
//     }
// }

#[allow(clippy::missing_fields_in_debug)]
impl<T: CanObj + fmt::Debug> fmt::Debug for Obj<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Obj")
            .field("type_id", &self.type_id)
            .field("value", self.value())
            .field("ref_count", unsafe { self.ref_count.as_ref() })
            .finish()
    }
}

pub fn type_id<T: CanObj>() -> usize {
    (type_name::<T>() as *const str).cast::<()>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{size_of, transmute};

    fn is_valid<T: CanObj>(val: T) -> bool {
        assert_eq!(size_of::<Obj<T>>(), size_of::<Obj<PtyPtr>>());
        let obj = Obj::new(val);

        let obj_repr: [u8; 32] = unsafe { transmute(obj.clone()) };
        let pty_repr: [u8; 32] = unsafe { transmute(obj.cast_petty()) };

        pty_repr == obj_repr
    }

    #[test]
    fn test_valid() {
        assert!(is_valid(0));
        assert!(is_valid(String::new()));
    }
}
