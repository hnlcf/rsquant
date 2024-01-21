use std::{error, fs};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use quant_config::LogConfig;
use quant_util::constants::DEFAULT_LOG_FILE;

#[derive(Default)]
pub struct Logger {
    log_dir: String,
}

impl Logger {
    pub fn from_config(config: LogConfig) -> Self {
        let log_path = config.log_path.unwrap_or(DEFAULT_LOG_FILE.into());
        Self { log_dir: log_path }
    }

    pub fn init(&self) -> Result<(), Box<dyn error::Error>> {
        self.init_log_file()?;
        self.init_logger()?;

        Ok(())
    }

    fn init_log_file(&self) -> Result<(), Box<dyn error::Error>> {
        fs::create_dir_all(&self.log_dir)?;

        Ok(())
    }

    fn init_logger(&self) -> Result<(), Box<dyn error::Error>> {
        let filter_layer = EnvFilter::try_from_env("QUANT_LOG_LEVEL")
            .or_else(|_| EnvFilter::try_new("info"))
            .unwrap();

        // let file_appender = tracing_appender::rolling::daily(&self.log_dir, "quant.log");
        // let (log_file, _guard) = tracing_appender::non_blocking(file_appender);

        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_ansi(true)
            .with_file(false)
            .with_line_number(true)
            .with_thread_names(false)
            .with_thread_ids(false)
            .with_writer(std::io::stdout);

        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(filter_layer)
            .init();

        Ok(())
    }
}
