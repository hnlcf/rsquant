use async_trait::async_trait;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

pub use account_info::AccountInfoRes;
use binance_spot_connector_rust::http::Credentials;
use binance_spot_connector_rust::hyper::BinanceHttpClient;

mod account_info;

type BinanHttpClient = BinanceHttpClient<HttpsConnector<HttpConnector>>;

#[async_trait]
pub trait BinanResponse {
    async fn get(client: &BinanHttpClient, credentials: &Credentials) -> Self;
}
