use std::rc::Rc;

use actix::{
    dev::ToEnvelope,
    Actor,
    ActorContext,
    ActorFutureExt,
    AsyncContext,
    Context,
    Handler,
    Message,
    ResponseActFuture,
    WrapFuture,
};

use crate::{
    api::{
        credential,
        req::{
            ApiImpl,
            HttpClient,
        },
    },
    message::{
        AccountInfoApiRequest,
        AccountInfoApiResponse,
        KlineApiRequest,
        KlineApiResponse,
        MessagePack,
        MultipleTickerApiRequest,
        MultipleTickerApiResponse,
        NewOrderApiRequest,
        NewOrderApiResponse,
        NormalRequest,
        NormalResponse,
        TickerApiRequest,
        TickerApiResponse,
    },
    util::config,
    Error,
};

pub struct BinanApiActor {
    client: Rc<HttpClient>,
}

impl BinanApiActor {
    pub fn from_config(credentials: config::CredentialsConfig) -> Self {
        match credentials {
            config::CredentialsConfig::Binance(binan_credentials) => {
                let credentials = credential::CredentialBuilder::from_config(binan_credentials)
                    .expect("Failed to get credentials from config file.");

                let client = Rc::new(HttpClient::new(credentials.to_owned()));

                Self { client }
            }
            _ => BinanApiActor::default(),
        }
    }
}

impl Default for BinanApiActor {
    fn default() -> Self {
        let credentials = credential::CredentialBuilder::from_env()
            .expect("Failed to create credential from envs.");
        Self {
            client: Rc::new(HttpClient::new(credentials.to_owned())),
        }
    }
}

impl Actor for BinanApiActor {
    type Context = Context<Self>;
}

impl<M, A> Handler<MessagePack<M, A>> for BinanApiActor
where
    M: Message + Send + 'static,
    M::Result: Message + Send,
    <M::Result as Message>::Result: Send,
    A: Actor,
    A: Handler<M::Result>,
    A::Context: ToEnvelope<A, M::Result>,
    Self: Handler<M>,
{
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: MessagePack<M, A>, ctx: &mut Self::Context) -> Self::Result {
        let (msg, ret_addr) = msg.into_tuple();
        let self_addr = ctx.address();

        tokio::spawn(async move {
            if let Ok(res) = self_addr.send(msg).await {
                let _ = ret_addr.send(res).await;
            }
        });
        Ok(())
    }
}

impl Handler<NormalRequest> for BinanApiActor {
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

impl Handler<AccountInfoApiRequest> for BinanApiActor {
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

impl Handler<TickerApiRequest> for BinanApiActor {
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

impl Handler<MultipleTickerApiRequest> for BinanApiActor {
    type Result = ResponseActFuture<Self, Result<MultipleTickerApiResponse, Error>>;

    fn handle(&mut self, msg: MultipleTickerApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        async move {
            ApiImpl::get_multi_ticker_price(&client, msg)
                .await
                .map_err(Error::from)
        }
        .into_actor(self)
        .map(|res, _slf, _ctx| res.map(|tickers| MultipleTickerApiResponse { tickers }))
        .boxed_local()
    }
}

impl Handler<KlineApiRequest> for BinanApiActor {
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

impl Handler<NewOrderApiRequest> for BinanApiActor {
    type Result = ResponseActFuture<Self, Result<NewOrderApiResponse, Error>>;

    fn handle(&mut self, msg: NewOrderApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        async move { ApiImpl::new_order(&client, msg).await.map_err(Error::from) }
            .into_actor(self)
            .map(|res, _slf, _ctx| res.map(|res| NewOrderApiResponse { res }))
            .boxed_local()
    }
}
