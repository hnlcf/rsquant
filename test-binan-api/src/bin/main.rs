use env_logger::Builder;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;

use binance_spot_connector_rust::http::Credentials;
use binance_spot_connector_rust::hyper::BinanceHttpClient;
use binance_spot_connector_rust::hyper::Error as BinanHyperError;
use binance_spot_connector_rust::market::klines::KlineInterval;

use test_binan_api::credential;
use test_binan_api::res;
use test_binan_api::util;

type BinanHttpClient = BinanceHttpClient<HttpsConnector<HttpConnector>>;

lazy_static! {
    static ref CREDENTIALS: Credentials =
        credential::CredentialBuilder::from_env().expect("Failed to get credential from envs.");
    static ref CLIENT: BinanHttpClient = BinanceHttpClient::default();
}

#[tokio::main]
async fn main() -> Result<(), BinanHyperError> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let account_info = res::get_account_info(&CLIENT, &CREDENTIALS)
        .await
        .remove_blank_coin();
    log::info!("Account info:\n{}", account_info);

    let start_time = util::TimeConverter::date_to_unix_time("2023-05-07 23:00:00").unwrap();
    let end_time = util::TimeConverter::date_to_unix_time("2023-05-08 23:00:00").unwrap();

    let kline = res::get_kline(
        &CLIENT,
        "ETHUSDT",
        KlineInterval::Hours1,
        start_time,
        end_time,
        1000,
    )
    .await;
    log::info!("{:#?}", kline);

    Ok(())
}
