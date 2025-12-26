use std::ffi::c_void;
use tracing::info;

use super::RumeC;
use crate::{
    rume::{NewRumeConfig, Rume},
    rume_api_c::{
        key_code_to_key_table::{extract_modifiers_from_flag, get_key_table_from_key_code},
        utils::{c_char_to_str, extract_rume_instance, return_result_helper},
        RumeKeyEventResultC, RumeNewConfigC,
    },
};

impl From<*const RumeNewConfigC> for NewRumeConfig {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(val: *const RumeNewConfigC) -> Self {
        let default_app_name = "rume_app".to_string();
        if val.is_null() {
            return NewRumeConfig {
                app_name: default_app_name,
                min_log_level: None,
                log_dir: None,
                stdout_log: true,
            };
        }
        let config_ref = unsafe { &*val };

        NewRumeConfig {
            app_name: c_char_to_str(config_ref.app_name)
                .unwrap_or(&default_app_name)
                .to_string(),
            min_log_level: None,
            log_dir: c_char_to_str(config_ref.log_dir).map(|s| s.to_string()),
            stdout_log: config_ref.stdout_log,
        }
    }
}

pub fn rume_new_impl(config: *const RumeNewConfigC) -> *mut RumeC {
    let new_opts: NewRumeConfig = config.into();
    let inner = Box::into_raw(Box::new(Rume::new(Some(new_opts)))) as *mut c_void;
    let rume_instance = RumeC { inner };
    Box::into_raw(Box::new(rume_instance))
}

pub fn rume_free_impl(instance: *mut RumeC) {
    if instance.is_null() {
        return;
    }
    unsafe {
        let wrapper = Box::from_raw(instance);
        if !wrapper.inner.is_null() {
            info!("Freeing Rume instance");
            let _ = Box::from_raw(wrapper.inner as *mut Rume);
        }
    }
}

pub fn rume_init_impl(instance: *mut RumeC) -> i32 {
    let rume_impl = match extract_rume_instance(instance) {
        Some(r) => r,
        _ => return -1,
    };

    return_result_helper(rume_impl.init())
}

pub fn rume_handle_key_down_impl(
    instance: *mut RumeC,
    key_code: u16,
    modifiers_flag: u32,
) -> RumeKeyEventResultC {
    let rume_impl = match extract_rume_instance(instance) {
        Some(r) => r,
        _ => return RumeKeyEventResultC::Error,
    };

    let Some(key) = get_key_table_from_key_code(key_code) else {
        info!("Unknown key for key_code: {}", key_code);
        return RumeKeyEventResultC::NotHandled;
    };

    let modifiers = extract_modifiers_from_flag(modifiers_flag);

    match rume_impl.handle_key_down(key, modifiers) {
        Ok(handled) => {
            if handled {
                RumeKeyEventResultC::Handled
            } else {
                RumeKeyEventResultC::NotHandled
            }
        }
        Err(_) => RumeKeyEventResultC::Error,
    }
}
