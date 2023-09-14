use crate::prelude::*;

pub type Int = i64;

impl CanObj for Int {
    fn __add__(lhs: &Obj<PtyPtr>, rhs: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        // Safety: at the moment we are assuming the first argument on methods
        // is guaranteed to be self.
        let lhs = unsafe { lhs.downcast_ref_unchecked::<Int>() };
        let rhs = rhs.downcast_ref::<Int>().unwrap();

        Obj::from(lhs.value() + rhs.value())
    }
    fn __sub__(lhs: &Obj<PtyPtr>, rhs: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        // Safety: TODO
        let lhs = unsafe { lhs.downcast_ref_unchecked::<Int>() };
        let rhs = rhs.downcast_ref::<Int>().unwrap();

        Obj::from(lhs.value() - rhs.value())
    }
    fn __mul__(lhs: &Obj<PtyPtr>, rhs: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        // Safety: TODO
        let lhs = unsafe { lhs.downcast_ref_unchecked::<Int>() };
        let rhs = rhs.downcast_ref::<Int>().unwrap();

        Obj::from(lhs.value() * rhs.value())
    }
    fn __div__(lhs: &Obj<PtyPtr>, rhs: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        // Safety: TODO
        let lhs = unsafe { lhs.downcast_ref_unchecked::<Int>() };
        let rhs = rhs.downcast_ref::<Int>().unwrap();

        Obj::from(lhs.value() / rhs.value())
    }
}

unsafe impl ValueObj for Int {}

#[test]
fn test_ints() {
    let value_of = |int: &Obj<PtyPtr>| int.downcast_ref::<Int>().map(Obj::value).copied();

    let mut int = Obj::from(5);
    int = PtyPtr::__add__(&int, &Obj::from(3));
    assert_eq!(value_of(&int), Some(8));
    int = PtyPtr::__sub__(&int, &Obj::from(2));
    assert_eq!(value_of(&int), Some(6));
    int = PtyPtr::__mul__(&int, &Obj::from(3));
    assert_eq!(value_of(&int), Some(18));
    int = PtyPtr::__div__(&int, &Obj::from(2));
    assert_eq!(value_of(&int), Some(9));
}
