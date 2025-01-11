use crate::{rime::module::ModuleManager, rime_api::RimeModule};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref DICT_MODULE: Mutex<RimeModule> = Mutex::new(RimeModule {
        module_name: "dict".to_string(),
        initialize: Some(Box::new(|| {
            // @TODO
        }))
    });
}

pub fn register_dict() {
    let mut mm = ModuleManager::instance()
        .lock()
        .expect("Failed to lock ModuleManager");

    let module = DICT_MODULE.lock().unwrap();
    let module_ref = Arc::new(module.clone());

    mm.register(&module_ref);
}
