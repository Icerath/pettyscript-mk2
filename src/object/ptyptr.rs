use crate::prelude::*;
use std::mem::transmute;

#[derive(Clone, Copy, Debug)]
pub struct PtyPtr;

impl CanObj for PtyPtr {
    fn get_item(vm: &mut Vm, obj: &Obj<PtyPtr>, key: &str) -> Obj<PtyPtr> {
        ((obj.vtable).get_item)(vm, obj, key)
    }
    fn set_item(vm: &mut Vm, obj: &Obj<PtyPtr>, key: &str, val: &Obj<PtyPtr>) {
        ((obj.vtable).set_item)(vm, obj, key, val);
    }
    fn call(vm: &mut Vm, obj: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        ((obj.vtable).call)(vm, obj)
    }
    fn delete(obj: &Obj<PtyPtr>) {
        (obj.vtable.delete)(obj);
    }
}

impl Obj<PtyPtr> {
    pub fn cast<T: CanObj>(self) -> Option<Obj<T>> {
        if type_id::<T>() != self.type_id {
            return None;
        }
        Some(unsafe { self.cast_unchecked() })
    }
    pub fn cast_ref<T: CanObj>(&self) -> Option<&Obj<T>> {
        if type_id::<T>() != self.type_id {
            return None;
        }
        unsafe { Some(self.cast_ref_unchecked()) }
    }
    /// # Safety
    /// The caller must guarantee that this object was originally created using T
    pub unsafe fn cast_unchecked<T: CanObj>(self) -> Obj<T> {
        transmute(self)
    }
    /// # Safety
    /// The caller must guarantee that this object was originally created using T
    pub unsafe fn cast_ref_unchecked<T: CanObj>(&self) -> &Obj<T> {
        &*(self as *const Self).cast::<Obj<T>>()
    }
}
