use std::fmt::{Display, Formatter};

use super::rime::setup::{setup_deployer, setup_logging};
use crate::rime::setup::{load_modules, DEPLOYER_MODULES};
use lazy_static::lazy_static;

#[derive(Default)]
pub struct RimeApi;

#[derive(Default)]
pub struct RimeTraits {
    pub app_name: &'static str,
    pub distribution_name: &'static str,
    pub min_log_level: Option<i32>,
    pub modules: Option<&'static [&'static str]>,

    pub log_dir: Option<&'static str>,
    pub shared_data_dir: Option<&'static str>,
    pub staging_dir: Option<&'static str>,
    pub user_data_dir: Option<&'static str>,
}

pub struct RimeConfig;
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct RimeModule {
    pub initialize: Option<Box<fn()>>,
    pub module_name: String,
}

impl Display for RimeModule {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "RimeModule")
    }
}

impl RimeApi {
    pub fn setup(&self, traits: &Option<RimeTraits>) {
        // TODO: This differentiates between shared/static library build
        // >> rime_declare_module_dependencies();

        setup_deployer(traits);
        setup_logging(traits);
    }
    pub fn get_module(&self, module_name: &str) -> Option<RimeModule> {
        // TODO
        Some(RimeModule {
            initialize: None,
            module_name: module_name.to_string(),
        })
    }
    pub fn config_load_string(&self, config: &mut RimeConfig, yaml: String) -> bool {
        true
    }
    pub fn config_close(&self, config: &RimeConfig) {}
    pub fn finalize(&self) {}
    pub fn deployer_initialize(&self, traits: &Option<RimeTraits>) {
        setup_deployer(traits);

        if traits.is_none() {
            return;
        }

        let traits = traits.as_ref().unwrap();

        if traits.modules.is_some() {
            let module_names = traits.modules.unwrap();
            load_modules(module_names);
        } else {
            load_modules(DEPLOYER_MODULES);
        }
    }
}

impl RimeModule {
    pub fn get_api<A>(&self) -> A
    where
        A: From<&'static str>,
    {
        return A::from("foo");
    }
}

impl RimeConfig {
    pub fn new() -> Self {
        Self
    }
}

lazy_static! {
    static ref RIME_API: RimeApi = RimeApi::default();
}

pub fn get_rime_api() -> &'static RimeApi {
    &RIME_API
}
