use std::collections::HashMap;

use super::config_types::{ConfigItem, ConfigValue};

#[derive(Default)]
pub struct ConfigData {
    is_modified: bool,
    root: Option<ConfigItem>,
}

impl ConfigData {
    pub(crate) fn new_with_map() -> Self {
        Self {
            root: Some(ConfigItem::Map(Box::new(HashMap::new()))),
            ..Default::default()
        }
    }

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

    pub(crate) fn traverse(&self, path: &str) -> Result<Option<ConfigItem>, String> {
        let fragments: Vec<&str> = path.split('/').collect();
        let fragments_len = fragments.len();
        let head_opt = &self.root;

        if head_opt.is_none() {
            return Err("None root".to_string());
        }

        let mut head = head_opt.as_ref().unwrap();

        for (fragment_idx, fragment) in fragments.into_iter().enumerate() {
            let is_container = head.is_container();

            if !is_container {
                break;
            }

            let new_head = head.get_item(fragment);
            if new_head.is_none() {
                break;
            }
            head = new_head.unwrap();

            if fragment_idx == fragments_len - 1 {
                return Ok(Some(head.clone()));
            }
        }

        Ok(None)
    }

    pub(crate) fn traverse_write(&mut self, path: &str, value: ConfigValue) -> Result<(), String> {
        self.is_modified = true;

        return self.traverse_copy_on_write(path, value);
    }

    fn traverse_copy_on_write(&mut self, path: &str, value: ConfigValue) -> Result<(), String> {
        let fragments: Vec<&str> = path.split('/').collect();
        let fragments_len = fragments.len();
        let head_opt = &mut self.root;

        if head_opt.is_none() {
            return Err("None root".to_string());
        }

        let mut head = head_opt.as_mut().unwrap();

        for (fragment_idx, fragment) in fragments.into_iter().enumerate() {
            let is_container = head.is_container();

            if !is_container {
                break;
            }

            if fragment_idx == fragments_len - 1 {
                return head.set_item(fragment, ConfigItem::Value(Box::new(value)));
            } else {
                let new_head = head.get_item_mut(fragment);
                if new_head.is_none() {
                    break;
                }
                head = new_head.unwrap();
            }
        }

        Err("Value not found".to_string())
    }
}
