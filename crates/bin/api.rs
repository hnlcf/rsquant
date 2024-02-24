use std::sync::Arc;

use actix::dev::ContextFutureSpawner;
use actix::{
    fut, Actor, ActorContext, ActorFuture, ActorFutureExt, AsyncContext, Context, Handler,
    ResponseActFuture, System, WrapFuture,
};
use binan_spot::{http::Credentials, market::klines::KlineInterval};
use quant_api::{
    credential,
    message::{
        AccountInfoApiRequest, AccountInfoApiResponse, KlineApiRequest, KlineApiResponse,
        NewOrderApiRequest, NewOrderApiResponse, NormalRequest, NormalResponse, TickerApiRequest,
        TickerApiResponse,
    },
    res::{BinanHttpClient, GetResponse},
};
use quant_config::{CredentialsConfig, NetworkConfig};
use quant_model::{account_info, kline, ticker_price};
use quant_util::env::EnvManager;

pub struct Api {
    credentials: Credentials,
    client: Arc<BinanHttpClient>,
}

impl Api {
    pub fn from_config(credentials: CredentialsConfig, network: NetworkConfig) -> Self {
        match credentials {
            CredentialsConfig::Binance(binan_credentials) => {
                let credentials = credential::CredentialBuilder::from_config(binan_credentials)
                    .expect("Failed to get credentials from config file.");
                match network.proxy {
                    Some(proxy_config) => {
                        let proxy_uri = proxy_config.https_proxy.unwrap_or("".into());
                        let client = Arc::new(
                            BinanHttpClient::default_with_proxy(&proxy_uri)
                                .credentials(credentials.to_owned()),
                        );

                        Self {
                            credentials,
                            client,
                        }
                    }
                    None => Api::default_with_proxy(),
                }
            }
            _ => Api::default_with_proxy(),
        }
    }

    pub fn default_with_proxy() -> Self {
        let proxy = EnvManager::get_env_var_or("https_proxy", "");
        let credentials = credential::CredentialBuilder::from_env()
            .expect("Failed to create credential from envs.");
        Self {
            credentials: credentials.to_owned(),
            client: Arc::new(
                BinanHttpClient::default_with_proxy(&proxy).credentials(credentials.to_owned()),
            ),
        }
    }
}

impl Actor for Api {
    type Context = Context<Self>;
}

impl Handler<NormalRequest> for Api {
    type Result = Result<NormalResponse, quant_core::Error>;

    fn handle(&mut self, msg: NormalRequest, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            NormalRequest::Stop => {
                ctx.stop();
                Ok(NormalResponse::Success)
            }
        }
    }
}

impl Handler<AccountInfoApiRequest> for Api {
    type Result = ResponseActFuture<Self, Result<AccountInfoApiResponse, quant_core::Error>>;

    fn handle(&mut self, msg: AccountInfoApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        async move {
            GetResponse::get_account_info(&client, msg)
                .await
                .map_err(quant_core::Error::from)
        }
        .into_actor(self)
        .map(|res, _slf, _ctx| res.map(|info| AccountInfoApiResponse { info }))
        .boxed_local()
    }
}

impl Handler<TickerApiRequest> for Api {
    type Result = ResponseActFuture<Self, Result<TickerApiResponse, quant_core::Error>>;

    fn handle(&mut self, msg: TickerApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        async move {
            GetResponse::get_ticker_price(&client, msg)
                .await
                .map_err(quant_core::Error::from)
        }
        .into_actor(self)
        .map(|res, _slf, _ctx| res.map(|ticker| TickerApiResponse { ticker }))
        .boxed_local()
    }
}

impl Handler<KlineApiRequest> for Api {
    type Result = ResponseActFuture<Self, Result<KlineApiResponse, quant_core::Error>>;

    fn handle(&mut self, msg: KlineApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        async move {
            GetResponse::get_kline(&client, msg)
                .await
                .map_err(quant_core::Error::from)
        }
        .into_actor(self)
        .map(|res, _slf, _ctx| res.map(|klines| KlineApiResponse { klines }))
        .boxed_local()
    }
}

impl Handler<NewOrderApiRequest> for Api {
    type Result = ResponseActFuture<Self, Result<NewOrderApiResponse, quant_core::Error>>;

    fn handle(&mut self, msg: NewOrderApiRequest, _ctx: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        async move {
            GetResponse::new_order(&client, msg)
                .await
                .map_err(quant_core::Error::from)
        }
        .into_actor(self)
        .map(|res, _slf, _ctx| res.map(|res| NewOrderApiResponse { res }))
        .boxed_local()
    }
}
