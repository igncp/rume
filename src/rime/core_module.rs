use super::{
    config::config_component::{ConfigBuilder, ConfigLoader},
    module::ModuleManager,
};
use crate::{
    rime::{
        config::{
            config_component::{
                ConfigLoaderStruct, ConfigSchema, DeployedConfigComponent, UserConfigComponent,
            },
            plugins::{
                AutoPatchConfigPlugin, BuildInfoPlugin, DefaultConfigPlugin,
                LegacyDictionaryConfigPlugin, LegacyPresetConfigPlugin, SaveOutputPlugin,
            },
        },
        registry::{Registry, RegistryValue},
    },
    rime_api::RimeModule,
};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tracing::info;

impl ConfigLoader for ConfigBuilder {}
impl ConfigLoader for ConfigLoaderStruct {}

lazy_static! {
    pub static ref CORE_MODULE: Mutex<RimeModule> = Mutex::new(RimeModule {
        module_name: "core".to_string(),
        initialize: Some(Box::new(|| {
            info!("registering core components.");
            let get_r = || {
                Registry::instance()
                    .lock()
                    .expect("Failed to lock ModuleManager")
            };

            {
                let config_builder = ConfigBuilder {
                    init: |builder: &mut ConfigBuilder| {
                        builder.install_plugin(AutoPatchConfigPlugin);
                        builder.install_plugin(DefaultConfigPlugin);
                        builder.install_plugin(LegacyPresetConfigPlugin);
                        builder.install_plugin(LegacyDictionaryConfigPlugin);
                        builder.install_plugin(BuildInfoPlugin);
                        builder.install_plugin(SaveOutputPlugin);
                    },
                };

                let mut r = get_r();
                r.register(
                    "config_builder",
                    RegistryValue::ConfigBuilder(config_builder),
                );
            }

            {
                let deployed_config_component = Arc::new(Mutex::new(DeployedConfigComponent));
                let schema_config = ConfigSchema(deployed_config_component.clone());

                let mut r = get_r();

                r.register(
                    "config",
                    RegistryValue::DeployedConfigComponent(deployed_config_component),
                );
                r.register("schema", RegistryValue::SchemaComponent(schema_config));
            }

            {
                let user_config = UserConfigComponent {
                    init: |loader: &mut ConfigLoaderStruct| {
                        loader.set_auto_save(true);
                    },
                };
                let mut r = get_r();

                r.register(
                    "user_config",
                    RegistryValue::UserConfigComponent(user_config),
                );
            }
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
