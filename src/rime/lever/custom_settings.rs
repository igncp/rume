use crate::rime::{config::config_component::Config, service::Service};

#[derive(Default)]
pub struct RimeCustomSettings {
    config: Config,
    config_id: String,
    custom_config: Config,
    generator_id: String,
    modified: bool,
}

impl RimeCustomSettings {
    pub fn new(config_id: &str, generator_id: &str) -> Self {
        Self {
            config_id: config_id.to_string(),
            generator_id: generator_id.to_string(),
            ..Default::default()
        }
    }

    pub(crate) fn load(&mut self) -> bool {
        let (staging_dir, prebuilt_data_dir, user_data_dir) = {
            let service = Service::instance().lock().unwrap();

            (
                service.deployer().staging_dir.clone(),
                service.deployer().prebuilt_data_dir.clone(),
                service.deployer().user_data_dir.clone(),
            )
        };

        let config_path = staging_dir.join(format!("{}.yaml", self.config_id));
        // if (!config_.LoadFromFile(config_path)) {
        //   config_path = deployer_->prebuilt_data_dir / (config_id_ + ".yaml");
        //   if (!config_.LoadFromFile(config_path)) {
        //     LOG(WARNING) << "cannot find '" << config_id_ << ".yaml'.";
        //   }
        // }

        let custom_config_path = user_data_dir.join(format!("{}.yaml", self.config_id));
        // if (!custom_config_.LoadFromFile(custom_config_path)) {
        //   return false;
        // }

        self.modified = false;

        true
    }
}
