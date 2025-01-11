pub use crate::rime::lever::custom_settings::RimeCustomSettings;
use crate::rime_api::RimeConfig;

pub struct RimeLeversApi;

impl RimeLeversApi {
    pub fn custom_settings_init(config_id: &str, schema_id: &str) -> RimeCustomSettings {
        RimeCustomSettings::new(config_id, schema_id)
    }
    pub fn load_settings(settings: &RimeCustomSettings) -> bool {
        settings.load()
    }
    pub fn customize_item(settings: &RimeCustomSettings, key: &str, config: &RimeConfig) -> bool {
        true
    }
    pub fn save_settings(settings: &RimeCustomSettings) {}
    pub fn custom_settings_destroy(settings: RimeCustomSettings) {}
}
