use core::fmt;

use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::assets_ticker_price_data;
use crate::DecodeFromStr;

#[derive(Debug, Clone, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = assets_ticker_price_data)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TickerPrice {
    pub symbol: String,
    pub price: String,
    #[serde(skip_deserializing)]
    pub update_time: chrono::NaiveDateTime,
}

impl TickerPrice {
    pub fn from_ticker(ticker: TickerPrice) -> Self {
        Self {
            update_time: chrono::Local::now().naive_local(),
            ..ticker
        }
    }
}

impl DecodeFromStr<'_, TickerPrice> for TickerPrice {}

impl fmt::Display for TickerPrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ symbol: {}, price: {} }}", self.symbol, self.price)
    }
}
