use crate::rime_api::{RimeConfig, RimeCustomSettings};
pub struct RimeLeversApi;

impl From<&'static str> for RimeLeversApi {
    fn from(_s: &'static str) -> Self {
        Self
    }
}

impl RimeLeversApi {
    pub fn custom_settings_init(&self, config_id: &str, schema_id: &str) -> RimeCustomSettings {
        RimeCustomSettings
    }
    pub fn load_settings(&self, settings: &RimeCustomSettings) {}
    pub fn customize_item(
        &self,
        settings: &RimeCustomSettings,
        key: &str,
        config: &RimeConfig,
    ) -> bool {
        true
    }
    pub fn save_settings(&self, settings: &RimeCustomSettings) {}
    pub fn custom_settings_destroy(&self, settings: RimeCustomSettings) {}
}
