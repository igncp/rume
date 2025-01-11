// usage:
//   rime_patch config_id key [yaml]
// example:
//   rime_patch default "menu/page_size" 9
//   rime_patch default schema_list/@0/schema combo_pinyin
//   rime_patch default schema_list '[{schema: luna_pinyin}]'
//   rime_patch default schema_list  # read yaml from stdin

use rime::{
    rime_api::{get_rime_api, RimeConfig, RimeTraits},
    rime_levers_api::RimeLeversApi,
};
use std::{env, io::Read};

fn apply_patch(config_id: &str, key: &str, yaml: String) -> i32 {
    let rime = get_rime_api();

    let module = rime.get_module("levers");

    if module.is_none() {
        eprintln!("missing Rime module: levers");
        return 1;
    }

    let mut config = RimeConfig::new();

    let mut ret = 1;
    if rime.config_load_string(&mut config, yaml) {
        let mut settings = RimeLeversApi::custom_settings_init(config_id, "rime_patch");

        RimeLeversApi::load_settings(&mut settings);

        if RimeLeversApi::customize_item(&settings, key, &config) {
            RimeLeversApi::save_settings(&settings);
            eprintln!("patch applied.");
            ret = 0;
        }

        RimeLeversApi::custom_settings_destroy(settings);
        rime.config_close(&config);
    } else {
        eprintln!("bad yaml document.");
    }

    return ret;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        eprintln!("usage: {} config_id key [yaml]", args[0]);
        std::process::exit(1);
    }

    {
        let rime = get_rime_api();

        let traits = Some(RimeTraits {
            app_name: "rime.patch",
            min_log_level: Some(3),
            log_dir: Some(""),
            ..Default::default()
        });

        rime.setup(&traits);
        rime.deployer_initialize(&traits);
    }

    let config_id = &args[1];
    let key = &args[2];

    let yaml = if args.len() > 3 {
        args[3].clone()
    } else {
        let mut new_yaml = String::new();
        std::io::stdin().read_to_string(&mut new_yaml).unwrap();
        new_yaml
    };

    let ret_value = apply_patch(config_id, key, yaml);

    {
        let rime = get_rime_api();
        rime.finalize();
    }

    std::process::exit(ret_value);
}
