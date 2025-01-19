use super::config_types::ConfigValue;

#[derive(Default)]
pub struct ConfigData;

impl ConfigData {
    pub(crate) fn load_from_file(&mut self) -> bool {
        // // update status
        // file_path_ = file_path;
        // modified_ = false;
        // root.reset();
        // if (!std::filesystem::exists(file_path)) {
        //   LOG(WARNING) << "nonexistent config file '" << file_path << "'.";
        //   return false;
        // }
        // LOG(INFO) << "loading config file '" << file_path << "'.";
        // try {
        //   YAML::Node doc = YAML::LoadFile(file_path.string());
        //   root = ConvertFromYaml(doc, compiler);
        // } catch (YAML::Exception& e) {
        //   LOG(ERROR) << "Error parsing YAML: " << e.what();
        //   return false;
        // }

        true
    }

    pub(crate) fn traverse(&self, path: &str) -> Result<Option<ConfigValue>, String> {
        return Ok(Some(ConfigValue::String("value".to_string())));
    }

    pub(crate) fn traverse_write(&mut self, path: &str, value: ConfigValue) -> Result<(), String> {
        return Ok(());
    }
}
