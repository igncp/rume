use super::module::ModuleManager;
use crate::{
    rime::{config::config_component::ConfigComponent, registry::Registry},
    rime_api::RimeModule,
};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tracing::info;

lazy_static! {
    pub static ref CORE_MODULE: Mutex<RimeModule> = Mutex::new(RimeModule {
        module_name: "core".to_string(),
        initialize: Some(Box::new(|| {
            info!("registering core components.");

          // @TODO
          // auto config_builder =
          //     new ConfigComponent<ConfigBuilder>([&](ConfigBuilder* builder) {
          //       builder->InstallPlugin(new AutoPatchConfigPlugin);
          //       builder->InstallPlugin(new DefaultConfigPlugin);
          //       builder->InstallPlugin(new LegacyPresetConfigPlugin);
          //       builder->InstallPlugin(new LegacyDictionaryConfigPlugin);
          //       builder->InstallPlugin(new BuildInfoPlugin);
          //       builder->InstallPlugin(new SaveOutputPlugin);
          //     });
            {
                let config_builder = ConfigComponent;

                let mut r = Registry::instance()
                    .lock()
                    .expect("Failed to lock ModuleManager");

                r.register("config_builder", config_builder);
            }

          // auto config_loader =
          //     new ConfigComponent<ConfigLoader, DeployedConfigResourceProvider>;
          // r.Register("config", config_loader);
          // r.Register("schema", new SchemaComponent(config_loader));

          // auto user_config =
          //     new ConfigComponent<ConfigLoader, UserConfigResourceProvider>(
          //         [](ConfigLoader* loader) { loader->set_auto_save(true); });
          // r.Register("user_config", user_config);
        }))
    });
}

pub fn register_core() {
    let mut mm = ModuleManager::instance()
        .lock()
        .expect("Failed to lock ModuleManager");

    let module = CORE_MODULE.lock().unwrap();
    let module_ref = Arc::new(module.clone());

    mm.register(&module_ref);
}
