#![allow(dead_code)]

use actix::{Actor, Addr};
use quant_config::{ConfigBuilder, QuantConfig};
use quant_core::{Error, Result};
use quant_db::recorder::Recorder;
use quant_log::Logger;
use quant_model::order;

use crate::{
    api::Api,
    message::{ApiRequest, ApiResponse, NormalRequest},
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
    pub fn from_config() -> Result<Self> {
        if let Ok(config) = ConfigBuilder::build() {
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
        } else {
            Ok(QuantState::default())
        }
    }

    pub fn init(&mut self) -> Result<()> {
        self.logger.init()?;
        self.recorder.init();

        Ok(())
    }

    pub async fn stop(&self) {
        let _ = self.api.send(NormalRequest::Stop).await;
    }

    pub fn recorder(&self) -> &Recorder {
        &self.recorder
    }

    pub async fn get_info(&self, req: ApiRequest) -> Result<ApiResponse> {
        let res = self
            .api
            .send(req)
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;

        match res {
            ApiResponse::Ticker(ref t) => self.recorder.record_ticker_price_data(t),
            ApiResponse::Kline(ref k) => self.recorder.record_kline_data(k),
        }?;

        tracing::debug!("{}", res);

        Ok(res)
    }

    pub async fn get_orders(&self) -> Vec<order::Order> {
        todo!("Get orders")
    }
}
