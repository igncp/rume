use std::ffi::{c_char, c_void};
use tracing::debug;

use super::RumeC;
use crate::{
    rume::{NewRumeConfig, Rume},
    rume_api_c::utils::{c_char_to_str, extract_rume_instance, return_result_helper},
};

pub fn rume_new_impl(log_dir_c: *const c_char) -> *mut RumeC {
    let log_dir = c_char_to_str(log_dir_c).map(|s| s.to_string());
    let new_opts = NewRumeConfig {
        app_name: "RumeApp".to_string(),
        min_log_level: None,
        log_dir,
    };
    let inner = Box::into_raw(Box::new(Rume::new(Some(new_opts)))) as *mut c_void;
    let rume_instance = RumeC { inner };
    Box::into_raw(Box::new(rume_instance))
}

/// # Safety
///
/// The caller must ensure that `instance` is a valid pointer
pub unsafe fn rume_free_impl(instance: *mut RumeC) {
    if instance.is_null() {
        return;
    }
    let wrapper = Box::from_raw(instance);
    if !wrapper.inner.is_null() {
        debug!("Freeing Rume instance");
        let _ = Box::from_raw(wrapper.inner as *mut Rume);
    }
}

pub fn rume_init_impl(instance: *mut RumeC) -> i32 {
    let rume_impl = match extract_rume_instance(instance) {
        Some(r) => r,
        _ => return -1,
    };

    return_result_helper(rume_impl.init())
}
