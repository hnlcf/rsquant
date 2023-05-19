use core::fmt;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct TickerPriceRes {
    pub symbol: String,
    pub price: String,
}

impl super::api::BinanResponse<'_> for TickerPriceRes {}

impl fmt::Display for TickerPriceRes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ symbol: {}, price: {} }}", self.symbol, self.price)
    }
}
