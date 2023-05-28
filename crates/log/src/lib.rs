use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};

use quant_config::LogConfig;
use quant_util::constants::DEFAULT_LOG_FILE;

#[derive(Default)]
pub struct Logger {
    log_path: String,
}

impl Logger {
    pub fn from_config(config: LogConfig) -> Self {
        let log_path = config.log_path.unwrap_or(DEFAULT_LOG_FILE.into());
        Self { log_path }
    }

    pub fn init(&self) -> Result<(), fern::InitError> {
        let colors = ColoredLevelConfig::new()
            .info(Color::Green)
            .warn(Color::Yellow)
            .debug(Color::White)
            .error(Color::Red)
            .trace(Color::Blue);

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
            .chain(fern::log_file(&self.log_path)?)
            .apply()?;
        Ok(())
    }
}
