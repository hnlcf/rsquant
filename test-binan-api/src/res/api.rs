use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde::Deserialize;

use super::account_info;
use super::kline;

use binance_spot_connector_rust::http::request::Request;
use binance_spot_connector_rust::http::Credentials;
use binance_spot_connector_rust::hyper::BinanceHttpClient;
use binance_spot_connector_rust::market;
use binance_spot_connector_rust::market::klines::KlineInterval;
use binance_spot_connector_rust::trade::account::Account;

pub trait BinanResponse<'a>: Deserialize<'a> {}

type BinanHttpClient = BinanceHttpClient<HttpsConnector<HttpConnector>>;

pub async fn get_account_info(
    client: &BinanHttpClient,
    credentials: &Credentials,
) -> account_info::AccountInfoRes {
    let request: Request = Account::default()
        .credentials(credentials)
        .recv_window(5000)
        .into();

    let data = get_response(client, request).await;
    decode_response(&data).await
}

pub async fn get_kline(
    client: &BinanHttpClient,
    symbol: &str,
    interval: KlineInterval,
    start_time: u64,
    end_time: u64,
    limit: u32,
) -> Vec<kline::KlineRes> {
    let request = market::klines(symbol, interval)
        .start_time(start_time)
        .end_time(end_time)
        .limit(limit);

    let data = get_response(client, request).await;
    decode_response(&data).await
}

async fn decode_response<'a, T: Deserialize<'a>>(data: &'a str) -> T {
    serde_json::from_str(data).expect("Failed to parse response.")
}

async fn get_response(client: &BinanHttpClient, request: impl Into<Request>) -> String {
    client
        .send(request)
        .await
        .expect("Failed to send request.")
        .into_body_str()
        .await
        .expect("Failed to convert response body into string.")
}
