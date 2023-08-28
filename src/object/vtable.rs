use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Vtable {
    pub get_item: fn(&Obj<PtyPtr>, &str) -> Obj<PtyPtr>,
    pub set_item: fn(&Obj<PtyPtr>, &str, &Obj<PtyPtr>),
    pub call: fn(&Obj<PtyPtr>) -> Obj<PtyPtr>,
}

impl Vtable {
    pub fn new<T: CanObj + ?Sized>() -> Self {
        Self {
            get_item: T::get_item,
            set_item: T::set_item,
            call: T::call,
        }
    }
}
