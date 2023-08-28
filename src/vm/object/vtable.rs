use crate::vm::prelude::*;

#[derive(Clone, Copy)]
pub struct Vtable {
    pub get_item: fn(&Obj<PtyPtr>, &str) -> Obj<PtyPtr>,
}

impl Vtable {
    pub fn new<T: CanObj + ?Sized>() -> Self {
        Self {
            get_item: T::get_item,
        }
    }
}
