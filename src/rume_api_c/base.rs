use std::ffi::{c_char, c_void};

use crate::rume::ProcessKeyResult;

#[repr(C)]
pub struct RumeC {
    pub(super) inner: *mut c_void,
}

#[repr(C)]
pub struct RumeNewConfigC {
    pub app_name: *const c_char,
    pub log_dir: *const c_char,
    pub stdout_log: bool,
}

#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum RumeKeyEventResultC {
    RumeKERHandled,
    RumeKEREnabled,
    RumeKERDisabled,
    RumeKERNotHandled,
    RumeKERError,
}

pub type RumeSessionIdC = u32;

#[repr(C)]
pub struct RumeMenuC {
    pub num_candidates: u32,
}

#[repr(C)]
pub struct RumeContextC {
    pub menu: RumeMenuC,
    pub preedit_text: *const c_char,
    pub committed_text: *const c_char,
}

impl From<ProcessKeyResult> for RumeKeyEventResultC {
    fn from(val: ProcessKeyResult) -> Self {
        match val {
            ProcessKeyResult::Handled => RumeKeyEventResultC::RumeKERHandled,
            ProcessKeyResult::Enabled => RumeKeyEventResultC::RumeKEREnabled,
            ProcessKeyResult::Disabled => RumeKeyEventResultC::RumeKERDisabled,
            ProcessKeyResult::NotHandled => RumeKeyEventResultC::RumeKERNotHandled,
        }
    }
}
