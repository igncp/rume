use super::config_data::ConfigData;
use crate::rime::component::ComponentBase;

#[derive(Default)]
pub struct Config {
    data: ConfigData,
}

impl Config {
    pub(crate) fn load_from_file(&mut self) -> bool {
        self.data.load_from_file()
    }
}

pub struct ConfigComponent;

impl ComponentBase for ConfigComponent {}
