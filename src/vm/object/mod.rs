mod can_obj;
mod ptyptr;
mod vtable;

pub use can_obj::CanObj;
use core::fmt;
use core::ptr::NonNull;
pub use ptyptr::PtyPtr;
use std::any::type_name;
use std::mem::transmute;
use vtable::Vtable;

#[repr(C)]
pub struct Obj<T: CanObj> {
    ref_count: NonNull<usize>,
    inner: NonNull<T>,
    typename: &'static str,
    vtable: Vtable,
}

impl<T: CanObj> Obj<T> {
    pub fn new(inner: T) -> Self {
        debug_assert_ne!(type_name::<T>(), type_name::<i128>());
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
    pub fn cast_petty(self) -> Obj<PtyPtr> {
        unsafe { std::mem::transmute(self) }
    }
    pub fn cast_petty_ref(&self) -> &Obj<PtyPtr> {
        unsafe { &*(self as *const Obj<T>).cast() }
    }
    pub fn get_item(&self, key: &str) -> Obj<PtyPtr> {
        T::get_item(self.cast_petty_ref(), key)
    }
    pub fn inner(&self) -> &T {
        debug_assert_ne!(self.typename, type_name::<i128>());
        unsafe { self.inner.as_ref() }
    }
}

impl Obj<i128> {
    pub fn new_int(val: i128) -> Self {
        let (ref_count, inner): (_, _) = unsafe { transmute(val) };
        Self {
            ref_count,
            inner,
            typename: type_name::<i128>(),
            vtable: Vtable::new::<i128>(),
        }
    }
    pub fn inner_int(&self) -> i128 {
        unsafe { transmute((self.ref_count, self.inner)) }
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

#[allow(clippy::missing_fields_in_debug)]
impl<T: CanObj> fmt::Debug for Obj<T> {
    default fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let mut writer = f.debug_struct("Obj");
            writer.field("type", &self.typename);

            if let Some(int) = self.cast_petty_ref().try_cast::<i128>() {
                writer.field("ref_count", &0i32);
                writer.field("inner", &int.inner_int());
            } else {
                writer.field("ref_count", self.ref_count.as_ref());
                writer.field("inner", &"");
            }

            writer.finish()
        }
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl<T: CanObj + fmt::Debug> fmt::Debug for Obj<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let mut writer = f.debug_struct("Obj");
            writer.field("type", &self.typename);

            if let Some(int) = self.cast_petty_ref().try_cast::<i128>() {
                writer.field("ref_count", &0i32);
                writer.field("inner", &int.inner_int());
            } else {
                writer.field("ref_count", self.ref_count.as_ref());
                writer.field("inner", self.inner());
            }

            writer.finish()
        }
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

        let obj_repr: [u8; 40] = unsafe { transmute(obj.clone()) };
        let pty_repr: [u8; 40] = unsafe { transmute(obj.cast_petty()) };

        pty_repr == obj_repr
    }

    #[test]
    fn test_valid() {
        assert!(is_valid(0));
        assert!(is_valid(String::new()));
    }
}
