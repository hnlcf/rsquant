#![allow(dead_code)]

use binan_spot::market::klines::KlineInterval;
use quant_config::{ConfigBuilder, QuantConfig};
use quant_core::Result;
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
    pub fn from_config() -> Result<Self> {
        if let Ok(config) = ConfigBuilder::build() {
            let QuantConfig {
                api_credentials,
                network,
                database,
                log,
                ..
            } = config;

            let api = Api::from_config(api_credentials, network);
            let recorder = Recorder::from_config(database)?;
            let logger = Logger::from_config(log);

            Ok(Self {
                api,
                recorder,
                logger,
            })
        } else {
            Ok(Manager::default())
        }
    }

    pub fn init(&mut self) -> Result<()> {
        self.logger.init()?;
        self.recorder.init();

        Ok(())
    }

    pub fn recorder(&self) -> &Recorder {
        &self.recorder
    }

    pub async fn get_account_snapshot(&self) -> Result<String> {
        self.api.get_account_snapshot().await
    }

    pub async fn get_account_info(&self) -> Result<account_info::AccountInfo> {
        self.api.get_account_info().await
    }

    pub async fn get_ticker_price(&self, symbol: &str) -> Result<ticker_price::TickerPrice> {
        let ticker_price = self.api.get_ticker_price(symbol).await?;

        self.recorder.record_ticker_price_data(&ticker_price)?;

        Ok(ticker_price)
    }

    pub async fn get_kline(
        &self,
        symbol: &str,
        interval: KlineInterval,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<kline::Kline>> {
        let klines = self
            .api
            .get_kline(symbol, interval, start_time, end_time)
            .await?;

        self.recorder.record_kline_data(&klines)?;

        Ok(klines)
    }

    pub async fn get_orders(&self) -> Vec<order::Order> {
        todo!("Get orders")
    }
}
