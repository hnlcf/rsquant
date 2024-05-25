use std::fs;

use tracing_appender::non_blocking::{
    NonBlocking,
    WorkerGuard,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

use crate::{
    util::{
        config::LogConfig,
        constants::DEFAULT_LOG_FILE,
    },
    Result,
};

pub struct Logger {
    log_dir: String,
    guards: Vec<WorkerGuard>,
    log_file: NonBlocking,
}

impl Logger {
    pub fn from_config(config: LogConfig) -> Self {
        let log_path = config.log_path.unwrap_or(DEFAULT_LOG_FILE.into());

        let file_appender = tracing_appender::rolling::never(&log_path, DEFAULT_LOG_FILE);
        let (log_file, guard) = tracing_appender::non_blocking(file_appender);

        Self {
            log_dir: log_path,
            guards: vec![guard],
            log_file,
        }
    }

    pub fn init(&self) -> Result<()> {
        self.init_log_file()?;
        self.init_logger()?;

        Ok(())
    }

    fn init_log_file(&self) -> Result<()> {
        fs::create_dir_all(&self.log_dir)?;

        Ok(())
    }

    fn init_logger(&self) -> Result<()> {
        let filter_layer = EnvFilter::try_from_env("QUANT_LOG_LEVEL")
            .or_else(|_| EnvFilter::try_new("info"))
            .unwrap();

        let file_layer = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_ansi(false)
            .with_file(false)
            .with_line_number(true)
            .with_thread_names(true)
            .with_thread_ids(false)
            .with_writer(self.log_file.clone());

        let tui_layer = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_ansi(true)
            .with_file(false)
            .with_line_number(true)
            .with_thread_names(false)
            .with_thread_ids(false)
            .with_writer(std::io::stdout);

        tracing_subscriber::registry()
            .with(file_layer)
            .with(tui_layer)
            .with(filter_layer)
            .init();

        Ok(())
    }
}
