use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};

use crate::env;

const DEFAULT_LOG_FILE: &str = "log/output.log";

pub struct Logger(());

impl Logger {
    pub fn setup_logger() -> Result<(), fern::InitError> {
        let colors = ColoredLevelConfig::new()
            .info(Color::Green)
            .warn(Color::Yellow)
            .debug(Color::White)
            .error(Color::Red)
            .trace(Color::Blue);
        let log_file =
            env::EnvManager::get_env_var("BINAN_LOG_FILE").unwrap_or(DEFAULT_LOG_FILE.into());
        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    colors.color(record.level()),
                    record.target(),
                    message
                ))
            })
            .level(log::LevelFilter::Debug)
            .chain(std::io::stdout())
            .chain(fern::log_file(log_file)?)
            .apply()?;
        Ok(())
    }
}
