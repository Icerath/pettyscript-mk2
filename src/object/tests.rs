use std::{cell::Cell, rc::Rc};

use super::*;

#[test]
fn test_obj_casts() {
    let petty_obj = Obj::new(String::from("Hello, World!").into_boxed_str()).cast_petty();
    let ref_obj = petty_obj.downcast_ref::<Box<str>>().unwrap();
    assert_eq!(ref_obj.value().as_ref(), "Hello, World!");
    let obj = petty_obj.downcast::<Box<str>>().unwrap();
    assert_eq!(obj.value().as_ref(), "Hello, World!");
}

#[test]
fn test_value_obj_casts() {
    let petty_obj = Obj::new(10 * 2 + 3).cast_petty();
    let ref_obj = petty_obj.downcast_ref::<i64>().unwrap();
    assert_eq!(*ref_obj.value(), 10 * 2 + 3);
    let obj = petty_obj.downcast::<i64>().unwrap();
    assert_eq!(*obj.value(), 10 * 2 + 3);
}

/// tests that objects are dropped proplery
#[derive(Debug)]
struct Dropper(Rc<Cell<bool>>);
impl Drop for Dropper {
    fn drop(&mut self) {
        self.0.set(!self.0.get());
    }
}
impl CanObj for Dropper {}
#[test]
fn test_drop() {
    let is_dropped = Rc::new(Cell::new(false));
    Dropper(is_dropped.clone());
    assert!(is_dropped.get());
    Obj::new(Dropper(is_dropped.clone()));
    assert!(!is_dropped.get());
    Obj::new(Dropper(is_dropped.clone())).cast_petty();
    assert!(is_dropped.get());
}

/// tests that objects are deleted proplery
#[derive(Debug)]
struct Deleter(Rc<Cell<bool>>);

impl CanObj for Deleter {
    unsafe fn delete(obj: &Obj<PtyPtr>) {
        let this = obj.downcast_ref::<Self>().unwrap();
        let value = this.value();
        value.0.set(!value.0.get());
        // Safety: this is the default implementation.
        unsafe {
            dealloc(obj.value.assume_init().cast::<Self>());
            dealloc(obj.ref_count.unwrap());
        };
    }
}
#[test]
fn test_deletion() {
    let is_deleted = Rc::new(Cell::new(false));
    Obj::new(Deleter(is_deleted.clone()));
    assert!(is_deleted.get());
}
