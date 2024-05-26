use core::fmt;

use serde::Deserialize;

use crate::{
    model::DecodeFromStr,
    util::time::u64_to_datetime,
};

#[derive(Debug, Deserialize)]

pub struct Kline {
    /// 开始时间
    #[serde(deserialize_with = "u64_to_datetime")]
    pub open_time: chrono::NaiveDateTime,
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
    #[serde(deserialize_with = "u64_to_datetime")]
    pub close_time: chrono::NaiveDateTime,
    /// 成交额
    pub quote_asset_volume: String,
    pub trades_num: i64,
    pub buy_base_asset_volume: String,
    pub buy_quote_asset_volume: String,
    pub ignore_field: String,
}

impl DecodeFromStr<'_, Vec<Kline>> for Vec<Kline> {}

impl fmt::Display for Kline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{
    open_time: {},
    open_price: {},
    high_price: {},
    low_price: {},
    close_price: {},
    volume: {},
    close_time: {},
    quote_asset_volume: {},
    trades_num: {},
    buy_base_asset_volume: {},
    buy_quote_asset_volume: {},
    ignore_field: {}
}}",
            self.open_time,
            self.open_price,
            self.high_price,
            self.low_price,
            self.close_price,
            self.volume,
            self.close_time,
            self.quote_asset_volume,
            self.trades_num,
            self.buy_base_asset_volume,
            self.buy_quote_asset_volume,
            self.ignore_field
        )
    }
}
