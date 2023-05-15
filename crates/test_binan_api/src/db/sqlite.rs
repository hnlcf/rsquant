use rusqlite::{Connection, Params};

use super::DBConnection;

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
                log::warn!("{}", e);
            }
        }
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
