use std::ffi::{c_void, CString};
use tracing::info;

use super::RumeC;
use crate::{
    rume::{Rume, RumeNewConfig},
    rume_api_c::{
        base::{RumeContextC, RumeMenuC},
        key_code_to_key_table::{extract_modifiers_from_flag, get_key_table_from_key_code},
        utils::{c_char_to_str, extract_rume_instance, get_session_id, return_result_helper},
        RumeKeyEventResultC, RumeNewConfigC, RumeSessionIdC,
    },
};

impl From<*const RumeNewConfigC> for RumeNewConfig {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(val: *const RumeNewConfigC) -> Self {
        let default_app_name = "rume_app".to_string();
        if val.is_null() {
            return RumeNewConfig {
                app_name: default_app_name,
                min_log_level: None,
                log_dir: None,
                stdout_log: true,
            };
        }
        let config_ref = unsafe { &*val };

        RumeNewConfig {
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
    let new_opts: RumeNewConfig = config.into();
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
    let Some(rume_impl) = extract_rume_instance(instance) else {
        return -1;
    };

    return_result_helper(rume_impl.init())
}

pub fn rume_process_key_impl(
    instance: *mut RumeC,
    session_id: RumeSessionIdC,
    key_code: u16,
    modifiers_flag: u32,
) -> RumeKeyEventResultC {
    let Some(rume_impl) = extract_rume_instance(instance) else {
        return RumeKeyEventResultC::RumeKERError;
    };

    let Some(key) = get_key_table_from_key_code(key_code) else {
        info!("Unknown key for key_code: {}", key_code);
        return RumeKeyEventResultC::RumeKERNotHandled;
    };

    info!(
        "Processing key event: session_id='{}' key_code='{}' key='{}'",
        session_id, key_code, key
    );

    let modifiers = extract_modifiers_from_flag(modifiers_flag);

    match rume_impl.process_key(get_session_id(session_id), key, modifiers) {
        Ok(handled) => {
            if handled {
                RumeKeyEventResultC::RumeKERHandled
            } else {
                RumeKeyEventResultC::RumeKERNotHandled
            }
        }
        Err(_) => RumeKeyEventResultC::RumeKERError,
    }
}

pub fn rume_create_session_impl(instance: *mut RumeC) -> RumeSessionIdC {
    let Some(rume_impl) = extract_rume_instance(instance) else {
        return 0;
    };

    rume_impl.create_session() as RumeSessionIdC
}

pub fn rume_delete_session_impl(instance: *mut RumeC, session_id: RumeSessionIdC) {
    let Some(rume_impl) = extract_rume_instance(instance) else {
        return;
    };

    rume_impl.delete_session(get_session_id(session_id));
}

pub fn rume_get_context_impl(
    instance: *mut RumeC,
    session_id: RumeSessionIdC,
) -> *const RumeContextC {
    let Some(rume_impl) = extract_rume_instance(instance) else {
        return std::ptr::null();
    };
    rume_impl
        .get_session(get_session_id(session_id))
        .map_or_else(std::ptr::null, |session| {
            let menu = RumeMenuC {
                num_candidates: session.context.menu.num_candidates as u32,
            };
            let preedit_text = CString::new(session.context.preedit_text.clone())
                .unwrap()
                .into_raw();
            let committed_text = CString::new(session.commited_text.clone())
                .unwrap()
                .into_raw();
            let context = RumeContextC {
                menu,
                preedit_text,
                committed_text,
            };

            Box::into_raw(Box::new(context))
        })
}

pub fn rume_free_context_impl(context: *const RumeContextC) {
    if context.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(context as *mut RumeContextC);
    }
}
