use core::fmt;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::assets_ticker_price_data;
use crate::DecodeFromStr;

#[derive(Queryable, Selectable)]
#[diesel(table_name = assets_ticker_price_data)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TickerPriceQueryEntry {
    pub id: i32,
    pub symbol: String,
    pub price: String,
    pub update_time: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = assets_ticker_price_data)]
pub struct TickerPriceInsertEntry {
    pub symbol: String,
    pub price: String,
    pub update_time: DateTime<Utc>,
}

#[derive(Clone, Deserialize)]
pub struct TickerPrice {
    pub symbol: String,
    pub price: String,
}

impl DecodeFromStr<'_, TickerPrice> for TickerPrice {}

impl From<TickerPrice> for TickerPriceInsertEntry {
    fn from(value: TickerPrice) -> Self {
        Self {
            symbol: value.symbol,
            price: value.price,
            update_time: Utc::now(),
        }
    }
}

impl fmt::Display for TickerPrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ symbol: {}, price: {} }}", self.symbol, self.price)
    }
}
