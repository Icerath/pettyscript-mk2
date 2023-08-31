use super::*;

/// tests that objects are dropped proplery
#[derive(Debug)]
struct Dropper<'a>(&'a mut bool);
impl<'a> Drop for Dropper<'a> {
    fn drop(&mut self) {
        *self.0 = !*self.0;
    }
}
impl<'a> CanObj for Dropper<'a> {}
#[test]
fn test_drop() {
    let mut is_dropped = false;
    Dropper(&mut is_dropped);
    assert!(is_dropped);
    Obj::new(Dropper(&mut is_dropped));
    assert!(!is_dropped);
    Obj::new(Dropper(&mut is_dropped)).cast_petty();
    assert!(is_dropped);
}

/// tests that objects are deleted proplery
#[derive(Debug)]
struct Deleter(*mut bool);

impl CanObj for Deleter {
    fn delete(obj: &Obj<PtyPtr>) {
        let self_ = obj.clone().cast::<Self>().unwrap();
        let value = self_.value();
        unsafe { *value.0 = !*value.0 };
        unsafe { dealloc(self_.value) };
    }
}
#[test]
fn test_deletion() {
    let mut is_deleted = false;
    Obj::new(Deleter(&mut is_deleted as *mut bool));
    assert!(is_deleted);
}
