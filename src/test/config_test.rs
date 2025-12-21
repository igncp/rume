use crate::rime::{
    config::config_component::{
        Config, ConfigComponent, ConfigInit, ConfigLoaderStruct, UserConfigResourceProvider,
    },
    registry::Registry,
};

#[test]
fn test_round_trip() {
    let mut registry = Registry::instance().lock().unwrap();
    let component = ConfigComponent {
        loader: ConfigLoaderStruct,
        resource_provider: UserConfigResourceProvider,
        init: ConfigInit::InitFn(|loader: &mut ConfigLoaderStruct| {
            loader.set_auto_save(true);
        }),
    };

    registry.register("test_config", component);

    let extracted = Config::require(&registry, "test_config");

    assert_eq!(extracted.is_some(), true);

    registry.unregister("test_config");
}
