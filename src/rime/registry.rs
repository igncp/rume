use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tracing::info;

use super::config::config_component::{
    ConfigBuilder, ConfigSchema, DeployedConfigComponent, UserConfigComponent,
};

#[derive(Default)]
pub struct Registry {
    map_: HashMap<String, RegistryValue>,
}

lazy_static! {
    pub static ref REGISTRY: Mutex<Box<Registry>> = Mutex::new(Box::new(Registry::default()));
}

pub enum RegistryValue {
    ConfigBuilder(ConfigBuilder),
    DeployedConfigComponent(Arc<Mutex<DeployedConfigComponent>>),
    SchemaComponent(ConfigSchema),
    UserConfigComponent(UserConfigComponent),
}

impl Registry {
    pub fn instance() -> &'static REGISTRY {
        &REGISTRY
    }

    pub fn register(&mut self, name: &str, value: RegistryValue) {
        info!("Registering {}", name);

        self.map_.insert(name.to_string(), value);
    }

    pub fn unregister(&mut self, name: &str) {
        info!("Unregistering {}", name);

        self.map_.remove(name);
    }

    pub fn find(&self, name: &str) -> Option<&RegistryValue> {
        self.map_.get(name)
    }
}
