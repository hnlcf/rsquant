use crate::DBConnection;

use diesel::prelude::*;

use quant_config::PostgresqlConfig;
use quant_core::{Error, Result};
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
}

impl Default for PostgresConnection {
    fn default() -> Self {
        let conn = PgConnection::establish(DEFAULT_POSTGRES_ADDR).unwrap();
        Self { conn }
    }
}

impl DBConnection for PostgresConnection {}
