mod ptyptr;
mod vtable;

use core::fmt;
use core::ptr::NonNull;
pub use ptyptr::PtyPtr;
use std::any::type_name;
use std::mem::{size_of, transmute, transmute_copy};
use vtable::Vtable;

#[repr(C)]
pub struct Obj<T: CanObj> {
    ref_count: NonNull<isize>,
    inner: NonNull<T>,
    typename: &'static str,
    vtable: Vtable,
}

pub trait CanObj: Clone {
    fn get_item(obj: &Obj<PtyPtr>, key: &str) -> Obj<PtyPtr>;
    fn set_item(obj: &Obj<PtyPtr>, key: &str, val: &Obj<PtyPtr>);
    fn call(obj: &Obj<PtyPtr>) -> Obj<PtyPtr>;
}

pub trait ObjNew<T> {
    fn new(inner: T) -> Self;
}

impl<T: CanObj> ObjNew<T> for Obj<T> {
    default fn new(inner: T) -> Self {
        debug_assert_ne!(type_name::<T>(), type_name::<i64>());
        let inner = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(inner))) };
        let ref_count = NonNull::new(Box::into_raw(Box::new(1))).unwrap();
        let vtable = Vtable::new::<T>();
        let typename = type_name::<T>();

        Self {
            ref_count,
            inner,
            typename,
            vtable,
        }
    }
}

impl<T: CanObj + Copy> ObjNew<T> for Obj<T> {
    fn new(inner: T) -> Self {
        debug_assert_eq!(size_of::<T>(), size_of::<NonNull<T>>());
        let inner = unsafe { transmute_copy(&inner) };
        let ref_count = NonNull::new(Box::into_raw(Box::new(isize::MIN + 1))).unwrap();
        let vtable = Vtable::new::<T>();
        let typename = type_name::<T>();

        Self {
            ref_count,
            inner,
            typename,
            vtable,
        }
    }
}

impl<T: CanObj> Obj<T> {
    pub fn cast_petty(self) -> Obj<PtyPtr> {
        unsafe { transmute(self) }
    }
    pub fn cast_petty_ref(&self) -> &Obj<PtyPtr> {
        unsafe { &*(self as *const Obj<T>).cast() }
    }
    pub fn get_item(&self, key: &str) -> Obj<PtyPtr> {
        T::get_item(self.cast_petty_ref(), key)
    }
    pub fn inner(&self) -> &T {
        unsafe {
            if self.is_copy() {
                std::ptr::addr_of!(self.inner).cast::<T>().as_ref().unwrap()
            } else {
                self.inner.as_ref()
            }
        }
    }
    pub fn is_copy(&self) -> bool {
        unsafe { *self.ref_count.as_ptr() < 0 }
    }
}

impl<T: CanObj> fmt::Display for Obj<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("Obj")
                .field("type", &self.typename)
                .field("ref_count", self.ref_count.as_ref())
                .finish()
        }
    }
}

impl<T: CanObj> Drop for Obj<T> {
    fn drop(&mut self) {
        unsafe {
            *self.ref_count.as_ptr() -= 1;
            if *self.ref_count.as_ptr() == 0 {
                std::ptr::drop_in_place(self.inner.as_ptr());
            }
        }
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl<T: CanObj> fmt::Debug for Obj<T> {
    default fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Obj")
            .field("type", &self.typename)
            .field("ref_count", unsafe { self.ref_count.as_ref() })
            .field("inner", &"")
            .finish()
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl<T: CanObj + fmt::Debug> fmt::Debug for Obj<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Obj")
            .field("type", &self.typename)
            .field("ref_count", unsafe { self.ref_count.as_ref() })
            .field("inner", self.inner())
            .finish()
    }
}

impl<T: CanObj> Clone for Obj<T> {
    fn clone(&self) -> Self {
        unsafe { *self.ref_count.as_ptr() += 1 };
        Self {
            ref_count: self.ref_count,
            inner: self.inner,
            typename: self.typename,
            vtable: self.vtable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{size_of, transmute};

    fn is_valid<T: CanObj>(val: T) -> bool {
        assert_eq!(size_of::<Obj<T>>(), size_of::<Obj<PtyPtr>>());
        let obj = Obj::new(val);

        let obj_repr: [u8; 56] = unsafe { transmute(obj.clone()) };
        let pty_repr: [u8; 56] = unsafe { transmute(obj.cast_petty()) };

        pty_repr == obj_repr
    }

    #[test]
    fn test_valid() {
        assert!(is_valid(0));
        assert!(is_valid(String::new()));
    }
}
