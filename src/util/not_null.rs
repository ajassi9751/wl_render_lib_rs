// Just a pointer that is garunteed to be not null
// Better than NonNull because you don't have to check if its valid every time you access it
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
    // Could be accesed without &mut self but that seems a bit unsafe
    pub fn get_mut (&mut self) -> *mut T {
        self.ptr
    }
    pub fn get (&self) -> *const T {
        self.ptr
    }
    pub fn as_mut (&mut self) -> &mut T {
        unsafe {
            self.ptr.as_mut_unchecked()
        }
    }
    pub fn as_ref (&self) -> &T {
        unsafe {
            self.ptr.as_ref_unchecked()
        }
    }
}

#[test]
fn not_null_none_test () {
    let ptr: Option<NotNull<u8>> = NotNull::try_from(core::ptr::null_mut::<u8>());
    assert!(matches!(ptr, None));
}