use crate::prelude::*;
use std::borrow::Cow;

pub type PtyStr = Cow<'static, str>;

impl CanObj for PtyStr {
    fn __add__(lhs: &Obj<PtyPtr>, rhs: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        // Safety: at the moment we are assuming the first argument on methods is guaranteed to be self.
        let lhs = unsafe { lhs.downcast_ref_unchecked::<Self>() };
        let rhs = rhs.downcast_ref::<Self>().unwrap();

        if lhs.value().is_empty() {
            return rhs.clone().cast_petty();
        } else if rhs.value().is_empty() {
            return lhs.clone().cast_petty();
        }

        Obj::from(lhs.value().to_string() + rhs.value())
    }
    fn __mul__(lhs: &Obj<PtyPtr>, rhs: &Obj<PtyPtr>) -> Obj<PtyPtr> {
        // Safety: at the moment we are assuming the first argument on methods is guaranteed to be self.
        let lhs = unsafe { lhs.downcast_ref_unchecked::<Self>() };
        let rhs = rhs.downcast_ref::<Int>().unwrap();

        if lhs.value().is_empty() {
            return lhs.clone().cast_petty();
        }
        let int = usize::try_from(*rhs.value()).unwrap_or(0);
        Obj::from(lhs.value().repeat(int))
    }
}

impl From<&'static str> for Obj<PtyPtr> {
    fn from(value: &'static str) -> Self {
        PtyStr::from(value).into()
    }
}

impl From<String> for Obj<PtyPtr> {
    fn from(value: String) -> Self {
        PtyStr::from(value).into()
    }
}

#[test]
fn test_pty_str() {
    let literal = Obj::from("Hello, ");
    let owned = Obj::from("World!".to_owned());

    let added: Obj<PtyStr> = PtyPtr::__add__(&literal, &owned).downcast().unwrap();
    assert_eq!(added.value(), "Hello, World!");
    // let multiplied: Obj<PtyStr> = PtyPtr::__mul__(&literal, &Obj::from(3)).downcast().unwrap();
    // assert_eq!(multiplied.value(), &"Hello, World!".repeat(3));
}
