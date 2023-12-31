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
    /// It is also very dangerous to call `Obj::clone` inside this method as that could call this delete function again.
    unsafe fn delete(obj: &Obj<PtyPtr>) {
        // Safety: This is safe as the caller must guarantee that this object was not created with a ValueType
        // and that this function will only ever be called once.
        unsafe {
            dealloc(obj.value.assume_init().cast::<Self>());
            dealloc(obj.ref_count.unwrap_unchecked());
        };
    }

    fn __add__(_: &Obj<PtyPtr>, _: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        todo!()
    }
    fn __sub__(_: &Obj<PtyPtr>, _: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        todo!()
    }
    fn __mul__(_: &Obj<PtyPtr>, _: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        todo!()
    }
    fn __div__(_: &Obj<PtyPtr>, _: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        todo!()
    }
}

/// Types that implement this trait are stored inside the value ptr instead of being heap allocated.
/// This also avoids the need for the `ref_count` meaning that creating an
/// `Obj<T> where T: ValueObj` has 0 allocations.
/// # Safety
/// Types that implement `ValueObj` must be pointer sized or also implement `ZeroObj`.
pub unsafe trait ValueObj: CanObj + Copy {}
/// # Safety
/// Types that implement this trait must be zero sized types.
pub unsafe trait ZeroObj: ValueObj {}

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
        Self {
            value: MaybeUninit::new(alloc(value)),
            ref_count: Some(alloc(1)),
            vtable: T::VTABLE,
        }
    }
    default fn value(&self) -> &T {
        // Safety: This function is only called for non-value types
        unsafe { self.value.assume_init().as_ref() }
    }
}

impl<T: ValueObj> ObjImpl<T> for Obj<T> {
    default fn new(value: T) -> Self {
        Self {
            // Safety: It is up to implementors of the ValueObj trait to guarantee that this is safe.
            value: unsafe { std::mem::transmute_copy(&value) },
            ref_count: None,
            vtable: T::VTABLE,
        }
    }
    default fn value(&self) -> &T {
        // Safety: This is safe as this is only being called where T: ValueObj
        // which guarantees that self.value was created from T
        unsafe { &*std::ptr::addr_of!(self.value).cast::<T>() }
    }
}

impl<T: ZeroObj> ObjImpl<T> for Obj<T> {
    fn new(_: T) -> Self {
        Self {
            value: MaybeUninit::uninit(),
            ref_count: None,
            vtable: T::VTABLE,
        }
    }
    fn value(&self) -> &T {
        // Safety: ZST are allowed to be dangling
        unsafe { std::ptr::NonNull::dangling().as_ref() }
    }
}

impl ObjImpl<PtyPtr> for Obj<PtyPtr> {
    fn new(_: PtyPtr) -> Self {
        Obj::from(Null)
    }
    fn value(&self) -> &PtyPtr {
        &PtyPtr
    }
}

impl<T: CanObj> Obj<T> {
    /// Casts `Obj<T>` into a Obj<PtyPtr>
    /// This is just a transmute and does not mutate the object in any way.
    pub fn cast_petty(self) -> Obj<PtyPtr> {
        // Safety: Casting into a PtyPtr is always safe.
        unsafe { std::mem::transmute(self) }
    }
    /// Casts `&Obj<T>` into a &Obj<PtyPtr>
    /// This is just a `ptr::cast` and does not mutate the object in any way.
    pub fn cast_petty_ref(&self) -> &Obj<PtyPtr> {
        // Safety: Casting into a PtyPtr is always safe.
        unsafe { &*std::ptr::from_ref(self).cast() }
    }
    /// Helper method to call this objects `get_item`.
    pub fn get_item(&self, vm: &mut Vm, key: &str) -> Obj<PtyPtr> {
        T::get_item(vm, self.cast_petty_ref(), key)
    }
    /// Returns true if this `Obj` was created with a `ValueObj`.
    /// It knows this by checking if the reference count was null.
    pub fn is_value(&self) -> bool {
        self.ref_count.is_none()
    }
    /// Returns the reference count behind the `Obj`.
    /// `Obj`s that were created with a `ValueObj` don't have a reference count.
    pub fn ref_count(&self) -> Option<&usize> {
        // Safety: This ref count should always be valid
        unsafe { self.ref_count.map(|ptr| ptr.as_ref()) }
    }
    /// Returns `true` if `self` was created with the same type `U`.
    pub fn is_type<U: CanObj>(&self) -> bool {
        std::ptr::from_ref(self.vtable) == U::VTABLE
    }
}

impl<T: CanObj> From<T> for Obj<PtyPtr> {
    fn from(value: T) -> Self {
        Obj::new(value).cast_petty()
    }
}

impl<T: CanObj> Clone for Obj<T> {
    fn clone(&self) -> Self {
        if let Some(ref_count) = self.ref_count {
            // Safety: We already checked that this is not a ValueObj
            // so if this object exists ref_count should be valid.
            unsafe { *ref_count.as_ptr() += 1 };
        }
        // Safety: it is now safe to do a bit copy because we have incremented the ref count
        // for when that copy is dropped.
        unsafe { std::ptr::read(self) }
    }
}

impl<T: CanObj> Drop for Obj<T> {
    fn drop(&mut self) {
        let Some(ref_count) = self.ref_count else {
            return;
        };
        // Safety:
        // When this type is cloned ref_count is incremented and
        // when this type is dropped ref_count is decremented.
        // This means that when ref_count hits 0 there are no more
        // instances of Obj and it is safe to call delete.
        unsafe {
            *ref_count.as_ptr() -= 1;
            if *ref_count.as_ptr() == 0 {
                T::delete(self.cast_petty_ref());
            }
        }
    }
}

impl<T: CanObj> fmt::Debug for Obj<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Obj")
            .field("value", self.value())
            .field("ref_count", &self.ref_count())
            .finish()
    }
}
