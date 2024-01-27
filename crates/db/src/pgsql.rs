use crate::DBConnection;

use diesel::prelude::*;

use quant_config::PostgresqlConfig;
use quant_core::{Error, Result};
use quant_model::{kline::Kline, ticker_price::TickerPrice};
use quant_util::constants::DEFAULT_POSTGRES_ADDR;

pub struct PostgresConnection {
    conn: PgConnection,
}

impl PostgresConnection {
    pub fn from_config(config: PostgresqlConfig) -> Result<Self> {
        let pg_addr = match config.pg_addr {
            Some(addr) => {
                tracing::debug!("Get database address from config file: {}.", addr);
                addr
            }
            None => {
                tracing::warn!("Use default database address: {}.", DEFAULT_POSTGRES_ADDR);
                DEFAULT_POSTGRES_ADDR.to_owned()
            }
        };

        PgConnection::establish(&pg_addr)
            .map(|conn| Self { conn })
            .map_err(Error::from)
    }

    pub fn init(&self) {}

    pub fn insert_kline(&mut self, klines: &[Kline]) -> Result<usize> {
        use quant_model::schema::assets_kline_data;

        diesel::insert_into(assets_kline_data::table)
            .values(klines)
            .execute(&mut self.conn)
            .map_err(Error::from)
    }

    pub fn insert_ticker_price(&mut self, ticker_price: &TickerPrice) -> Result<usize> {
        use quant_model::schema::assets_ticker_price_data;

        diesel::insert_into(assets_ticker_price_data::table)
            .values(ticker_price)
            .execute(&mut self.conn)
            .map_err(Error::from)
    }
}

impl Default for PostgresConnection {
    fn default() -> Self {
        let conn = PgConnection::establish(DEFAULT_POSTGRES_ADDR).unwrap();
        Self { conn }
    }
}

impl DBConnection for PostgresConnection {}
