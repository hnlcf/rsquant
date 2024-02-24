use actix::{Actor, Addr};
use quant_config::QuantConfig;
use quant_core::{Error, Result};
use quant_db::recorder::Recorder;
use quant_log::Logger;
use quant_model::{kline::Kline, order, ticker_price::TickerPrice};

use crate::{
    api::Api,
    message::{
        KlineApiRequest, KlineApiResponse, NewOrderApiRequest, NormalRequest, TickerApiRequest,
        TickerApiResponse,
    },
};

pub struct QuantState {
    config: QuantConfig,
    api: Addr<Api>,
    recorder: Recorder,
    logger: Logger,
}

unsafe impl Send for QuantState {}

unsafe impl Sync for QuantState {}

impl Default for QuantState {
    fn default() -> Self {
        Self {
            config: QuantConfig::default(),
            api: Api::default_with_proxy().start(),
            recorder: Recorder::default(),
            logger: Logger::default(),
        }
    }
}

impl QuantState {
    pub fn from_config(config: QuantConfig) -> Result<Self> {
        let QuantConfig {
            api_credentials,
            network,
            database,
            log,
            ..
        } = config.to_owned();

        let api = Api::from_config(api_credentials, network).start();
        let recorder = Recorder::from_config(database)?;
        let logger = Logger::from_config(log);

        Ok(Self {
            config,
            api,
            recorder,
            logger,
        })
    }

    pub fn config(&self) -> &QuantConfig {
        &self.config
    }

    pub fn init(&mut self) -> Result<()> {
        self.logger.init()?;
        self.recorder.init();

        Ok(())
    }

    pub async fn stop(&self) {
        let _ = self.api.send(NormalRequest::Stop).await;
        tracing::debug!("Send stop signal to actor")
    }

    pub fn recorder(&self) -> &Recorder {
        &self.recorder
    }

    pub async fn get_kline(&self, req: KlineApiRequest) -> Result<Vec<Kline>> {
        let res = self
            .api
            .send(req)
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;

        self.recorder().record_kline_data(&res.klines)?;

        tracing::trace!("{:#?}", res);

        Ok(res.klines)
    }

    pub async fn get_ticker(&self, req: TickerApiRequest) -> Result<TickerPrice> {
        let res = self
            .api
            .send(req)
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;

        self.recorder().record_ticker_price_data(&res.ticker)?;

        tracing::trace!("{:#?}", res);

        Ok(res.ticker)
    }

    pub async fn new_order(&self, req: NewOrderApiRequest) -> Result<String> {
        let res = self
            .api
            .send(req)
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;

        tracing::trace!("{:#?}", res);

        Ok(res.res)
    }

    pub async fn get_orders(&self) -> Vec<order::Order> {
        todo!("Get orders")
    }
}
