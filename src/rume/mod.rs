use tracing::{debug, info};

use crate::rume::logger::setup_logs;

pub mod config_handler;
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
}
