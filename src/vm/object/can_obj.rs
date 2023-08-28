use crate::vm::prelude::*;

pub trait CanObj: Clone {
    fn get_item(obj: &Obj<PtyPtr>, key: &str) -> Obj<PtyPtr>;
}
