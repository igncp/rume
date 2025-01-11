use super::{
    core_module::register_core, dict::dictionary::register_dict,
    lever::levers_module::register_levers, module::ModuleManager,
};
use crate::rime_api::RimeModule;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Deployer {
    pub prebuilt_data_dir: std::path::PathBuf,
    pub shared_data_dir: std::path::PathBuf,
    pub staging_dir: std::path::PathBuf,
}

lazy_static! {
    pub static ref DEPLOYER_MODULE: Mutex<RimeModule> = Mutex::new(RimeModule {
        module_name: "deployer".to_string(),
        initialize: Some(Box::new(|| {
            let modules_to_load = ["core", "dict", "levers"];

            for module_name in modules_to_load.iter() {
                if let Some(initialize_fn) = {
                    let mut mm = crate::rime::module::ModuleManager::instance()
                        .lock()
                        .expect("Failed to lock ModuleManager");

                    mm.load_module(module_name)
                } {
                    initialize_fn();
                }
            }
        }))
    });
}

pub fn register_deployer() {
    register_core();
    register_dict();
    register_levers();

    let mut mm = ModuleManager::instance()
        .lock()
        .expect("Failed to lock ModuleManager");

    let module = DEPLOYER_MODULE.lock().unwrap();
    let module_ref = Arc::new(module.clone());

    mm.register(&module_ref);
}
