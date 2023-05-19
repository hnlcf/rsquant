use rusqlite::Params;

use super::sqlite::SqliteConnection;

const DEFAULT_SQLITE_DB_FILE: &str = "database/bitcoin.db";

pub struct Recorder {
    conn: SqliteConnection,
}

impl Recorder {
    pub fn new(conn: SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn init(&self) {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS assets_price (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            price       TEXT NOT NULL,
            unix_time   INTEGER NOT NULL,
            date_time   TEXT NOT NULL
         )",
            (),
        );
    }

    pub fn record_ticker_price_data<V: Params>(&self, fields: &[&str], data: V) {
        self.conn.insert_data("assets_price", fields, data);
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Self {
            conn: SqliteConnection::create_connection(DEFAULT_SQLITE_DB_FILE),
        }
    }
}
