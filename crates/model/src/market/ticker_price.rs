use core::fmt;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct TickerPrice {
    pub symbol: String,
    pub price: String,
}

impl fmt::Display for TickerPrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ symbol: {}, price: {} }}", self.symbol, self.price)
    }
}
