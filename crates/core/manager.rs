#![allow(dead_code)]

use binan_spot::market::klines::KlineInterval;
use quant_config::ConfigBuilder;
use quant_db::recorder::Recorder;
use quant_log::Logger;
use quant_model::{account_info, kline, order, ticker_price};

use crate::api::Api;

pub struct Manager {
    api: Api,
    recorder: Recorder,
    logger: Logger,
}

unsafe impl Send for Manager {}

unsafe impl Sync for Manager {}

impl Default for Manager {
    fn default() -> Self {
        Self {
            api: Api::default_with_proxy(),
            recorder: Recorder::default(),
            logger: Logger::default(),
        }
    }
}

impl Manager {
    pub fn from_config() -> Self {
        if let Some(config) = ConfigBuilder::build() {
            let api_config = config.api_credentials;
            let net_config = config.network;
            let db_config = config.database;
            let log_config = config.log;

            Self {
                api: Api::from_config(api_config, net_config),
                recorder: Recorder::from_config(db_config),
                logger: Logger::from_config(log_config),
            }
        } else {
            Manager::default()
        }
    }

    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.logger.init()?;
        self.recorder.init();

        Ok(())
    }

    pub fn recorder(&self) -> &Recorder {
        &self.recorder
    }

    pub async fn get_account_snapshot(&self) -> String {
        self.api.get_account_snapshot().await
    }

    pub async fn get_account_info(&self) -> account_info::AccountInfo {
        self.api.get_account_info().await
    }

    pub async fn get_ticker_price(&self, symbol: &str) -> ticker_price::TickerPrice {
        let ticker_price = self.api.get_ticker_price(symbol).await;

        self.recorder
            .record_ticker_price_data(ticker_price.to_owned());

        ticker_price
    }

    pub async fn get_kline(
        &self,
        symbol: &str,
        interval: KlineInterval,
        start_time: u64,
        end_time: u64,
    ) -> Vec<kline::Kline> {
        let klines = self
            .api
            .get_kline(symbol, interval, start_time, end_time)
            .await;

        self.recorder
            .record_kline_data(symbol, &interval.to_string(), &klines);

        klines
    }

    pub async fn get_orders(&self) -> Vec<order::Order> {
        todo!("Get orders")
    }
}
