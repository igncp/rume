use tracing::{debug, info};

use crate::rume::logger::setup_logs;

pub mod config_handler;
pub mod logger;
pub mod version;

pub struct NewRumeConfig {
    pub app_name: String,
    pub min_log_level: Option<u32>,
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
        setup_logs();
        info!("Rume initializing...");
        Ok(())
    }
}
