use crate::rime_api::RimeModule;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use tracing::info;

#[derive(Default)]
pub struct ModuleManager {
    loaded_: HashSet<Arc<RimeModule>>,
    map_: HashMap<String, Arc<RimeModule>>,
}

lazy_static! {
    pub static ref MM: Mutex<Box<ModuleManager>> = Mutex::new(Box::new(ModuleManager::default()));
}

impl ModuleManager {
    pub fn instance() -> &'static MM {
        &MM
    }

    pub fn find(&self, module_name: &str) -> Option<&Arc<RimeModule>> {
        return self.map_.get(module_name);
    }

    pub fn register(&mut self, module: &Arc<RimeModule>) {
        self.map_.insert(module.module_name.clone(), module.clone());
    }

    pub fn load_module(&mut self, module_name: &str) -> Option<impl FnOnce()> {
        let already_loaded = self.loaded_.iter().any(|m| m.module_name == module_name);

        if already_loaded {
            return None;
        }

        info!("loading module: {}", module_name);

        let module = self
            .find(module_name)
            .expect(&format!("Error loading module {module_name}"))
            .clone();

        self.loaded_.insert(module.clone());

        if module.initialize.is_none() {
            info!("missing initialize() function in module: {}", module);
            return None;
        }

        let initialize_fn = module.initialize.as_ref().unwrap().clone();

        return Some(initialize_fn);
    }
}
