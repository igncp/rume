#[cfg(test)]
mod tests;

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
            return Ok(());
        }
        self.initialized = true;
        Ok(())
    }

    pub fn apply_patch(&self, config_id: &str, key: &str, yaml: &str) -> Result<(), String> {
        println!(
            "Applying patch to config_id: {}, key: {}, yaml: {}",
            config_id, key, yaml
        );

        Ok(())
    }
}
