#![allow(unused)]

use core::fmt;

use actix::Message;
use binan_spot::{
    market::klines::KlineInterval,
    trade::order::{Side, TimeInForce},
};
use quant_core::Error;
use quant_model::{kline::Kline, ticker_price::TickerPrice};
use rust_decimal::Decimal;

#[derive(Message)]
#[rtype(result = "Result<NormalResponse, quant_core::Error>")]
pub enum NormalRequest {
    Stop,
}

pub enum NormalResponse {
    Success,
    Failure(Error),
}
#[derive(Message)]
#[rtype(result = "Result<TickerApiResponse, quant_core::Error>")]
pub struct TickerApiRequest {
    pub symbol: String,
}

#[derive(Debug)]
pub struct TickerApiResponse {
    pub ticker: TickerPrice,
}

#[derive(Message)]
#[rtype(result = "Result<KlineApiResponse, quant_core::Error>")]
pub struct KlineApiRequest {
    pub symbol: String,
    pub interval: KlineInterval,
    pub start_time: u64,
    pub end_time: u64,
    pub limit: u32,
}

#[derive(Debug)]
pub struct KlineApiResponse {
    pub klines: Vec<Kline>,
}

#[derive(Message)]
#[rtype(result = "Result<NewOrderApiResponse, quant_core::Error>")]
pub struct NewOrderApiRequest {
    pub symbol: String,
    pub side: Side,
    pub r#type: String,
    pub time_in_force: TimeInForce,
    pub quantity: Decimal,
    pub price: Decimal,
    pub stop_price: Decimal,
}

#[derive(Debug)]
pub struct NewOrderApiResponse {
    pub res: String,
}

pub enum SchedulerDataRequest {
    Ticker,
    Kline,
}

/// Response message to scheduler from DB/API
pub enum SchedulerDataResponse {
    Ticker(TickerPrice),
    Kline(Vec<Kline>),
}

pub enum SchedulerToStrategyRequest {
    GeneralTickerPolicy,
}

pub enum TradeRequest {
    Buy,
    Sell,
}
