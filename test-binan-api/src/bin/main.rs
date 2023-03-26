use env_logger::Builder;

use binance_spot_connector_rust::{
    http::{request::RequestBuilder, Method},
    hyper::{BinanceHttpClient, Error},
};

use test_binan_api::{credential::CredentialBuilder, res::AccountRes};

static CREDENTIAL_FILE: &str = "binance-credential.json";


#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let client = BinanceHttpClient::default();

    let credentials =
        CredentialBuilder::from_json(CREDENTIAL_FILE).expect("Can't parse signature file");
    let request = RequestBuilder::new(Method::Get, "/api/v3/account")
        .params(vec![("recvWindow", "5000")])
        .credentials(credentials)
        .sign();

    let data = client.send(request).await?.into_body_str().await?;
    let json: AccountRes = serde_json::from_str(&data).expect("Can't parse response");

    println!("{}", json);
    Ok(())
}
