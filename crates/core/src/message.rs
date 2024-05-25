#![allow(unused)]

use core::fmt;

use actix::Message;
use binan_spot::{
    http::Credentials,
    market::klines::KlineInterval,
    trade::order::{
        Side,
        TimeInForce,
    },
};
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    model::{
        account_info::AccountInfo,
        kline::Kline,
        ticker_price::TickerPrice,
    },
    Error,
};

#[derive(Message)]
#[rtype(result = "Result<NormalResponse, Error>")]
pub enum NormalRequest {
    Stop,
}

pub enum NormalResponse {
    Success,
    Failure(Error),
}

#[derive(Message)]
#[rtype(result = "Result<AccountInfoApiResponse, Error>")]
pub struct AccountInfoApiRequest;

#[derive(Debug)]
pub struct AccountInfoApiResponse {
    pub info: AccountInfo,
}

#[derive(Debug, Clone, Message, Deserialize)]
#[rtype(result = "Result<TickerApiResponse, Error>")]
pub struct TickerApiRequest {
    pub symbol: String,
    pub interval: u64,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct TickerApiResponse {
    pub ticker: TickerPrice,
}

#[derive(Debug, Default, Clone, Message, Deserialize)]
#[rtype(result = "Result<MultipleTickerApiResponse, Error>")]
pub struct MultipleTickerApiRequest {
    pub symbols: Vec<String>,
    pub interval: u64,
}

unsafe impl Send for MultipleTickerApiRequest {}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct MultipleTickerApiResponse {
    pub tickers: Vec<TickerPrice>,
}

#[derive(Message, Clone)]
#[rtype(result = "Result<KlineApiResponse, Error>")]
pub struct KlineApiRequest {
    pub symbol: String,
    pub interval: KlineInterval,
    pub start_time: u64,
    pub end_time: u64,
    pub limit: u32,
}

#[derive(Debug)]
pub struct KlineApiResponse {
    pub symbol: String,
    pub interval: KlineInterval,
    pub klines: Vec<Kline>,
}

#[derive(Message)]
#[rtype(result = "Result<NewOrderApiResponse, Error>")]
pub struct NewOrderApiRequest {
    pub symbol: String,
    pub side: Side,
    pub r#type: String,
    pub time_in_force: TimeInForce,
    pub quantity: Decimal,
    pub price: Decimal,
}

#[derive(Debug)]
pub struct NewOrderApiResponse {
    pub res: String,
}

#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub struct SendEmailRequest {
    pub subject: String,
    pub content: String,
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

pub type SubscribeTickerRequest = MultipleTickerApiRequest;
