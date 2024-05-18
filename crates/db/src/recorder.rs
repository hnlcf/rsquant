use std::sync::{
    Arc,
    Mutex,
};

use quant_core::{
    util::config,
    Error,
    Result,
};

use super::pgsql::PostgresConnection;

#[derive(Default)]
pub struct Recorder {
    conn: Arc<Mutex<PostgresConnection>>,
}

impl Recorder {
    pub fn from_config(config: config::DatabaseConfig) -> Result<Self> {
        match config {
            config::DatabaseConfig::Postgresql(config) => {
                PostgresConnection::from_config(config).map(Self::new)
            }
            _ => Err(Error::Custom("Unsupported database type.".to_owned())),
        }
    }

    pub fn new(conn: PostgresConnection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub fn init(&self) {
        self.conn.lock().unwrap().init();
    }
}
