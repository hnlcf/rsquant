use std::rc::Rc;

use actix::{
    Actor,
    ActorContext,
    ActorFutureExt,
    Context,
    Handler,
    ResponseActFuture,
    WrapFuture,
};

use crate::{
    api::{
        credential,
        message::{
            AccountInfoApiRequest,
            AccountInfoApiResponse,
            KlineApiRequest,
            KlineApiResponse,
            NewOrderApiRequest,
            NewOrderApiResponse,
            NormalRequest,
            NormalResponse,
            TickerApiRequest,
            TickerApiResponse,
        },
        req::{
            ApiImpl,
            HttpClient,
        },
    },
    util::config,
    Error,
};

pub struct BinanApi {
    client: Rc<HttpClient>,
}

impl BinanApi {
    pub async fn from_config(credentials: config::CredentialsConfig) -> Self {
        match credentials {
            config::CredentialsConfig::Binance(binan_credentials) => {
                let credentials = credential::CredentialBuilder::from_config(binan_credentials)
                    .expect("Failed to get credentials from config file.");

                let client = Rc::new(HttpClient::new(credentials.to_owned()).await);

                Self { client }
            }
            _ => BinanApi::default().await,
        }
    }

    pub async fn default() -> Self {
        let credentials = credential::CredentialBuilder::from_env()
            .expect("Failed to create credential from envs.");
        Self {
            client: Rc::new(HttpClient::new(credentials.to_owned()).await),
        }
    }
}

impl Actor for BinanApi {
    type Context = Context<Self>;
}

impl Handler<NormalRequest> for BinanApi {
    type Result = Result<NormalResponse, Error>;

    fn handle(&mut self, msg: NormalRequest, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            NormalRequest::Stop => {
                ctx.stop();
                Ok(NormalResponse::Success)
            }
        }
    }
}

impl Handler<AccountInfoApiRequest> for BinanApi {
    type Result = ResponseActFuture<Self, Result<AccountInfoApiResponse, Error>>;

    fn handle(&mut self, msg: AccountInfoApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        async move {
            ApiImpl::get_account_info(&client, msg)
                .await
                .map_err(Error::from)
        }
        .into_actor(self)
        .map(|res, _slf, _ctx| res.map(|info| AccountInfoApiResponse { info }))
        .boxed_local()
    }
}

impl Handler<TickerApiRequest> for BinanApi {
    type Result = ResponseActFuture<Self, Result<TickerApiResponse, Error>>;

    fn handle(&mut self, msg: TickerApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        async move {
            ApiImpl::get_ticker_price(&client, msg)
                .await
                .map_err(Error::from)
        }
        .into_actor(self)
        .map(|res, _slf, _ctx| res.map(|ticker| TickerApiResponse { ticker }))
        .boxed_local()
    }
}

impl Handler<KlineApiRequest> for BinanApi {
    type Result = ResponseActFuture<Self, Result<KlineApiResponse, Error>>;

    fn handle(&mut self, msg: KlineApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let KlineApiRequest {
            symbol, interval, ..
        } = msg.clone();

        let client = self.client.clone();
        async move { ApiImpl::get_kline(&client, msg).await.map_err(Error::from) }
            .into_actor(self)
            .map(move |res, _slf, _ctx| {
                res.map(|klines| KlineApiResponse {
                    symbol,
                    interval,
                    klines,
                })
            })
            .boxed_local()
    }
}

impl Handler<NewOrderApiRequest> for BinanApi {
    type Result = ResponseActFuture<Self, Result<NewOrderApiResponse, Error>>;

    fn handle(&mut self, msg: NewOrderApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        async move { ApiImpl::new_order(&client, msg).await.map_err(Error::from) }
            .into_actor(self)
            .map(|res, _slf, _ctx| res.map(|res| NewOrderApiResponse { res }))
            .boxed_local()
    }
}
