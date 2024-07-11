use actix::{
    Actor,
    Addr,
    Message,
};
use binan_spot::{
    market::klines::KlineInterval,
    trade::order::{
        Side,
        TimeInForce,
    },
};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{
    entity::{
        order,
        side,
    },
    model::{
        account_info::AccountInfo,
        kline::Kline,
        order::OrderResponse,
        ticker_price::TickerPrice,
    },
    trade::ToDataItem,
    Error,
};

#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub struct MessagePack<M, A>(M, Addr<A>)
where
    A: Actor;

impl<M, A> MessagePack<M, A>
where
    A: Actor,
{
    pub fn into_tuple(self) -> (M, Addr<A>) {
        (self.0, self.1)
    }
}

impl<M, A> From<(M, Addr<A>)> for MessagePack<M, A>
where
    A: Actor,
{
    fn from((msg, addr): (M, Addr<A>)) -> Self {
        MessagePack(msg, addr)
    }
}

#[derive(Message)]
#[rtype(result = "Result<NormalResponse, Error>")]
pub enum NormalRequest {
    Stop,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum NormalResponse {
    Success,
    Failure(Error),
}

#[derive(Message)]
#[rtype(result = "Result<AccountInfoApiResponse, Error>")]
pub struct AccountInfoApiRequest;

#[derive(Debug, Message)]
#[rtype(result = "()")]
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
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: u32,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct KlineApiResponse {
    pub symbol: String,
    pub interval: KlineInterval,
    pub klines: Vec<Kline>,
}

#[derive(Message, Clone)]
#[rtype(result = "Result<side::TradeSide, Error>")]
pub struct KlineStrategyRequest {
    pub strategy_topic: String,
    pub data: Vec<ta::DataItem>,
}

impl KlineStrategyRequest {
    pub fn from_klines(strategy_topic: &str, kline: KlineApiResponse) -> Self {
        let data = kline
            .klines
            .iter()
            .flat_map(|k| k.to_data_item().ok())
            .collect();
        KlineStrategyRequest {
            strategy_topic: strategy_topic.into(),
            data,
        }
    }
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

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct NewOrderApiResponse {
    pub res: OrderResponse,
}

#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub struct SendEmailRequest {
    pub subject: String,
    pub content: String,
}

#[derive(Message)]
#[rtype(result = "Result<order::ActiveModel, Error>")]
pub struct RecordOrderRequest {
    pub model: order::Model,
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
