use diesel::prelude::*;

use super::DBConnection;
use crate::{
    util::{
        config,
        constants,
    },
    Error,
    Result,
};

pub struct PostgresConnection {
    _conn: PgConnection,
}

impl PostgresConnection {
    pub fn from_config(config: config::PostgresqlConfig) -> Result<Self> {
        let pg_addr = match config.pg_addr {
            Some(addr) => {
                tracing::debug!("Get database address from config file: {}.", addr);
                addr
            }
            None => {
                tracing::warn!(
                    "Use default database address: {}.",
                    constants::DEFAULT_POSTGRES_ADDR
                );
                constants::DEFAULT_POSTGRES_ADDR.to_owned()
            }
        };

        PgConnection::establish(&pg_addr)
            .map(|conn| Self { _conn: conn })
            .map_err(Error::from)
    }

    pub fn init(&self) {}
}

impl Default for PostgresConnection {
    fn default() -> Self {
        let conn = PgConnection::establish(constants::DEFAULT_POSTGRES_ADDR).unwrap();
        Self { _conn: conn }
    }
}

impl DBConnection for PostgresConnection {}
