pub struct ConfigItem;
pub struct ConfigItemRef;
pub struct ConfigList;
pub struct ConfigMap;

#[derive(Clone)]
pub enum ConfigValue {
    String(String),
}
