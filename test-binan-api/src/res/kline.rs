use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KlineRes {
    /// 开始时间
    pub open_time: u64,
    /// 开盘价
    pub open_price: String,
    /// 最高价
    pub high_price: String,
    /// 最低价
    pub low_price: String,
    /// 收盘价
    pub close_price: String,
    /// 成交量
    pub volume: String,
    /// 结束时间
    pub close_time: u64,
    /// 成交额
    pub quote_asset_volume: String,
    pub trades_num: u64,
    pub buy_base_asset_volume: String,
    pub buy_quote_asset_volume: String,
    pub ignore_field: String,
}

impl super::api::BinanResponse<'_> for KlineRes {}
