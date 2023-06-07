use std::process::abort;

use crate::DBConnection;

use diesel::prelude::*;
use quant_config::PostgresqlConfig;
use quant_model::kline::Kline;
use quant_model::ticker_price::TickerPrice;
use quant_util::constants::DEFAULT_POSTGRES_ADDR;

pub struct PostgresConnection {
    conn: PgConnection,
}

impl PostgresConnection {
    pub fn from_config(config: PostgresqlConfig) -> Self {
        match PgConnection::establish(&config.pg_addr) {
            Ok(conn) => {
                log::debug!("Establish connection with postgresql.");
                Self { conn }
            }
            Err(e) => {
                log::error!("Failed to connect database: {} with {}.", config.pg_addr, e);
                abort();
            }
        }
    }

    pub fn init(&self) {}

    pub fn insert_kline(&mut self, symbol: &str, klines: &[Kline]) {
        use quant_model::market::kline::KlineInsertEntry;
        use quant_model::schema::assets_kline_data;

        let klines: Vec<KlineInsertEntry> = klines
            .iter()
            .map(|k| KlineInsertEntry::from_kline(symbol, k.clone()))
            .collect();

        diesel::insert_into(assets_kline_data::table)
            .values(&klines)
            .execute(&mut self.conn)
            .expect("Error saving new klines.");
    }

    pub fn insert_ticker_price(&mut self, ticker_price: TickerPrice) {
        use quant_model::market::ticker_price::TickerPriceInsertEntry;
        use quant_model::schema::assets_ticker_price_data;

        diesel::insert_into(assets_ticker_price_data::table)
            .values(&TickerPriceInsertEntry::from(ticker_price))
            .execute(&mut self.conn)
            .expect("Error saving new ticker price.");
    }
}

impl Default for PostgresConnection {
    fn default() -> Self {
        let conn = PgConnection::establish(DEFAULT_POSTGRES_ADDR).unwrap();
        Self { conn }
    }
}

impl DBConnection for PostgresConnection {}
