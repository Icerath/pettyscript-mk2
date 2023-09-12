use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Vtable {
    pub get_item: fn(&mut Vm, &Obj<PtyPtr>, &str) -> Obj<PtyPtr>,
    pub set_item: fn(&mut Vm, &Obj<PtyPtr>, &str, &Obj<PtyPtr>),
    pub call: fn(&mut Vm, &Obj<PtyPtr>) -> Obj<PtyPtr>,
    pub delete: unsafe fn(&Obj<PtyPtr>),
}

impl Vtable {
    pub const fn new<T: CanObj>() -> &'static Self {
        &Self {
            get_item: T::get_item,
            set_item: T::set_item,
            call: T::call,
            delete: T::delete,
        }
    }
}
