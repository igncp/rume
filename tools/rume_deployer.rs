use std::{env, io::Read};

use rume::rume::{
    config_handler::{ApplyPatchOpts, ConfigHandler},
    NewRumeConfig, Rume,
};

pub enum RumeDeployAction {
    SetActiveSchema { schema_id: String },
}

pub fn rume_deploy(action: RumeDeployAction) -> Result<(), String> {
    let mut rume = Rume::new(Some(NewRumeConfig {
        app_name: "rume_deployer".to_string(),
        log_dir: None,
        min_log_level: Some(3),
    }));

    rume.init()?;

    match action {
        RumeDeployAction::SetActiveSchema { schema_id } => {
            ConfigHandler::apply_patch(
                &rume,
                &ApplyPatchOpts {
                    config_id: "user.yaml".to_string(),
                    key: "var/previously_selected_schema".to_string(),
                    yaml_value: schema_id,
                    ..Default::default()
                },
            )?;

            Ok(())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("usage: {} <action>", args[0]);
        eprintln!("actions:");
        eprintln!("  set_active_schema [<schema_id>]");
        eprintln!("    Sets the active schema to <schema_id>. If <schema_id> is not provided, reads from stdin.");
        std::process::exit(1);
    }

    let action = match args[1].as_str() {
        "set_active_schema" => {
            let schema_id = if args.len() > 2 {
                args[2].clone()
            } else {
                let mut new_yaml = String::new();
                std::io::stdin().read_to_string(&mut new_yaml).unwrap();
                new_yaml
            };
            RumeDeployAction::SetActiveSchema { schema_id }
        }
        _ => {
            eprintln!("Unknown action: {}", args[1]);
            std::process::exit(1);
        }
    };

    if let Err(e) = rume_deploy(action) {
        eprintln!("Error during deployment: {}", e);
        std::process::exit(1);
    }
}
