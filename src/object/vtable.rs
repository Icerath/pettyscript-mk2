use crate::prelude::*;

type BinOp = fn(&Obj<PtyPtr>, &Obj<PtyPtr>) -> Obj<PtyPtr>;

#[derive(Clone, Copy)]
pub struct Vtable {
    pub get_item: fn(&mut Vm, &Obj<PtyPtr>, &str) -> Obj<PtyPtr>,
    pub set_item: fn(&mut Vm, &Obj<PtyPtr>, &str, &Obj<PtyPtr>),
    pub call: fn(&mut Vm, &Obj<PtyPtr>) -> Obj<PtyPtr>,
    pub delete: unsafe fn(&Obj<PtyPtr>),

    pub __add__: BinOp,
    pub __sub__: BinOp,
    pub __mul__: BinOp,
    pub __div__: BinOp,
}

impl Vtable {
    pub const fn new<T: CanObj>() -> &'static Self {
        &Self {
            get_item: T::get_item,
            set_item: T::set_item,
            call: T::call,
            delete: T::delete,

            __add__: T::__add__,
            __sub__: T::__sub__,
            __mul__: T::__mul__,
            __div__: T::__div__,
        }
    }
}
