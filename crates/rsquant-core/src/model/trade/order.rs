use serde::Deserialize;

use crate::model::DecodeFromStr;

#[derive(Debug, Deserialize)]
pub struct OrderResponse {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: u64,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    pub r#type: String,
    pub side: String,
    pub working_time: u64,
    pub self_trade_prevention_mode: String,
}

impl DecodeFromStr<'_, OrderResponse> for OrderResponse {}
