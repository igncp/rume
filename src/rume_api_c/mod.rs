use std::ffi::{c_char, c_void};

use rume_api_c_impl::{rume_free_impl, rume_init_impl, rume_new_impl, rume_process_key_impl};

use crate::rume_api_c::rume_api_c_impl::{rume_create_session_impl, rume_delete_session_impl};

mod key_code_to_key_table;
mod rume_api_c_impl;
mod utils;

#[repr(C)]
pub struct RumeC {
    inner: *mut c_void,
}

#[repr(C)]
pub struct RumeNewConfigC {
    pub app_name: *const c_char,
    pub log_dir: *const c_char,
    pub stdout_log: bool,
}

#[repr(C)]
pub enum RumeKeyEventResultC {
    RumeKERHandled,
    RumeKERNotHandled,
    RumeKERError,
}

pub type RumeSessionIdC = u32;

#[no_mangle]
pub extern "C" fn rume_new(config: *const RumeNewConfigC) -> *mut RumeC {
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

#[no_mangle]
pub extern "C" fn rume_create_session(instance: *mut RumeC) -> RumeSessionIdC {
    rume_create_session_impl(instance)
}

#[no_mangle]
pub extern "C" fn rume_delete_session(instance: *mut RumeC, session_id: RumeSessionIdC) {
    rume_delete_session_impl(instance, session_id)
}

#[no_mangle]
pub extern "C" fn rume_process_key(
    instance: *mut RumeC,
    session_id: RumeSessionIdC,
    key_code: u16,
    modifier_flag: u32,
) -> RumeKeyEventResultC {
    rume_process_key_impl(instance, session_id, key_code, modifier_flag)
}
