use std::{env, io::Read};

use rume::rume::{
    config_handler::{ApplyPatchOpts, ConfigCurrentTime, ConfigHandler},
    Rume, RumeNewConfig,
};

mod test_rume_patch;

pub fn rume_patch(
    config_id: &str,
    key: &str,
    yaml: String,
    current_time: ConfigCurrentTime,
) -> Result<(), String> {
    let mut rume = Rume::new(Some(RumeNewConfig {
        app_name: "rume_patch".to_string(),
        // Suppress logs during tests without needing env vars.
        stdout_log: !cfg!(test),
        ..Default::default()
    }));

    rume.init()?;

    ConfigHandler::apply_patch(
        &rume,
        &ApplyPatchOpts {
            config_id: config_id.to_string(),
            key: key.to_string(),
            yaml_value: yaml,
            current_time,
        },
    )?;

    Ok(())
}

// usage:
//   rime_patch config_id key [yaml]
// example:
//   rime_patch default "menu/page_size" 9
//   rime_patch default schema_list/@0/schema combo_pinyin
//   rime_patch default schema_list '[{schema: luna_pinyin}]'
//   rime_patch default schema_list  # read yaml from stdin

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        eprintln!("usage: {} config_id key [yaml]", args[0]);
        std::process::exit(1);
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

    rume_patch(config_id, key, yaml, None).unwrap_or_else(|err| {
        eprint!("Unexpected error {err}");
        std::process::exit(1);
    })
}
