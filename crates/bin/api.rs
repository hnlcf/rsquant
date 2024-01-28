use std::sync::Arc;

use actix::dev::ContextFutureSpawner;
use actix::{
    fut, Actor, ActorContext, ActorFuture, ActorFutureExt, AsyncContext, Context, Handler,
    ResponseActFuture, WrapFuture,
};
use binan_spot::{http::Credentials, market::klines::KlineInterval};
use quant_api::res::GetResponse;
use quant_api::{credential, res::BinanHttpClient};
use quant_config::{CredentialsConfig, NetworkConfig};
use quant_model::{account_info, kline, ticker_price};
use quant_util::env::EnvManager;

use crate::message::{ApiRequest, ApiResponse, NormalRequest, NormalResponse};

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
                        let client = Arc::new(BinanHttpClient::default_with_proxy(&proxy_uri));

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
        Self {
            credentials: credential::CredentialBuilder::from_env()
                .expect("Failed to create credential from envs."),
            client: Arc::new(BinanHttpClient::default_with_proxy(&proxy)),
        }
    }

    pub async fn get_account_snapshot(&self) -> Result<String, quant_core::Error> {
        GetResponse::get_account_snapshot(&self.client)
            .await
            .map_err(quant_core::Error::from)
    }

    /// # Get account information
    ///
    /// ## Examples
    ///
    /// ```
    /// let api = Api::default();
    /// let account_info = api.get_account_info().await;
    ///
    /// println!("{:#?}", account_info);
    /// ```
    pub async fn get_account_info(&self) -> Result<account_info::AccountInfo, quant_core::Error> {
        let account_info = GetResponse::get_account_info(&self.client, &self.credentials)
            .await
            .map_err(quant_core::Error::from)?
            .remove_blank_coin();

        tracing::info!("Get account info:\n{}", account_info);
        Ok(account_info)
    }
}

impl Actor for Api {
    type Context = Context<Self>;
}

impl Handler<ApiRequest> for Api {
    type Result = ResponseActFuture<Self, Result<ApiResponse, quant_core::Error>>;

    fn handle(&mut self, msg: ApiRequest, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ApiRequest::Ticker { symbol } => {
                let client = self.client.clone();
                async move {
                    GetResponse::get_ticker_price(&client, &symbol)
                        .await
                        .map_err(quant_core::Error::from)
                }
                .into_actor(self)
                .map(|res, _slf, _ctx| res.map(ApiResponse::Ticker))
                .boxed_local()
            }
            ApiRequest::Kline {
                symbol,
                interval,
                start_time,
                end_time,
            } => {
                let client = self.client.clone();
                async move {
                    GetResponse::get_kline(&client, &symbol, interval, start_time, end_time, 1000)
                        .await
                        .map_err(quant_core::Error::from)
                }
                .into_actor(self)
                .map(|res, _slf, _ctx| res.map(ApiResponse::Kline))
                .boxed_local()
            }
        }
    }
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
