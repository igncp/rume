use super::{
    component::ComponentBase,
    config::config_component::{
        ConfigBuilder, ConfigLoader, DefaultConfigResourceProvider, DeployedConfigResourceProvider,
        UserConfigResourceProvider,
    },
    module::ModuleManager,
    schema::SchemaComponent,
};
use crate::{
    rime::{
        config::{
            config_component::{ConfigComponent, ConfigInit, ConfigLoaderStruct},
            plugins::{
                AutoPatchConfigPlugin, BuildInfoPlugin, DefaultConfigPlugin,
                LegacyDictionaryConfigPlugin, LegacyPresetConfigPlugin, SaveOutputPlugin,
            },
        },
        registry::Registry,
    },
    rime_api::RimeModule,
};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tracing::info;

impl ConfigLoader for ConfigBuilder {}
impl ConfigLoader for ConfigLoaderStruct {}

impl<A> ComponentBase for ConfigComponent<DefaultConfigResourceProvider, ConfigBuilder, A> where
    A: Fn(&mut ConfigBuilder) + Send
{
}

impl<A> ComponentBase for ConfigComponent<UserConfigResourceProvider, ConfigLoaderStruct, A> where
    A: Fn(&mut ConfigLoaderStruct) + Send
{
}

impl ComponentBase
    for ConfigComponent<
        DeployedConfigResourceProvider,
        ConfigLoaderStruct,
        fn(&mut ConfigLoaderStruct),
    >
{
}

impl ComponentBase
    for SchemaComponent<
        ConfigComponent<
            DeployedConfigResourceProvider,
            ConfigLoaderStruct,
            fn(&mut ConfigLoaderStruct),
        >,
    >
{
}

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
                let config_builder: ConfigComponent<
                    DefaultConfigResourceProvider,
                    ConfigBuilder,
                    _,
                > = ConfigComponent {
                    loader: ConfigBuilder,
                    resource_provider: DefaultConfigResourceProvider,
                    init: ConfigInit::InitFn(|builder: &mut ConfigBuilder| {
                        builder.install_plugin(AutoPatchConfigPlugin);
                        builder.install_plugin(DefaultConfigPlugin);
                        builder.install_plugin(LegacyPresetConfigPlugin);
                        builder.install_plugin(LegacyDictionaryConfigPlugin);
                        builder.install_plugin(BuildInfoPlugin);
                        builder.install_plugin(SaveOutputPlugin);
                    }),
                };

                let mut r = get_r();
                r.register("config_builder", config_builder);
            }

            {
                let create_config_loader = || ConfigComponent {
                    loader: ConfigLoaderStruct,
                    resource_provider: DeployedConfigResourceProvider,
                    init: ConfigInit::InitDefaultType,
                };

                let mut r = get_r();

                r.register("config", create_config_loader());
                r.register("schema", SchemaComponent(create_config_loader()));
            }

            {
                let user_config: ConfigComponent<
                    UserConfigResourceProvider,
                    ConfigLoaderStruct,
                    _,
                > = ConfigComponent {
                    loader: ConfigLoaderStruct,
                    resource_provider: UserConfigResourceProvider,
                    init: ConfigInit::InitFn(|loader: &mut ConfigLoaderStruct| {
                        loader.set_auto_save(true);
                    }),
                };

                let mut r = get_r();

                r.register("user_config", user_config);
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
