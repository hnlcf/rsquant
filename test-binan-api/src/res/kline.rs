use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Kline {
    pub open_time: u64,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub volume: String,
    pub close_time: u64,
    pub quote_asset_volume: String,
    pub trades_num: u64,
    pub buy_base_asset_volume: String,
    pub buy_quote_asset_volume: String,
    pub ignore_field: String,
}
