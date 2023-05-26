use quant_util::constants::DEFAULT_SQLITE_DB_FILE;
use rusqlite::Params;

use super::sqlite::SqliteConnection;

pub struct Recorder {
    conn: SqliteConnection,
}

impl Recorder {
    pub fn new(conn: SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn init(&self) {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS assets_ticker_price (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            price       TEXT NOT NULL,
            unix_time   INTEGER NOT NULL,
            date_time   TEXT NOT NULL
         )",
            (),
        );
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS assets_kline_data (
            id                  INTEGER PRIMARY KEY,
            name                TEXT NOT NULL,
            open_price          TEXT NOT NULL,
            high_price          TEXT NOT NULL,
            low_price           TEXT NOT NULL,
            close_price         TEXT NOT NULL,
            volume              TEXT NOT NULL,
            quote_asset_volume  TEXT NOT NULL,
            open_date_time      TEXT NOT NULL,
            close_date_time     TEXT NOT NULL,
            open_unix_time      INTEGER NOT NULL,
            close_unix_time     INTEGER NOT NULL
         )",
            (),
        );
    }

    pub fn record_ticker_price_data<V: Params>(&self, fields: &[&str], data: V) {
        self.conn.insert_data("assets_ticker_price", fields, data);
    }

    pub fn record_kline_data<V: Params>(&self, fields: &[&str], data: V) {
        self.conn.insert_data("assets_kline_data", fields, data);
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Self {
            conn: SqliteConnection::create_connection(DEFAULT_SQLITE_DB_FILE),
        }
    }
}
