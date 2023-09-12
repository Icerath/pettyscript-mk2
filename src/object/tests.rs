use super::*;

#[test]
fn test_obj_casts() {
    let petty_obj = Obj::new(String::from("Hello, World!")).cast_petty();
    let ref_obj = petty_obj.cast_ref::<String>().unwrap();
    assert_eq!(ref_obj.value(), "Hello, World!");
    let obj = petty_obj.cast::<String>().unwrap();
    assert_eq!(obj.value(), "Hello, World!");
}

#[test]
fn test_value_obj_casts() {
    let petty_obj = Obj::new(10 * 2 + 3).cast_petty();
    let ref_obj = petty_obj.cast_ref::<i64>().unwrap();
    assert_eq!(*ref_obj.value(), 10 * 2 + 3);
    let obj = petty_obj.cast::<i64>().unwrap();
    assert_eq!(*obj.value(), 10 * 2 + 3);
}

/// tests that objects are dropped proplery
#[derive(Debug)]
struct Dropper(*mut bool);
impl Drop for Dropper {
    fn drop(&mut self) {
        // Safety: Dropper's pointer if valid.
        unsafe { *self.0 = !*self.0 };
    }
}
impl CanObj for Dropper {}
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
    unsafe fn delete(obj: &Obj<PtyPtr>) {
        let this = obj.cast_ref::<Self>().unwrap();
        let value = this.value();
        *value.0 = !*value.0;
        // Safety: Deleter is not a ValueObj
        unsafe {
            dealloc(this.value.assume_init());
            dealloc(this.ref_count.unwrap());
        };
    }
}
#[test]
fn test_deletion() {
    let mut is_deleted = false;
    Obj::new(Deleter(std::ptr::addr_of_mut!(is_deleted)));
    assert!(is_deleted);
}
