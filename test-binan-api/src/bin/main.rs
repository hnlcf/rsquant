use env_logger::Builder;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;

use binance_spot_connector_rust::http::Credentials;
use binance_spot_connector_rust::hyper::BinanceHttpClient;
use binance_spot_connector_rust::hyper::Error as BinanHyperError;
use test_binan_api::credential::CredentialBuilder;
use test_binan_api::res;

type BinanHttpClient = BinanceHttpClient<HttpsConnector<HttpConnector>>;

lazy_static! {
    static ref CREDENTIALS: Credentials =
        CredentialBuilder::from_env().expect("Failed to get credential from envs.");
    static ref CLIENT: BinanHttpClient = BinanceHttpClient::default();
}

#[tokio::main]
async fn main() -> Result<(), BinanHyperError> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let account_info = res::get_account_info(&CLIENT, &CREDENTIALS).await;

    println!("{}", account_info);
    Ok(())
}
