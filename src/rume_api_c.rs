use rume_api_c_impl::{rume_free_impl, rume_init_impl, rume_new_impl};
use std::ffi::{c_char, c_void};

mod rume_api_c_impl;
mod utils;

#[repr(C)]
pub struct RumeC {
    inner: *mut c_void,
}

#[no_mangle]
pub extern "C" fn rume_init(instance: *mut RumeC) -> i32 {
    rume_init_impl(instance)
}

#[no_mangle]
pub extern "C" fn rume_new(log_dir: *const c_char) -> *mut RumeC {
    rume_new_impl(log_dir)
}

/// # Safety
///
/// The caller must ensure that `instance` is a valid pointer
#[no_mangle]
pub unsafe extern "C" fn rume_free(instance: *mut RumeC) {
    rume_free_impl(instance);
}
