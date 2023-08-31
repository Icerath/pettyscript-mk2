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
        unsafe {
            let this = obj.clone().cast_unchecked::<Self>();
            let value = this.value();
            *value.0 = !*value.0;
            dealloc(this.value);
        }
    }
}
#[test]
fn test_deletion() {
    let mut is_deleted = false;
    Obj::new(Deleter(std::ptr::addr_of_mut!(is_deleted)));
    assert!(is_deleted);
}
