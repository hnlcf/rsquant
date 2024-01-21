use quant_config::SqliteConfig;
use quant_util::constants::DEFAULT_SQLITE_DB_FILE;
use rusqlite::{Connection, Params};

use super::DBConnection;

pub struct SqliteConnection {
    conn: Option<Connection>,
}

impl SqliteConnection {
    pub fn from_config(config: SqliteConfig) -> Self {
        let conn = Connection::open(config.db_path);
        match conn {
            Ok(conn) => Self { conn: Some(conn) },
            Err(e) => {
                tracing::error!("{}", e);
                Self { conn: None }
            }
        }
    }

    pub fn init(&self) {
        self.execute(
            "CREATE TABLE IF NOT EXISTS assets_ticker_price (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            price       TEXT NOT NULL,
            unix_time   INTEGER NOT NULL,
            date_time   TEXT NOT NULL
         )",
            (),
        );
        self.execute(
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

    pub fn insert_data<V: Params>(&self, table_name: &str, fields: &[&str], data: V) {
        let mut fields_name = vec![];
        let mut values_count = vec![];
        for (i, &f) in fields.iter().enumerate() {
            fields_name.push(f.to_owned());
            values_count.push(format!("?{}", i + 1));
        }

        let fields_name_str = fields_name.join(",");
        let values_count_str = values_count.join(",");
        let insert_sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name, fields_name_str, values_count_str
        );

        self.execute(&insert_sql, data);
    }

    pub fn execute<P: Params>(&self, sql: &str, params: P) {
        if let Some(ref conn) = self.conn {
            let res = conn.execute(sql, params);
            if let Err(e) = res {
                tracing::warn!("{}", e);
            }
        }
    }
}

impl Default for SqliteConnection {
    fn default() -> Self {
        let conn = Connection::open(DEFAULT_SQLITE_DB_FILE).unwrap();
        Self { conn: Some(conn) }
    }
}

impl DBConnection for SqliteConnection {}

#[cfg(test)]
mod tests {
    #[test]
    fn sql_insert() {
        assert_eq!(1 + 1, 2);
    }
}
