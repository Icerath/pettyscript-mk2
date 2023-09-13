use crate::prelude::*;

pub type Int = i64;

impl CanObj for Int {
    fn __add__(lhs: &Obj<PtyPtr>, rhs: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        unsafe {
            // Safety: at the moment we are assuming the first argument on methods
            // is guaranteed to be self.
            let lhs = lhs.downcast_ref_unchecked::<Int>();
            let rhs = rhs.downcast_ref::<Int>().unwrap();

            Obj::new(lhs.value() + rhs.value()).cast_petty()
        }
    }
}
unsafe impl ValueObj for Int {}
