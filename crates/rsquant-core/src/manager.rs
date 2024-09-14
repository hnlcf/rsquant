use std::{
    collections::HashMap,
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
    trade::Strategy,
    util::{
        config::QuantConfig,
        log::Logger,
    },
    Error,
    FlattenErr,
    Result,
};

pub static STATE: OnceLock<Addr<QuantState>> = OnceLock::new();

pub async fn init_state<T>(
    config: QuantConfig,
    strategies: impl IntoIterator<Item = (T, Box<dyn Strategy>)>,
) where
    T: Into<String>,
{
    let mut state = QuantState::from_config(config)
        .await
        .expect("Failed to create manager");

    for (t, s) in strategies {
        state = state.register_strategy(t.into(), s);
    }

    let state = state.start();
    let _manager = STATE.get_or_init(move || state);
}

pub struct QuantState {
    config: QuantConfig,
    api: Option<Addr<BinanApiActor>>,
    email: Option<Addr<EmailActor>>,
    strategies: HashMap<String, Addr<StrategyActor>>,
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
        let recorder = DBService::from_config(database).await?.start();
        let logger = Logger::from_config(log);
        logger.init().expect("Failed to init logger");

        Ok(Self {
            config,
            api: Some(api),
            email: Some(email),
            strategies: HashMap::new(),
            db_service: Some(recorder),
            _logger: Some(logger),
        })
    }

    pub fn register_strategy(mut self, topic: String, strategy: Box<dyn Strategy>) -> Self {
        let strategy = StrategyActor::new(strategy).start();
        self.strategies.insert(topic, strategy);
        self
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
        if let Some(api) = self.api.clone() {
            api.send(msg)
                .into_actor(self)
                .map(|res, _slf, _ctx| res.flatten_err())
                .boxed_local()
        } else {
            async move { Err(Error::Custom("API actor is not initialized".into())) }
                .into_actor(self)
                .boxed_local()
        }
    }
}

impl Handler<AccountInfoApiRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<AccountInfoApiResponse>>;

    fn handle(&mut self, msg: AccountInfoApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(api) = self.api.clone() {
            api.send(msg)
                .into_actor(self)
                .map(|res, _slf, _ctx| res.flatten_err())
                .boxed_local()
        } else {
            async move { Err(Error::Custom("API actor is not initialized".into())) }
                .into_actor(self)
                .boxed_local()
        }
    }
}

impl Handler<MultipleTickerApiRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<MultipleTickerApiResponse>>;

    fn handle(&mut self, msg: MultipleTickerApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(api) = self.api.clone() {
            api.send(msg)
                .into_actor(self)
                .map(|res, _slf, _ctx| res.flatten_err())
                .boxed_local()
        } else {
            async move { Err(Error::Custom("API actor is not initialized".into())) }
                .into_actor(self)
                .boxed_local()
        }
    }
}

impl Handler<KlineApiRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<KlineApiResponse>>;

    fn handle(&mut self, msg: KlineApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(api) = self.api.clone() {
            api.send(msg)
                .into_actor(self)
                .map(|res, _slf, _ctx| res.flatten_err())
                .boxed_local()
        } else {
            async move { Err(Error::Custom("API actor is not initialized".into())) }
                .into_actor(self)
                .boxed_local()
        }
    }
}

impl Handler<NewOrderApiRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<NewOrderApiResponse>>;

    fn handle(&mut self, msg: NewOrderApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(api) = self.api.clone() {
            api.send(msg)
                .into_actor(self)
                .map(|res, _slf, _ctx| res.flatten_err())
                .boxed_local()
        } else {
            async move { Err(Error::Custom("API actor is not initialized".into())) }
                .into_actor(self)
                .boxed_local()
        }
    }
}

impl Handler<KlineStrategyRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<entity::side::TradeSide>>;

    fn handle(&mut self, msg: KlineStrategyRequest, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(strategy) = self.strategies.get(&msg.strategy_topic).cloned() {
            strategy
                .send(msg)
                .into_actor(self)
                .map(|res, _slf, _ctx| res.flatten_err())
                .boxed_local()
        } else {
            async move { Err(Error::Custom("Strategy actor is not initialized".into())) }
                .into_actor(self)
                .boxed_local()
        }
    }
}

impl Handler<RecordOrderRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<entity::order::ActiveModel>>;

    fn handle(&mut self, msg: RecordOrderRequest, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(db_service) = self.db_service.clone() {
            db_service
                .send(msg)
                .into_actor(self)
                .map(|res, _slf, _ctx| res.flatten_err())
                .boxed_local()
        } else {
            async move { Err(Error::Custom("DB service actor is not initialized".into())) }
                .into_actor(self)
                .boxed_local()
        }
    }
}

impl Handler<SendEmailRequest> for QuantState {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: SendEmailRequest, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(email) = self.email.as_ref() {
            email
                .send(msg)
                .into_actor(self)
                .map(|res, _slf, _ctx| res.flatten_err())
                .boxed_local()
        } else {
            async move { Err(Error::Custom("Email actor is not initialized".into())) }
                .into_actor(self)
                .boxed_local()
        }
    }
}
