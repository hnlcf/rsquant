use std::sync::OnceLock;

use actix::{
    Actor,
    ActorContext,
    ActorFutureExt,
    Addr,
    AsyncContext,
    Context,
    Handler,
    ResponseActFuture,
    WrapFuture,
};

use crate::{
    actor::{
        BinanApiActor,
        EmailActor,
    },
    db::recorder::Recorder,
    message::{
        AccountInfoApiRequest,
        AccountInfoApiResponse,
        KlineApiRequest,
        KlineApiResponse,
        MultipleTickerApiRequest,
        MultipleTickerApiResponse,
        NewOrderApiRequest,
        NewOrderApiResponse,
        NormalRequest,
        NormalResponse,
        TickerApiRequest,
        TickerApiResponse,
    },
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
        .expect("Failed to create manager")
        .start();
    let _manager = STATE.get_or_init(move || state);
}

pub struct QuantState {
    config: QuantConfig,
    api: Option<Addr<BinanApiActor>>,
    email: Option<Addr<EmailActor>>,
    recorder: Option<Recorder>,
    logger: Option<Logger>,
}

unsafe impl Send for QuantState {}

unsafe impl Sync for QuantState {}

impl QuantState {
    pub fn get_addr() -> Addr<QuantState> {
        STATE.get().expect("Manager is not initialized").clone()
    }
}

impl Actor for QuantState {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let QuantConfig {
            api_credentials,
            database,
            log,
            email,
            ..
        } = self.config.to_owned();

        let api = BinanApiActor::from_config(api_credentials).start();
        let email = EmailActor::from_config(email).start();
        let recorder = Recorder::from_config(database).expect("");
        let logger = Logger::from_config(log);
        recorder.init();
        logger.init().expect("Failed to init logger");

        self.api = Some(api);
        self.email = Some(email);
        self.recorder = Some(recorder);
        self.logger = Some(logger);
    }
}

impl QuantState {
    pub fn from_config(config: QuantConfig) -> Result<Self> {
        Ok(Self {
            config,
            api: None,
            email: None,
            recorder: None,
            logger: None,
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
