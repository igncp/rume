use std::fs;
use std::io::{self, Write};
use std::path::Path;
use tracing::{info, Level};
use tracing_subscriber::fmt::writer::{BoxMakeWriter, MakeWriterExt};
use tracing_subscriber::FmtSubscriber;

/// cbindgen:ignore
pub const ENV_LOGGER_LEVEL: &str = "RUME_LOGGER_LEVEL";

pub fn setup_logs(app_name: &str, log_dir: Option<String>, stdout_log: bool) {
    // Auto-disable logs under tests: unit (cfg(test)) and integration (env set by test harness).
    if cfg!(test) || std::env::var("RUST_TEST_THREADS").is_ok() {
        return;
    }

    let logger_level_str = std::env::var(ENV_LOGGER_LEVEL).unwrap_or("".to_string());

    // Allow disabling logs entirely, useful for `cargo test`.
    // Usage: `RUME_LOGGER_LEVEL=off cargo test`
    if matches!(logger_level_str.as_str(), "off" | "none" | "silent") {
        return;
    }

    let logger_level = match logger_level_str.as_str() {
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

    struct PrefixingWriter<W: Write> {
        inner: W,
        prefix: String,
        wrote_prefix: bool,
    }

    impl<W: Write> Write for PrefixingWriter<W> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if !self.wrote_prefix {
                self.inner.write_all(self.prefix.as_bytes())?;
                self.wrote_prefix = true;
            }
            self.inner.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.inner.flush()
        }

        fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
            if !self.wrote_prefix {
                self.inner.write_all(self.prefix.as_bytes())?;
                self.wrote_prefix = true;
            }
            self.inner.write_all(buf)
        }
    }

    let prefix = format!("[{}] ", app_name);

    if let Some(dir) = log_dir {
        let dir_path = Path::new(&dir);
        let _ = fs::create_dir_all(dir_path);

        let file_path = dir_path.join("rume.log");
        let file_prefix = prefix.clone();
        let file_writer = BoxMakeWriter::new(move || {
            let f = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file_path)
                .unwrap();
            PrefixingWriter {
                inner: f,
                prefix: file_prefix.clone(),
                wrote_prefix: false,
            }
        });

        let stdout_prefix = prefix.clone();
        let stdout_writer = BoxMakeWriter::new(move || PrefixingWriter {
            inner: std::io::stdout(),
            prefix: stdout_prefix.clone(),
            wrote_prefix: false,
        });

        if stdout_log {
            let combined_writer = stdout_writer.and(file_writer);
            let subscriber = builder.with_writer(combined_writer).finish();

            tracing::subscriber::set_global_default(subscriber).unwrap();
        } else {
            let subscriber = builder.with_writer(file_writer).finish();

            tracing::subscriber::set_global_default(subscriber).unwrap();
        }
    } else if stdout_log {
        let stdout_prefix = prefix.clone();
        let subscriber = builder
            .with_writer(BoxMakeWriter::new(move || PrefixingWriter {
                inner: std::io::stdout(),
                prefix: stdout_prefix.clone(),
                wrote_prefix: false,
            }))
            .finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();
    }

    info!("---");
    info!("Logger initialized with level: {:?}", logger_level);
}
