use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TickerPriceRes {
    pub symbol: String,
    pub price: String,
}

impl super::api::BinanResponse<'_> for TickerPriceRes {}
