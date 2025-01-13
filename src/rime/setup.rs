use super::{deployer::register_deployer, module::ModuleManager, service::Service};
use crate::rime_api::RimeTraits;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub const DEPLOYER_MODULES: &[&str] = &["deployer"];

pub fn load_modules(module_names: &[&str]) {
    for module in module_names {
        if let Some(initialize_fn) = {
            let mut mm = ModuleManager::instance()
                .lock()
                .expect("Failed to lock ModuleManager");
            mm.load_module(module)
        } {
            initialize_fn();
        }
    }
}

pub fn setup_deployer(traits: &Option<RimeTraits>) {
    if traits.is_none() {
        return;
    }

    let traits = traits.as_ref().unwrap();

    // Register modules here, insted of using macros
    register_deployer();

    let mut service = Service::instance().lock().unwrap();
    let deployer = service.deployer_mut();

    if traits.shared_data_dir.is_some() {
        deployer.shared_data_dir = traits.shared_data_dir.clone().unwrap().into();
    }

    if traits.user_data_dir.is_some() {
        deployer.user_data_dir = traits.user_data_dir.clone().unwrap().into();
    }

    if !traits.distribution_name.is_empty() {
        deployer.distribution_code_name = traits.distribution_name.to_string().clone();
    }

    // if (RIME_PROVIDED(traits, distribution_code_name))
    //   deployer.distribution_code_name = traits->distribution_code_name;
    // if (RIME_PROVIDED(traits, distribution_version))
    //   deployer.distribution_version = traits->distribution_version;
    // if (RIME_PROVIDED(traits, app_name))
    //   deployer.app_name = traits->app_name;
    // if (RIME_PROVIDED(traits, prebuilt_data_dir))
    //   deployer.prebuilt_data_dir = path(traits->prebuilt_data_dir);
    // else
    deployer.prebuilt_data_dir = deployer.shared_data_dir.join("build");

    if traits.staging_dir.is_some() {
        deployer.staging_dir = traits.staging_dir.clone().unwrap().into();
    } else {
        deployer.staging_dir = deployer.shared_data_dir.join("build");
    }
}

pub fn setup_logging(traits_opt: &Option<RimeTraits>) {
    if traits_opt.is_none() {
        return;
    }

    let traits = traits_opt.as_ref().unwrap();

    if traits.log_dir.is_none() || traits.min_log_level.is_none() {
        return;
    }

    let log_dir = traits.log_dir.unwrap();
    let min_log_level = traits.min_log_level.unwrap();

    let logger_level = match min_log_level {
        4 => Level::TRACE,
        3 => Level::DEBUG,
        2 => Level::INFO,
        1 => Level::WARN,
        0 => Level::ERROR,
        _ => Level::WARN,
    };

    if log_dir.is_empty() {
        // TODO: log to stderr
        let subscriber = FmtSubscriber::builder()
            .compact()
            .with_max_level(logger_level)
            .with_ansi(false)
            .finish();

        tracing::subscriber::set_global_default(subscriber).unwrap();
    } else {
        //   FLAGS_logfile_mode = 0600;
        // TODO: log to specific file
    }
}
