use std::fs;
use std::path::Path;
use tracing::{info, Level};
use tracing_subscriber::fmt::writer::{BoxMakeWriter, MakeWriterExt};
use tracing_subscriber::FmtSubscriber;

pub const ENV_LOGGER_LEVEL: &str = "RUME_LOGGER_LEVEL";

pub fn setup_logs(log_dir: Option<String>) {
    let logger_level = std::env::var(ENV_LOGGER_LEVEL).unwrap_or("".to_string());

    let logger_level = match logger_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "warn" => Level::WARN,
        "info" => Level::INFO,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let builder = FmtSubscriber::builder()
        .compact()
        .with_max_level(logger_level)
        .with_ansi(false);

    if let Some(dir) = log_dir {
        let dir_path = Path::new(&dir);
        let _ = fs::create_dir_all(dir_path);

        let file_path = dir_path.join("rume.log");
        let file_writer = BoxMakeWriter::new(move || {
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file_path)
                .unwrap()
        });

        let stdout_writer = BoxMakeWriter::new(std::io::stdout);
        let combined_writer = stdout_writer.and(file_writer);
        let subscriber = builder.with_writer(combined_writer).finish();

        tracing::subscriber::set_global_default(subscriber).unwrap();
    } else {
        let subscriber = builder.finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();
    }

    info!("---");
    info!("Logger initialized with level: {:?}", logger_level);
}
