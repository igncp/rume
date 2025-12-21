use super::{config_data::ConfigData, plugins::ConfigCompilerPlugin};
use crate::rime::{component::ComponentBase, registry::Registry};

#[derive(Default)]
pub struct Config {
    data: ConfigData,
}

impl Config {
    pub(crate) fn load_from_file(&mut self) -> bool {
        self.data.load_from_file()
    }

    pub(crate) fn require<'a>(
        registry: &'a Registry,
        name: &str,
    ) -> Option<&'a Box<dyn ComponentBase>> {
        registry.find(name)
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

impl ConfigBuilder {
    pub fn install_plugin<T: ConfigCompilerPlugin>(&mut self, _p: T) {}
}
