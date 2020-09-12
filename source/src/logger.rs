use crate::config::{Config};
use shaku::{Component, Interface};
use slog::{Drain, Logger, PushFnValue};
use std::fs::OpenOptions;
use std::sync::{Mutex};

pub trait AppLoggerIf: Interface {
    fn logger(&self) -> &Logger;
}

#[derive(Component)]
#[shaku(interface = AppLoggerIf)]
pub struct AppLogger {
    #[shaku(no_default)]
    logger: Logger,
}

impl AppLoggerIf for AppLogger {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

pub trait WithLogger {
    fn logger(&self) -> &Logger;
}

pub fn build_app_logger(config: &Config) -> Logger {
    build_json_file_logger(
        config.clear_logger_files,
        &config.app_logger_file,
        "app_logger".to_string(),
        config.loggers_json_pretty,
    )
}

fn build_json_file_logger(
    truncate: bool,
    file_name: &str,
    logger_name: String,
    pretty: bool,
) -> Logger {
    let append = !truncate;

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(append)
        .truncate(truncate)
        .open(file_name)
        .unwrap();

    let json_drain = slog_json::Json::new(file)
        .set_pretty(pretty)
        .add_default_keys()
        .build();

    let drain = Mutex::new(json_drain).map(slog::Fuse);
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(
        drain.fuse(),
        o!(
            "app_ver" => env!("CARGO_PKG_VERSION"),
            "logger_name" => logger_name.clone(),
            "source_location" => PushFnValue(|record , s| {
                 s.emit(
                      format_args!(
                           "{}:{}:{}",
                           record.module(),
                           record.file(),
                           record.line(),
                      )
                 )
            })
        ),
    );

    info!(logger, "{}", format!("{} initialized", logger_name));

    logger
}
