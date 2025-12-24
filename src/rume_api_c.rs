use crate::rume::Rume as RumeImpl;
use std::ffi::c_void;

#[repr(C)]
pub struct Rume {
    inner: *mut c_void,
    /// # Safety
    /// The caller must ensure that the pointer is valid
    pub init: unsafe extern "C" fn(*mut Rume) -> i32,
}

impl Rume {
    pub extern "C" fn init(&mut self) -> i32 {
        if self.inner.is_null() {
            return -1;
        }
        let rume_impl = unsafe { &mut *(self.inner as *mut RumeImpl) };
        match rume_impl.init() {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }
}

/// # Safety
/// The caller must ensure that `instance` is a valid pointer
#[no_mangle]
pub unsafe extern "C" fn rume_init(instance: *mut Rume) -> i32 {
    if instance.is_null() {
        return -1;
    }
    (*instance).init()
}

#[no_mangle]
pub extern "C" fn rume_new() -> *mut Rume {
    let inner = Box::into_raw(Box::new(RumeImpl::new(None))) as *mut c_void;
    let rume_instance = Rume {
        inner,
        init: rume_init,
    };
    Box::into_raw(Box::new(rume_instance))
}

/// # Safety
///
/// The caller must ensure that `instance` is a valid pointer
#[no_mangle]
pub unsafe extern "C" fn rume_free(instance: *mut Rume) {
    if instance.is_null() {
        return;
    }
    let wrapper = Box::from_raw(instance);
    if !wrapper.inner.is_null() {
        let _ = Box::from_raw(wrapper.inner as *mut RumeImpl);
    }
}
