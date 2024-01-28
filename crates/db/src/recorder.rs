use std::sync::{Arc, Mutex};

use quant_config::DatabaseConfig;
use quant_core::{Error, Result};
use quant_model::kline::Kline;
use quant_model::ticker_price::TickerPrice;

use super::pgsql::PostgresConnection;

#[derive(Default)]
pub struct Recorder {
    conn: Arc<Mutex<PostgresConnection>>,
}

impl Recorder {
    pub fn from_config(config: DatabaseConfig) -> Result<Self> {
        match config {
            DatabaseConfig::Postgresql(config) => {
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

    pub fn record_ticker_price_data(&self, ticker_price: &TickerPrice) -> Result<usize> {
        self.conn.lock().unwrap().insert_ticker_price(ticker_price)
    }

    pub fn record_kline_data(&self, kline: &[Kline]) -> Result<usize> {
        self.conn.lock().unwrap().insert_kline(kline)
    }
}
