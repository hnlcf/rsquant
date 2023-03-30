use env_logger::Builder;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;

use binance_spot_connector_rust::{
    http::{request::Request, Credentials},
    hyper::{BinanceHttpClient, Error},
    trade::account::Account,
};
use test_binan_api::{credential::CredentialBuilder, res::AccountRes};

type BinanHttpClient = BinanceHttpClient<HttpsConnector<HttpConnector>>;

static CREDENTIAL_FILE: &str = "binance-credential.json";

lazy_static! {
    static ref CREDENTIALS: Credentials =
        CredentialBuilder::from_json(CREDENTIAL_FILE).expect("Can't parse signature file.");
    static ref CLIENT: BinanHttpClient = BinanceHttpClient::default();
}

async fn get_account_info(
    client: &BinanHttpClient,
    credentials: &Credentials,
) -> Result<AccountRes, Error> {
    let request: Request = Account::default()
        .credentials(&credentials)
        .recv_window(5000)
        .into();

    let data = client.send(request).await?.into_body_str().await?;
    Ok(serde_json::from_str(&data).expect("Can't parse account info response."))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let account_info = get_account_info(&CLIENT, &CREDENTIALS).await?;

    println!("{}", account_info);
    Ok(())
}
