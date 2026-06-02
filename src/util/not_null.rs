// Just a pointer that is garunteed to be not null
#[derive(Debug)]
pub struct NotNull<T> {
    ptr: *mut T
}

impl <T> NotNull <T> {
    pub fn try_from (ptr: *mut T) -> Option<Self> {
        if ptr.is_null() {
            None
        }
        else {
            Some(
                Self { ptr:ptr }
            )
        }
    }
    pub fn get (&self) -> *mut T {
        self.ptr
    }
}

#[test]
fn not_null_none_test () {
    let ptr: Option<NotNull<u8>> = NotNull::try_from(core::ptr::null_mut::<u8>());
    assert!(matches!(ptr, None));
}