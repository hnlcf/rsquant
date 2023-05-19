#![allow(dead_code)]

use binan_spot::market::klines::KlineInterval;
use quant_api::res::{account_info, kline, ticker_price};
use quant_db::recorder::Recorder;
use quant_util::log;

use crate::api::Api;

pub struct Event;

#[derive(Default)]
pub struct Executor {
    events: Vec<Event>,
}

#[derive(Default)]
pub struct Manager {
    api: Api,
    recorder: Recorder,
    executor: Executor,
}

unsafe impl Send for Manager {}
unsafe impl Sync for Manager {}

impl Manager {
    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::Logger::setup_logger()?;

        self.recorder.init();

        Ok(())
    }

    pub fn recorder(&self) -> &Recorder {
        &self.recorder
    }

    pub async fn get_ticker_price(&self, symbol: &str) -> ticker_price::TickerPriceRes {
        self.api.get_ticker_price(symbol).await
    }

    pub async fn get_account_info(&self) -> account_info::AccountInfoRes {
        self.api.get_account_info().await
    }

    pub async fn get_kline(
        &self,
        symbol: &str,
        interval: KlineInterval,
        start_time: &str,
        end_time: &str,
    ) -> Vec<kline::KlineRes> {
        self.api
            .get_kline(symbol, interval, start_time, end_time)
            .await
    }
}
