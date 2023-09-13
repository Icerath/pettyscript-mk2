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
    unsafe fn delete(obj: &Obj<PtyPtr>) {
        (obj.vtable.delete)(obj);
    }
    fn __add__(lhs: &Obj<PtyPtr>, rhs: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        (lhs.vtable.__add__)(lhs, rhs)
    }
    fn __sub__(lhs: &Obj<PtyPtr>, rhs: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        (lhs.vtable.__sub__)(lhs, rhs)
    }
    fn __mul__(lhs: &Obj<PtyPtr>, rhs: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        (lhs.vtable.__mul__)(lhs, rhs)
    }
    fn __div__(lhs: &Obj<PtyPtr>, rhs: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        (lhs.vtable.__div__)(lhs, rhs)
    }
}

impl Obj<PtyPtr> {
    pub fn downcast<T: CanObj>(self) -> Option<Obj<T>> {
        if !self.is_type::<T>() {
            return None;
        }
        // Safety: We just checked that this was created with T.
        Some(unsafe { self.downcast_unchecked() })
    }
    pub fn downcast_ref<T: CanObj>(&self) -> Option<&Obj<T>> {
        if !self.is_type::<T>() {
            return None;
        }
        // Safety: We just checked that this was created with T.
        unsafe { Some(self.downcast_ref_unchecked()) }
    }
    /// # Safety
    /// The caller must guarantee that this object was originally created using T
    pub unsafe fn downcast_unchecked<T: CanObj>(self) -> Obj<T> {
        transmute(self)
    }
    /// # Safety
    /// The caller must guarantee that this object was originally created using T
    pub unsafe fn downcast_ref_unchecked<T: CanObj>(&self) -> &Obj<T> {
        &*(self as *const Self).cast::<Obj<T>>()
    }
}
