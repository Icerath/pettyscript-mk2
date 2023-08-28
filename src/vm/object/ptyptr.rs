use crate::vm::prelude::*;

#[derive(Clone, Copy)]
pub struct PtyPtr;

impl CanObj for PtyPtr {
    fn get_item(obj: &Obj<PtyPtr>, key: &str) -> Obj<PtyPtr> {
        ((obj.vtable).get_item)(obj, key)
    }
}

impl Obj<PtyPtr> {
    pub fn try_cast<T: CanObj>(&self) -> Option<&Obj<T>> {
        if std::any::type_name::<T>() != self.typename {
            return None;
        }
        unsafe { (self as *const Self).cast::<Obj<T>>().as_ref() }
    }
}
