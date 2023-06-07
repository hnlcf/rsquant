use core::fmt;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;

use quant_util::time::{TimeConverter, UtcTimeTool};

use crate::schema::assets_kline_data;

#[derive(Queryable, Selectable)]
#[diesel(table_name = assets_kline_data)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct KlineQueryEntry {
    pub id: i32,
    pub symbol: String,
    pub open_time: DateTime<Utc>,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub volume: String,
    pub close_time: DateTime<Utc>,
    pub quote_asset_volume: String,
    pub trades_num: i64,
    pub buy_base_asset_volume: String,
    pub buy_quote_asset_volume: String,
    pub ignore_field: String,
}

#[derive(Insertable)]
#[diesel(table_name = assets_kline_data)]
pub struct KlineInsertEntry {
    pub symbol: String,
    pub open_time: DateTime<Utc>,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub volume: String,
    pub close_time: DateTime<Utc>,
    pub quote_asset_volume: String,
    pub trades_num: i64,
    pub buy_base_asset_volume: String,
    pub buy_quote_asset_volume: String,
    pub ignore_field: String,
}

#[derive(Clone, Deserialize)]
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
    pub trades_num: i64,
    pub buy_base_asset_volume: String,
    pub buy_quote_asset_volume: String,
    pub ignore_field: String,
}

impl KlineInsertEntry {
    pub fn from_kline(symbol: &str, value: Kline) -> Self {
        KlineInsertEntry {
            symbol: symbol.into(),
            open_time: UtcTimeTool::to_date_time(value.open_time as i64).unwrap(),
            open_price: value.open_price,
            high_price: value.high_price,
            low_price: value.low_price,
            close_price: value.close_price,
            volume: value.volume,
            close_time: UtcTimeTool::to_date_time(value.close_time as i64).unwrap(),
            quote_asset_volume: value.quote_asset_volume,
            trades_num: value.trades_num,
            buy_base_asset_volume: value.buy_base_asset_volume,
            buy_quote_asset_volume: value.buy_quote_asset_volume,
            ignore_field: value.ignore_field,
        }
    }
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
