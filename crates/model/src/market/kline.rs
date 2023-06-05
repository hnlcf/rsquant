use core::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Kline {
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
