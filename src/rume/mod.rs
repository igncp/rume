pub mod config_handler;
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
        println!("Rume initializing...");
        if self.initialized {
            return Ok(());
        }
        self.initialized = true;
        Ok(())
    }
}
