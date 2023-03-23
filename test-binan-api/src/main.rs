mod api;

use crate::api::Api;
use std::fs;

use binance_spot_connector_rust::{
    http::{
        request::{Request, RequestBuilder},
        Credentials, Method,
    },
    hyper::{BinanceHttpClient, Error},
    trade::account::Account,
};
use env_logger::Builder;
use serde::{Deserialize, Serialize};

static API_FILE: &str = "binance-api.json";

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let api_file = fs::File::open(API_FILE).expect("Can't open api file");
    let api: Api = serde_json::from_reader(api_file).expect("Can't parse api file");

    log::info!("{:?}", api);
    low_level_account_test(&api.api_key(), &api.api_secret()).await?;
    high_level_account_test(&api.api_key(), &api.api_secret()).await?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccountResponse {
    #[serde(rename = "accountType")]
    account_type: String,
    balances: Vec<CoinInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CoinInfo {
    asset: String,
    free: String,
    locked: String,
}

async fn low_level_account_test(api_key: &str, api_secret: &str) -> Result<(), Error> {
    let client = BinanceHttpClient::default();
    let credentials = Credentials::from_hmac(api_key, api_secret);

    let request = RequestBuilder::new(Method::Get, "/api/v3/account")
        .credentials(credentials)
        .sign();

    let data = client
        .send(request)
        .await
        .expect("Request failed")
        .into_body_str()
        .await
        .expect("Failed to read response body");

    let json: AccountResponse = serde_json::from_str(&data).expect("Can't parse response");

    for i in json.balances {
        log::info!("{:?}", i);
    }

    Ok(())
}

async fn high_level_account_test(api_key: &str, api_secret: &str) -> Result<(), Error> {
    let client = BinanceHttpClient::default();
    let credentials = Credentials::from_hmac(api_key, api_secret);

    let request: Request = Account::new()
        .recv_window(5000)
        .credentials(&credentials)
        .into();

    let data = client
        .send(request)
        .await
        .expect("Request failed")
        .into_body_str()
        .await
        .expect("Failed to read response body");

    let json: AccountResponse = serde_json::from_str(&data).expect("Can't parse response");

    for i in json.balances {
        log::info!("{:?}", i);
    }

    Ok(())
}
