use crate::rime::{
    config::config_component::{Config, ConfigLoaderStruct, UserConfigComponent},
    registry::{Registry, RegistryValue},
};

#[test]
fn test_round_trip() {
    let mut registry = Registry::instance().lock().unwrap();

    let component = UserConfigComponent {
        init: |loader: &mut ConfigLoaderStruct| {
            loader.set_auto_save(true);
        },
    };

    registry.register("test_config", RegistryValue::UserConfigComponent(component));

    let extracted = match Config::require(&registry, "test_config").unwrap() {
        RegistryValue::UserConfigComponent(v) => v,
        _ => panic!("Unexpected value"),
    };

    {
        let mut config = extracted.create("config_round_trip_test").unwrap();

        assert!(config.set_string("key", "value").is_ok())
    }

    {
        let config = extracted.create("config_round_trip_test").unwrap();
        let value = config.get_string("key").unwrap();

        assert_eq!(value, Some("value".to_string()));
    }

    registry.unregister("test_config");
}
