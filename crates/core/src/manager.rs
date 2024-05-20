use std::sync::OnceLock;

use actix::{
    Actor,
    Addr,
};

use crate::{
    api::{
        actor::BinanApi,
        message::{
            AccountInfoApiRequest,
            KlineApiRequest,
            NewOrderApiRequest,
            NormalRequest,
            TickerApiRequest,
        },
    },
    db::recorder::Recorder,
    model::{
        account_info::AccountInfo,
        kline::Kline,
        ticker_price::TickerPrice,
    },
    util::{
        config::QuantConfig,
        log::Logger,
    },
    Error,
    Result,
};

pub static STATE: OnceLock<QuantState> = OnceLock::new();

pub struct QuantState {
    config: QuantConfig,
    api: Addr<BinanApi>,
    recorder: Recorder,
    logger: Logger,
}

unsafe impl Send for QuantState {}

unsafe impl Sync for QuantState {}

impl QuantState {
    pub async fn from_config(config: QuantConfig) -> Result<Self> {
        let QuantConfig {
            api_credentials,
            database,
            log,
            ..
        } = config.to_owned();

        let api = BinanApi::from_config(api_credentials).await.start();
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
}

/// Implement the API methods based on `Api` actor
impl QuantState {
    pub async fn get_account_info(&self) -> Result<AccountInfo> {
        let res = self
            .api
            .send(AccountInfoApiRequest)
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;

        tracing::trace!("{:#?}", res);

        Ok(res.info)
    }

    pub async fn get_ticker(&self, req: TickerApiRequest) -> Result<TickerPrice> {
        let res = self
            .api
            .send(req)
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;

        tracing::trace!("{:#?}", res);

        Ok(res.ticker)
    }

    pub async fn get_kline(&self, req: KlineApiRequest) -> Result<Vec<Kline>> {
        let res = self
            .api
            .send(req)
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;

        tracing::trace!("{:#?}", res);

        Ok(res.klines)
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
}
