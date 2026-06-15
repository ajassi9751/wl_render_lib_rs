// Just a pointer that is garunteed to be not null
// Better than NonNull because you don't have to check if its valid every time you access it
// Its probably still not foolproof if stack memory goes out of scope of if heap memory isn't deallocated or multiple pointers are made causing thread unsaftey
#[derive(Debug, Clone)] // Only implements clone for ImageBuffer but it is unsafe because it can cause data races
pub struct NotNull<T> {
    ptr: *mut T,
}

impl<T> NotNull<T> {
    pub fn try_from(ptr: *mut T) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr: ptr })
        }
    }
    // Could be acessed without &mut self but that seems a bit unsafe
    pub fn get_mut(&mut self) -> *mut T {
        self.ptr
    }
    pub fn get(&self) -> *const T {
        self.ptr
    }
    // Also could be acessed without &mut self but that seems a bit unsafe too
    pub fn as_mut(&mut self) -> &mut T {
        // unsafe { self.ptr.as_mut_unchecked() }
        unsafe { self.ptr.as_mut().unwrap() } // The prior is preferred but in nixpkgs rust, this is an unstable api
    }
    pub fn as_ref(&self) -> &T {
        // unsafe { self.ptr.as_ref_unchecked() }
        unsafe { self.ptr.as_ref().unwrap() } // The prior is preferred but in nixpkgs rust, this is an unstable api
    }
}

#[test]
fn none_test() {
    let ptr: Option<NotNull<u8>> = NotNull::try_from(core::ptr::null_mut::<u8>());
    assert!(matches!(ptr, None));
}

#[test]
fn some_test() {
    let mut buf = [0_u8; 10];
    let ptr: Option<NotNull<u8>> = NotNull::try_from(buf.as_mut_ptr());
    assert!(matches!(ptr, Some(_)));
}
