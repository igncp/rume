use std::{ffi::CString, os::raw::c_char, sync::OnceLock};

use crate::rume_api_c::{
    base::{RumeC, RumeContextC, RumeKeyEventResultC, RumeNewConfigC, RumeSessionIdC},
    rume_api_c_impl::{
        rume_create_session_impl, rume_delete_session_impl, rume_free_context_impl,
        rume_get_context_impl,
    },
};
use rume_api_c_impl::{rume_free_impl, rume_init_impl, rume_new_impl, rume_process_key_impl};

mod base;
mod key_code_to_key_table;
mod rume_api_c_impl;
mod utils;

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

#[no_mangle]
pub extern "C" fn rume_get_context(
    instance: *mut RumeC,
    session_id: RumeSessionIdC,
) -> *const RumeContextC {
    rume_get_context_impl(instance, session_id)
}

#[no_mangle]
pub extern "C" fn rume_free_context(context: *const RumeContextC) {
    rume_free_context_impl(context)
}

#[no_mangle]
pub extern "C" fn rume_version() -> *const c_char {
    static VERSION: OnceLock<CString> = OnceLock::new();
    VERSION
        .get_or_init(|| CString::new(crate::rume::version::RUME_VERSION).unwrap())
        .as_ptr()
}

#[no_mangle]
pub extern "C" fn rume_commit_hash() -> *const c_char {
    static COMMIT: OnceLock<CString> = OnceLock::new();
    COMMIT
        .get_or_init(|| CString::new(crate::rume::version::RUME_COMMIT_HASH).unwrap())
        .as_ptr()
}
