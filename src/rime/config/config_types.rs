use std::collections::HashMap;

pub struct ConfigItemRef;
pub struct ConfigList;
pub struct ConfigMap;

#[derive(Clone, Debug)]
pub enum ConfigValue {
    String(String),
}

type ConfigKey = String;

#[derive(Clone, Debug)]
pub enum ConfigItem {
    List(Box<Vec<ConfigItem>>),
    Map(Box<HashMap<ConfigKey, ConfigItem>>),
    Value(Box<ConfigValue>),
}

impl ConfigItem {
    pub fn is_container(&self) -> bool {
        return match self {
            ConfigItem::List(_) | ConfigItem::Map(_) => true,
            ConfigItem::Value(_) => false,
        };
    }

    pub fn get_item(&self, key: &str) -> Option<&Self> {
        return match self {
            ConfigItem::Value(_) => unreachable!("Can't get item from value"),
            ConfigItem::Map(v) => v.get(key),
            ConfigItem::List(v) => {
                let idx = key.parse::<usize>().unwrap();
                v.get(idx)
            }
        };
    }

    pub fn get_item_mut(&mut self, key: &str) -> Option<&mut Self> {
        return match self {
            ConfigItem::Value(_) => unreachable!("Can't get item from value"),
            ConfigItem::Map(v) => v.get_mut(key),
            ConfigItem::List(v) => {
                let idx = key.parse::<usize>().unwrap();
                v.get_mut(idx)
            }
        };
    }

    pub fn set_item(&mut self, key: &str, val: ConfigItem) -> Result<(), String> {
        return match self {
            ConfigItem::Value(_) => unreachable!("Can't get item from value"),
            ConfigItem::Map(v) => {
                v.insert(key.to_string(), val);
                Ok(())
            }
            ConfigItem::List(v) => {
                let idx = key.parse::<usize>().unwrap();
                v[idx] = val;
                Ok(())
            }
        };
    }
}
