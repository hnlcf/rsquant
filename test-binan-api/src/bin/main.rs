use std::fs;

use binance_spot_connector_rust::{
    http::{request::RequestBuilder, Credentials, Method},
    hyper::{BinanceHttpClient, Error},
};
use env_logger::Builder;
use test_binan_api::{AccountRes, BinanHmacSignature};

static SIGNATURE_FILE: &str = "binance-signature.json";

fn get_credentials() -> Credentials {
    let sig_json_file = fs::File::open(SIGNATURE_FILE).expect("Can't open signatue file");
    let sig: BinanHmacSignature =
        serde_json::from_reader(sig_json_file).expect("Can't parse signature file");
    Credentials::from_hmac(sig.api_key(), sig.api_secret())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let client = BinanceHttpClient::default();
    let credentials = get_credentials();
    let request = RequestBuilder::new(Method::Get, "/api/v3/account")
        .params(vec![("recvWindow", "5000")])
        .credentials(credentials)
        .sign();

    let data = client.send(request).await?.into_body_str().await?;

    let json: AccountRes = serde_json::from_str(&data).expect("Can't parse response");

    println!("{}", json);

    Ok(())
}
