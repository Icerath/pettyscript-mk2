use crate::prelude::*;

impl CanObj for i64 {
    fn get_item(obj: &Obj<PtyPtr>, key: &str) -> Obj<PtyPtr> {
        Obj::new(key.to_uppercase()).cast_petty()
    }
    fn set_item(obj: &Obj<PtyPtr>, key: &str, val: &Obj<PtyPtr>) {
        todo!()
    }
    fn call(obj: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        todo!()
    }
}

impl ValueObj for i64 {}

impl CanObj for String {
    fn get_item(obj: &Obj<PtyPtr>, key: &str) -> Obj<PtyPtr> {
        todo!()
    }
    fn set_item(obj: &Obj<PtyPtr>, key: &str, val: &Obj<PtyPtr>) {
        todo!()
    }
    fn call(obj: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        todo!()
    }
}
