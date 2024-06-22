use std::{
    self,
    sync::OnceLock,
};

use actix::{
    self,
    Actor,
    ActorFutureExt,
    Addr,
    Context,
    Handler,
    ResponseActFuture,
    WrapFuture,
};

use crate::{
    actor::{
        BinanApiActor,
        EmailActor,
        StrategyActor,
    },
    db::service::DBService,
    entity,
    message::{
        AccountInfoApiRequest,
        AccountInfoApiResponse,
        KlineApiRequest,
        KlineApiResponse,
        KlineStrategyRequest,
        MultipleTickerApiRequest,
        MultipleTickerApiResponse,
        NewOrderApiRequest,
        NewOrderApiResponse,
        NormalRequest,
        NormalResponse,
        RecordOrderRequest,
        SendEmailRequest,
        TickerApiRequest,
        TickerApiResponse,
    },
    trade::CommonMacdAndRsiStrategy,
    util::{
        config::QuantConfig,
        log::Logger,
    },
    Error,
    Result,
};

pub static STATE: OnceLock<Addr<QuantState>> = OnceLock::new();

pub async fn init_state(config: QuantConfig) {
    let state = QuantState::from_config(config)
        .await
        .expect("Failed to create manager")
        .start();
    let _manager = STATE.get_or_init(move || state);
}

pub struct QuantState {
    config: QuantConfig,
    api: Option<Addr<BinanApiActor>>,
    email: Option<Addr<EmailActor>>,
    strategy: Option<Addr<StrategyActor>>,
    db_service: Option<Addr<DBService>>,
    _logger: Option<Logger>,
}

impl QuantState {
    pub fn get_addr() -> Addr<QuantState> {
        STATE.get().expect("Manager is not initialized").clone()
    }
}

impl Actor for QuantState {
    type Context = Context<Self>;
}

impl QuantState {
    pub async fn from_config(config: QuantConfig) -> Result<Self> {
        let QuantConfig {
            api_credentials,
            database,
            log,
            email,
            ..
        } = config.to_owned();

        let api = BinanApiActor::from_config(api_credentials).start();
        let email = EmailActor::from_config(email).start();
        let strategy_impl = CommonMacdAndRsiStrategy::new(12, 26, 9, 14, 30.0, 70.0);
        let strategy = StrategyActor::new(Box::new(strategy_impl)).start();
        let recorder = DBService::from_config(database).await?.start();
        let logger = Logger::from_config(log);
        logger.init().expect("Failed to init logger");

        Ok(Self {
            config,
            api: Some(api),
            email: Some(email),
            strategy: Some(strategy),
            db_service: Some(recorder),
            _logger: Some(logger),
        })
    }

    pub fn config(&self) -> &QuantConfig {
        &self.config
    }
}

impl Handler<NormalRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<NormalResponse>>;

    fn handle(&mut self, msg: NormalRequest, _ctx: &mut Self::Context) -> Self::Result {
        let api_opt = self.api.clone();
        let email_opt = self.email.clone();
        async move {
            match msg {
                NormalRequest::Stop => {
                    if let Some(api) = api_opt {
                        let _ = api.send(NormalRequest::Stop).await;
                    }
                    if let Some(email) = email_opt {
                        let _ = email.send(NormalRequest::Stop).await;
                    }
                    tracing::debug!("Send stop signal to actor");
                    Ok(NormalResponse::Success)
                }
            }
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<TickerApiRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<TickerApiResponse>>;

    fn handle(&mut self, msg: TickerApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let api_opt = self.api.clone();
        async move {
            if let Some(api) = api_opt {
                let res = api
                    .send(msg)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))??;

                tracing::trace!("{:#?}", res);

                Ok(res)
            } else {
                Err(Error::Custom("API actor is not initialized".into()))
            }
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<AccountInfoApiRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<AccountInfoApiResponse>>;

    fn handle(&mut self, msg: AccountInfoApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let api_opt = self.api.clone();
        async move {
            if let Some(api) = api_opt {
                let res = api
                    .send(msg)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))??;

                tracing::trace!("{:#?}", res);

                Ok(res)
            } else {
                Err(Error::Custom("API actor is not initialized".into()))
            }
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<MultipleTickerApiRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<MultipleTickerApiResponse>>;

    fn handle(&mut self, msg: MultipleTickerApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let api_opt = self.api.clone();
        async move {
            if let Some(api) = api_opt {
                let res = api
                    .send(msg)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))??;

                tracing::trace!("{:#?}", res);

                Ok(res)
            } else {
                Err(Error::Custom("API actor is not initialized".into()))
            }
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<KlineApiRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<KlineApiResponse>>;

    fn handle(&mut self, msg: KlineApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let api_opt = self.api.clone();
        async move {
            if let Some(api) = api_opt {
                let res = api
                    .send(msg)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))??;

                tracing::trace!("{:#?}", res);

                Ok(res)
            } else {
                Err(Error::Custom("API actor is not initialized".into()))
            }
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<NewOrderApiRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<NewOrderApiResponse>>;

    fn handle(&mut self, msg: NewOrderApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let api_opt = self.api.clone();
        async move {
            if let Some(api) = api_opt {
                let res = api
                    .send(msg)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))??;

                tracing::trace!("{:#?}", res);

                Ok(res)
            } else {
                Err(Error::Custom("API actor is not initialized".into()))
            }
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<KlineStrategyRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<entity::side::TradeSide>>;

    fn handle(&mut self, msg: KlineStrategyRequest, _ctx: &mut Self::Context) -> Self::Result {
        let strategy_opt = self.strategy.clone();
        async move {
            if let Some(strategy) = strategy_opt {
                let res = strategy
                    .send(msg)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))??;

                tracing::trace!("{:#?}", res);

                Ok(res)
            } else {
                Err(Error::Custom("Strategy actor is not initialized".into()))
            }
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<RecordOrderRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<entity::order::ActiveModel>>;

    fn handle(&mut self, msg: RecordOrderRequest, _ctx: &mut Self::Context) -> Self::Result {
        let db_service_opt = self.db_service.clone();
        async move {
            if let Some(db_service) = db_service_opt {
                let res = db_service
                    .send(msg)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))??;

                Ok(res)
            } else {
                Err(Error::Custom("DB service actor is not initialized".into()))
            }
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<SendEmailRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: SendEmailRequest, _ctx: &mut Self::Context) -> Self::Result {
        let email_opt = self.email.clone();
        async move {
            if let Some(email) = email_opt {
                email
                    .send(msg)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))??;

                Ok(())
            } else {
                Err(Error::Custom("Email actor is not initialized".into()))
            }
        }
        .into_actor(self)
        .boxed_local()
    }
}
