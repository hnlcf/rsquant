#![allow(unused)]

use core::fmt;

use actix::Message;
use binan_spot::market::klines::KlineInterval;
use quant_core::Error;
use quant_model::{kline::Kline, ticker_price::TickerPrice};

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
#[rtype(result = "Result<ApiResponse, quant_core::Error>")]
pub enum ApiRequest {
    Ticker {
        symbol: String,
    },
    Kline {
        symbol: String,
        interval: KlineInterval,
        start_time: u64,
        end_time: u64,
    },
}

#[derive(Debug)]
pub enum ApiResponse {
    Ticker(TickerPrice),
    Kline(Vec<Kline>),
}

impl fmt::Display for ApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiResponse::Ticker(ticker) => write!(f, "Ticker: {}", ticker),
            ApiResponse::Kline(klines) => {
                writeln!(f, "Kline:")?;
                for kline in klines {
                    write!(f, "{}", kline)?;
                }
                Ok(())
            }
        }
    }
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
