use std::mem::transmute;

use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct PtyPtr;

impl CanObj for PtyPtr {
    fn get_item(obj: &Obj<PtyPtr>, key: &str) -> Obj<PtyPtr> {
        ((obj.vtable).get_item)(obj, key)
    }
    fn set_item(obj: &Obj<PtyPtr>, key: &str, val: &Obj<PtyPtr>) {
        ((obj.vtable).set_item)(obj, key, val);
    }
    fn call(obj: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        ((obj.vtable).call)(obj)
    }
}

impl Obj<PtyPtr> {
    pub fn cast<T: CanObj>(self) -> Option<Obj<T>> {
        if std::any::type_name::<T>() != self.typename {
            return None;
        }
        Some(unsafe { self.cast_unchecked() })
    }
    pub fn cast_ref<T: CanObj>(&self) -> Option<&Obj<T>> {
        if std::any::type_name::<T>() != self.typename {
            return None;
        }
        unsafe { Some(self.cast_ref_unchecked()) }
    }
    pub unsafe fn cast_unchecked<T: CanObj>(self) -> Obj<T> {
        transmute(self)
    }
    pub unsafe fn cast_ref_unchecked<T: CanObj>(&self) -> &Obj<T> {
        &*(self as *const Self).cast::<Obj<T>>()
    }
}
