use rume_api_c_impl::{rume_free_impl, rume_init_impl, rume_new_impl};
use std::ffi::{c_char, c_void};

mod rume_api_c_impl;
mod utils;

#[repr(C)]
pub struct RumeC {
    inner: *mut c_void,
}

#[repr(C)]
pub struct NewRumeConfigC {
    pub app_name: *const c_char,
    pub log_dir: *const c_char,
    pub stdout_log: bool,
}

#[no_mangle]
pub extern "C" fn rume_new(config: *const NewRumeConfigC) -> *mut RumeC {
    rume_new_impl(config)
}

#[no_mangle]
pub extern "C" fn rume_free(instance: *mut RumeC) {
    rume_free_impl(instance);
}

#[no_mangle]
pub extern "C" fn rume_init(instance: *mut RumeC) -> i32 {
    rume_init_impl(instance)
}
