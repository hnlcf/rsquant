use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

pub use account_info::AccountInfoRes;
use binance_spot_connector_rust::http::request::Request;
use binance_spot_connector_rust::http::Credentials;
use binance_spot_connector_rust::hyper::BinanceHttpClient;
use binance_spot_connector_rust::trade::account::Account;

mod account_info;

type BinanHttpClient = BinanceHttpClient<HttpsConnector<HttpConnector>>;

pub trait BinanResponse {}

pub async fn get_account_info(
    client: &BinanHttpClient,
    credentials: &Credentials,
) -> AccountInfoRes {
    let request: Request = Account::default()
        .credentials(credentials)
        .recv_window(5000)
        .into();

    let data = client
        .send(request)
        .await
        .expect("Failed to send request.")
        .into_body_str()
        .await
        .expect("Failed to convert response body into string.");
    serde_json::from_str(&data).expect("Failed to parse `AccountInfo`.")
}
