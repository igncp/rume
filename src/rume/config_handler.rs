use crate::rume::Rume;
use chrono::Local;
use hashlink::LinkedHashMap;
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};

use crate::rume::version::RUME_VERSION;

pub type ConfigCurrentTime = Option<chrono::DateTime<chrono::Local>>;

#[derive(Default)]
pub struct ApplyPatchOpts {
    pub config_id: String,
    pub key: String,
    pub yaml_value: String,
    pub current_time: ConfigCurrentTime,
}

#[derive(Default)]
pub struct ConfigHandler {
    pub config_id: String,
    pub generator: String,
    pub current_time: ConfigCurrentTime,
    pub content: Option<Yaml>,
}

impl ConfigHandler {
    pub fn apply_patch(rume: &Rume, opts: &ApplyPatchOpts) -> Result<(), String> {
        let mut config = ConfigHandler {
            config_id: opts.config_id.clone(),
            generator: rume
                .rume_config
                .as_ref()
                .map_or("".to_string(), |c| c.app_name.clone()),
            current_time: opts.current_time,
            content: None,
        };

        config.load()?;
        config.customize_item(&opts.key, &opts.yaml_value)?;
        config.save()?;

        Ok(())
    }
}

impl ConfigHandler {
    fn load(&mut self) -> Result<(), String> {
        let file_name = self.get_file_name();

        let mut file_contents: Vec<Yaml> = vec![Yaml::Hash(LinkedHashMap::new())];

        if let Ok(content) = std::fs::read_to_string(&file_name) {
            file_contents =
                YamlLoader::load_from_str(&content).expect("Failed to parse existing YAML content");
        }

        let file_content = match file_contents.pop() {
            Some(doc) => doc,
            _ => Yaml::Hash(LinkedHashMap::new()),
        };

        self.content = Some(file_content);

        Ok(())
    }

    fn customize_item(&mut self, key: &str, yaml_value: &str) -> Result<(), String> {
        let file_content = self
            .content
            .clone()
            .unwrap_or(Yaml::Hash(LinkedHashMap::new()));
        let mut file_hash = match file_content {
            Yaml::Hash(hash) => hash,
            _ => {
                return Err("File content is not a valid YAML hash".to_string());
            }
        };

        let customization_key = Yaml::String("customization".to_string());

        if file_hash.get(&customization_key).is_none() {
            let mut customization_hash = LinkedHashMap::new();
            customization_hash.insert(
                Yaml::String("distribution_code_name".to_string()),
                Yaml::String("".to_string()),
            );
            customization_hash.insert(
                Yaml::String("distribution_version".to_string()),
                Yaml::String("".to_string()),
            );
            customization_hash.insert(
                Yaml::String("generator".to_string()),
                Yaml::String(self.generator.to_string()),
            );
            let modified_time_format = "%a %b %d %H:%M:%S %Y";
            let modified_time = self.current_time.map_or_else(
                || Local::now().format(modified_time_format).to_string(),
                |time| time.format(modified_time_format).to_string(),
            );
            customization_hash.insert(
                Yaml::String("modified_time".to_string()),
                Yaml::String(modified_time),
            );
            customization_hash.insert(
                Yaml::String("rume_version".to_string()),
                Yaml::String(RUME_VERSION.to_string()),
            );
            file_hash.insert(customization_key, Yaml::Hash(customization_hash));
        }

        let yaml_value = match YamlLoader::load_from_str(yaml_value) {
            Ok(value) => value[0].clone(),
            Err(_) => {
                // If parsing fails, treat the input as a simple string
                Yaml::String(yaml_value.to_string())
            }
        };

        file_hash.insert(
            Yaml::String("patch".to_string()),
            Yaml::Hash(LinkedHashMap::from_iter(vec![(
                Yaml::String(key.to_string()),
                yaml_value,
            )])),
        );

        self.content = Some(Yaml::Hash(file_hash));

        Ok(())
    }

    fn get_file_name(&self) -> String {
        if self.config_id.ends_with(".yaml") {
            return self.config_id.clone();
        }

        format!("{}.custom.yaml", self.config_id)
    }

    fn save(&self) -> Result<(), String> {
        let file_name = self.get_file_name();
        let content = self.content.clone().ok_or("No content to save")?;
        let mut file_result = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut file_result);
            emitter
                .dump(&content)
                .map_err(|e| format!("Failed to emit YAML: {}", e))?;
        }

        if file_result.starts_with("---\n") {
            file_result = file_result[4..].to_string();
        }

        std::fs::write(file_name, file_result)
            .map_err(|e| format!("Failed to write patch file: {}", e))?;

        Ok(())
    }
}
