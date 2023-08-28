use std::mem::transmute;

use crate::vm::prelude::*;

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
    pub fn try_cast_ref<T: CanObj>(&self) -> Option<&Obj<T>> {
        if std::any::type_name::<T>() != self.typename {
            return None;
        }
        unsafe { (self as *const Self).cast::<Obj<T>>().as_ref() }
    }
    pub fn try_cast<T: CanObj>(self) -> Option<Obj<T>> {
        if std::any::type_name::<T>() != self.typename {
            return None;
        }
        Some(unsafe { transmute(self) })
    }
}
