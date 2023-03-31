use env_logger::Builder;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

use binance_spot_connector_rust::http::request::Credentials;
use binance_spot_connector_rust::http::request::Request;
use binance_spot_connector_rust::hyper::BinanceHttpClient;
use binance_spot_connector_rust::hyper::Error as BinanHyperError;
use binance_spot_connector_rust::trade::account::Account;

use test_binan_api::credential::CredentialBuilder;
use test_binan_api::res::AccountInfoRes;
use test_binan_api::res::BinanResponse;

type BinanHttpClient = BinanceHttpClient<HttpsConnector<HttpConnector>>;

static CREDENTIAL_FILE: &str = "binance-credential.json";
static CREDENTIALS: Credentials =
    CredentialBuilder::from_json(CREDENTIAL_FILE).expect("Can't parse signature file.");
static CLIENT: BinanHttpClient = BinanceHttpClient::default();

#[tokio::main]
async fn main() -> Result<(), BinanHyperError> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let account_info = AccountInfoRes::get(&CLIENT, &CREDENTIALS).await;

    println!("{}", account_info);
    Ok(())
}
