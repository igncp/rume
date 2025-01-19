use std::sync::{Arc, Mutex};

use crate::rime::registry::{Registry, RegistryValue};

use super::{config_data::ConfigData, config_types::ConfigValue, plugins::ConfigCompilerPlugin};

#[derive(Default)]
pub struct Config {
    data: ConfigData,
}

impl Config {
    pub(crate) fn load_from_file(&mut self) -> bool {
        self.data.load_from_file()
    }

    pub(crate) fn require<'a>(registry: &'a Registry, name: &str) -> Option<&'a RegistryValue> {
        registry.find(name)
    }

    pub(crate) fn set_string(&mut self, path: &str, value: &str) -> Result<(), String> {
        let config_value = ConfigValue::String(value.to_string());

        return self.data.traverse_write(path, config_value);
    }

    pub(crate) fn get_string(&self, path: &str) -> Result<Option<String>, String> {
        return match self.data.traverse(path)? {
            Some(v) => match v {
                ConfigValue::String(v2) => Ok(Some(v2)),
                _ => Err("Invalid data type".to_string()),
            },
            _ => Ok(None),
        };
    }
}

pub trait ConfigLoader {}
pub trait ConfigResourceProvider {}

pub enum ConfigInit<A> {
    InitFn(A),
    InitDefaultType,
}

pub struct ConfigComponent<A, B, C>
where
    A: ConfigResourceProvider,
    B: ConfigLoader,
    C: Fn(&mut B),
{
    pub resource_provider: A,
    pub loader: B,
    pub init: ConfigInit<C>,
}

pub struct DefaultConfigResourceProvider;
pub struct DeployedConfigResourceProvider;
pub struct UserConfigResourceProvider;

impl ConfigResourceProvider for DefaultConfigResourceProvider {}
impl ConfigResourceProvider for DeployedConfigResourceProvider {}
impl ConfigResourceProvider for UserConfigResourceProvider {}

pub struct ConfigLoaderStruct;

// Equivalent to ConfigLoader class
impl ConfigLoaderStruct {
    pub fn set_auto_save(&mut self, auto_save: bool) {}
}

pub struct ConfigBuilder;
pub struct DeployedConfigComponent;
pub struct UserConfigComponent;
pub struct ConfigSchema(pub Arc<Mutex<DeployedConfigComponent>>);

impl ConfigBuilder {
    pub fn install_plugin<T: ConfigCompilerPlugin>(&mut self, _p: T) {}
}

impl UserConfigComponent {
    pub fn create(&self, name: &str) -> Result<Config, String> {
        return Ok(Config { data: ConfigData });
    }
}
