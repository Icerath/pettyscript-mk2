mod ptyptr;
mod vtable;

#[cfg(test)]
mod tests;

use crate::prelude::*;
use core::ptr::NonNull;
pub use ptyptr::PtyPtr;
use std::mem::MaybeUninit;
use vtable::Vtable;

#[repr(C)]
pub struct Obj<T: CanObj> {
    type_id: usize,
    value: MaybeUninit<NonNull<T>>,
    ref_count: Option<NonNull<usize>>,
    vtable: &'static Vtable,
}

pub trait CanObj: fmt::Debug + Sized + 'static {
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
        unsafe { dealloc(obj.cast_ref_unchecked::<Self>().value.assume_init()) };
    }
}

/// Types that implement this trait are stored inside the value ptr instead of being heap allocated.
/// This also avoids the need for the `ref_count` meaning that creating an
/// `Obj<T> where T: ValueObj` has 0 allocations.
/// # Safety
/// Types that implement `ValueObj` must be pointer sized.
pub unsafe trait ValueObj: Copy {}

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
        let value = MaybeUninit::new(alloc(value));
        let ref_count = Some(alloc(1));
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
        unsafe { self.value.assume_init().as_ref() }
    }
}

impl<T: CanObj + ValueObj> ObjImpl<T> for Obj<T> {
    fn new(value: T) -> Self {
        let value = unsafe { std::mem::transmute_copy(&value) };
        let ref_count = None;
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
        // This is safe as this is only being called where T: ValueObj
        // which guarantees that self.value was created from T
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
        unsafe { std::mem::transmute(self) }
    }
    pub fn cast_petty_ref(&self) -> &Obj<PtyPtr> {
        unsafe { &*(self as *const Obj<T>).cast() }
    }
    pub fn get_item(&self, vm: &mut Vm, key: &str) -> Obj<PtyPtr> {
        T::get_item(vm, self.cast_petty_ref(), key)
    }
    pub fn is_value(&self) -> bool {
        self.ref_count.is_none()
    }
}

impl<T: CanObj> fmt::Display for Obj<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("Obj")
                .field("type_id", &self.type_id)
                .field("ref_count", &self.ref_count.map(|ptr| ptr.as_ref()))
                .finish()
        }
    }
}

impl<T: CanObj> Clone for Obj<T> {
    fn clone(&self) -> Self {
        if let Some(ref_count) = self.ref_count {
            unsafe { *ref_count.as_ptr() += 1 };
        }
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
        let Some(ref_count) = self.ref_count else {
            return;
        };
        unsafe {
            if *ref_count.as_ptr() == 1 {
                T::delete(self.cast_petty_ref());
                dealloc(ref_count);
            } else {
                *ref_count.as_ptr() -= 1;
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
            .field("ref_count", unsafe {
                &self.ref_count.map(|ptr| ptr.as_ref())
            })
            .finish()
    }
}

pub fn type_id<T: CanObj + 'static>() -> usize {
    (unsafe { std::mem::transmute::<_, u128>(std::any::TypeId::of::<T>()) } as usize)
}
