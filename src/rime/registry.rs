use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};
use tracing::info;

use super::component::ComponentBase;

#[derive(Default)]
pub struct Registry {
    map_: HashMap<String, Box<dyn ComponentBase>>,
}

lazy_static! {
    pub static ref REGISTRY: Mutex<Box<Registry>> = Mutex::new(Box::new(Registry::default()));
}

impl Registry {
    pub fn instance() -> &'static REGISTRY {
        &REGISTRY
    }

    pub fn register<A>(&mut self, name: &str, value: A)
    where
        A: ComponentBase + 'static,
    {
        info!("Registering {}", name);

        let boxed = Box::new(value);
        self.map_.insert(name.to_string(), boxed);
    }
}
