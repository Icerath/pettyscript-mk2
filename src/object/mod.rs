mod ptyptr;
mod vtable;

#[cfg(test)]
mod tests;

use crate::prelude::*;
use core::ptr::NonNull;
pub use ptyptr::PtyPtr;
use std::mem::{self, MaybeUninit};
use vtable::Vtable;

#[repr(C)]
pub struct Obj<T: CanObj> {
    value: MaybeUninit<NonNull<T>>,
    ref_count: Option<NonNull<usize>>,
    vtable: &'static Vtable,
}

pub trait CanObj: fmt::Debug + Sized + 'static {
    // This can't be edited as the Vtable struct is private.
    const VTABLE: &'static Vtable = Vtable::new::<Self>();
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
    /// # Safety
    /// This function must only ever be called once and it is guaranteed to be called
    /// when `Obj::ref_count == 0`
    ///
    /// This means you must prevent an object's `ref_count` from hitting 0 if you want to call this.
    ///
    /// This function also maybe not be called `where T: ValueType`
    /// Or if this is an `Obj<PtyPtr>` that was created using a `ValueType`
    ///
    /// It is also very dangerous to call `Obj::clone` inside this method.
    unsafe fn delete(obj: &Obj<PtyPtr>) {
        // Safety: This is safe as the caller must guarantee that this object was not created with a ValueType
        // and that this function will only ever be called once.
        unsafe {
            dealloc(obj.downcast_ref_unchecked::<Self>().value.assume_init());
            dealloc(obj.ref_count.unwrap());
        };
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
        let vtable = T::VTABLE;
        Self {
            value,
            ref_count,
            vtable,
        }
    }
    default fn value(&self) -> &T {
        // Safety: This function is only called for non-value types
        unsafe { self.value.assume_init().as_ref() }
    }
}

impl<T: CanObj + ValueObj> ObjImpl<T> for Obj<T> {
    fn new(value: T) -> Self {
        // Safety: It is up to implementors of the ValueObj trait to guarantee that this is safe.
        let value = unsafe { mem::transmute_copy(&value) };
        let ref_count = None;
        let vtable = T::VTABLE;

        Self {
            value,
            ref_count,
            vtable,
        }
    }
    fn value(&self) -> &T {
        // Safety: This is safe as this is only being called where T: ValueObj
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
        // Safety: Casting into a PtyPtr is always safe.
        unsafe { mem::transmute(self) }
    }
    pub fn cast_petty_ref(&self) -> &Obj<PtyPtr> {
        // Safety: Casting into a PtyPtr is always safe.
        unsafe { &*std::ptr::from_ref(self).cast() }
    }
    pub fn get_item(&self, vm: &mut Vm, key: &str) -> Obj<PtyPtr> {
        T::get_item(vm, self.cast_petty_ref(), key)
    }
    pub fn is_value(&self) -> bool {
        self.ref_count.is_none()
    }
    pub fn ref_count(&self) -> Option<&usize> {
        // Safety: This ref count should always be valid
        unsafe { self.ref_count.map(|ptr| ptr.as_ref()) }
    }
    pub fn is_type<U: CanObj>(&self) -> bool {
        std::ptr::from_ref(self.vtable) == std::ptr::from_ref(U::VTABLE)
    }
}

impl<T: CanObj> fmt::Display for Obj<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<T: CanObj> Clone for Obj<T> {
    fn clone(&self) -> Self {
        if let Some(ref_count) = self.ref_count {
            // Safety: We already checked that this is not a ValueObj
            // so if this object exists ref_count should be valid.
            unsafe { *ref_count.as_ptr() += 1 };
        }
        Self {
            ref_count: self.ref_count,
            value: self.value,
            vtable: self.vtable,
        }
    }
}

impl<T: CanObj> Drop for Obj<T> {
    fn drop(&mut self) {
        let Some(ref_count) = self.ref_count else {
            return;
        };
        // Safety:
        unsafe {
            if *ref_count.as_ptr() == 1 {
                T::delete(self.cast_petty_ref());
            } else {
                *ref_count.as_ptr() -= 1;
            }
        }
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl<T: CanObj + fmt::Debug> fmt::Debug for Obj<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Obj")
            .field("value", self.value())
            .field("ref_count", &self.ref_count())
            .finish()
    }
}

pub const fn type_id<T: CanObj + 'static>() -> usize {
    // Safety: This is not safe or valid at all.
    // from my understanding it is pretty likely that
    unsafe { mem::transmute::<_, u128>(std::any::TypeId::of::<T>()) as usize }
}
