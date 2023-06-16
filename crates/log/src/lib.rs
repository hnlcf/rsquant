use std::{env, error, fs};

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

    pub fn init(&self) -> Result<(), Box<dyn error::Error>> {
        self.init_log_file()?;
        self.init_logger()?;

        Ok(())
    }

    fn init_log_file(&self) -> Result<(), Box<dyn error::Error>> {
        let mut log_file = env::current_dir()?;
        log_file.push(&self.log_path);

        if !log_file.exists() {
            let log_dir = log_file.parent().unwrap();
            fs::create_dir_all(log_dir)?;
            fs::File::create(log_file)?;
        }

        Ok(())
    }

    fn init_logger(&self) -> Result<(), Box<dyn error::Error>> {
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
