use core::fmt;
use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};

use crate::model::DecodeFromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerPrice {
    pub symbol: String,
    pub price: String,
}

impl TickerPrice {
    pub fn price(&self) -> Decimal {
        Decimal::from_str(self.price.as_str()).unwrap()
    }
}

impl DecodeFromStr<'_, TickerPrice> for TickerPrice {}
impl DecodeFromStr<'_, Vec<TickerPrice>> for Vec<TickerPrice> {}

impl fmt::Display for TickerPrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ symbol: {}, price: {} }}", self.symbol, self.price)
    }
}
