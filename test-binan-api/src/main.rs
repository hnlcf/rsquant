mod api;
mod kline;

use crate::api::Api;
use std::fs;

use binance_spot_connector_rust::{
    http::{request::Request, Credentials},
    hyper::{BinanceHttpClient, Error},
    trade::account::Account,
};
use env_logger::Builder;

const API_FILE: &str = "binance-api.json";

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let api_file = fs::File::open(API_FILE).expect("Can't open api file");
    let api: Api = serde_json::from_reader(api_file).expect("Can't parse api file");

    println!("{:?}", api);
    http_test(&api.api_key(), &api.api_secret()).await?;

    Ok(())
}

async fn http_test(api_key: &str, api_secret: &str) -> Result<(), Error> {
    let credentials = Credentials::from_hmac(api_key, api_secret);
    let request: Request = Account::new()
        .recv_window(5000)
        .credentials(&credentials)
        .into();
    let client = BinanceHttpClient::with_url("https://api3.binance.com").credentials(credentials);

    let data = client.send(request).await?.into_body_str().await?;

    log::info!("{}", data);

    Ok(())
}
