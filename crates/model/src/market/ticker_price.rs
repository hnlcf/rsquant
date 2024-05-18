use core::fmt;
use std::str::FromStr;

use quant_util::time::u64_to_datetime;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::DecodeFromStr;

#[derive(Debug, Deserialize)]
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

impl fmt::Display for TickerPrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ symbol: {}, price: {} }}", self.symbol, self.price)
    }
}
