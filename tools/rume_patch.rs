use std::{env, io::Read};

use rume::rume::{NewRumeConfig, Rume};

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

    let mut rume = Rume::new(Some(NewRumeConfig {
        app_name: "rime.patch".to_string(),
        min_log_level: Some(3),
    }));

    rume.init().unwrap_or_else(|err| {
        eprint!("Failed to initialize Rume: {}", err);
        std::process::exit(1);
    });

    let config_id = &args[1];
    let key = &args[2];

    let yaml = if args.len() > 3 {
        args[3].clone()
    } else {
        let mut new_yaml = String::new();
        std::io::stdin().read_to_string(&mut new_yaml).unwrap();
        new_yaml
    };

    rume.apply_patch(config_id, key, &yaml)
        .unwrap_or_else(|err| {
            eprint!("Failed to apply patch: {}", err);
            std::process::exit(1);
        });
}
