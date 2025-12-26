use std::collections::HashSet;

use tracing::{debug, info};

use crate::rume::{
    key_table::{RumeKeyModifier, RumeKeyTable},
    logger::setup_logs,
};

pub mod config_handler;
pub mod key_table;
pub mod logger;
pub mod version;

#[derive(Clone)]
pub struct NewRumeConfig {
    pub app_name: String,
    pub min_log_level: Option<u32>,
    pub log_dir: Option<String>,
    pub stdout_log: bool,
}

impl Default for NewRumeConfig {
    fn default() -> Self {
        Self {
            app_name: "rume_unknown".to_string(),
            min_log_level: None,
            log_dir: None,
            stdout_log: true,
        }
    }
}

pub struct Rume {
    pub rume_config: Option<NewRumeConfig>,
    initialized: bool,
}

impl Rume {
    pub fn new(opt: Option<NewRumeConfig>) -> Self {
        Self {
            rume_config: opt,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        if self.initialized {
            debug!("Rume already initialized");
            return Ok(());
        }
        self.initialized = true;

        let rume_config = self.rume_config.clone().unwrap_or_default();
        setup_logs(
            &rume_config.app_name,
            rume_config.log_dir,
            rume_config.stdout_log,
        );

        info!("Rume initializing...");
        info!("Rume initialized");

        Ok(())
    }

    // true: event handled, false: not handled
    pub fn handle_key_down(
        &self,
        key: RumeKeyTable,
        modifiers: HashSet<RumeKeyModifier>,
    ) -> Result<bool, String> {
        let modifiers_str = modifiers
            .iter()
            .map(|m| format!("{m}"))
            .collect::<Vec<String>>()
            .join(", ");
        info!("Key down event received: key='{key}' with modifiers='{modifiers_str}'");

        Ok(false)
    }
}
