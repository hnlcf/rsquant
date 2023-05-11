use rusqlite::{Connection, Params};

pub struct SqliteConnection {
    conn: Option<Connection>,
}

impl SqliteConnection {
    pub fn create_connection(db_file: &str) -> SqliteConnection {
        let conn = Connection::open(db_file);
        match conn {
            Ok(conn) => Self { conn: Some(conn) },
            Err(e) => {
                log::warn!("{}", e);
                Self { conn: None }
            }
        }
    }

    pub fn execute<P: Params>(&self, sql: &str, params: P) {
        if let Some(ref conn) = self.conn {
            let res = conn.execute(sql, params);
            if let Err(e) = res {
                log::warn!("{}", e);
            }
        }
    }
}
