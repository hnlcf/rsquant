use binan_spot::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    wallet,
};
use env_logger::Builder;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let credentials = Credentials::from_hmac("api-key".to_owned(), "api-secret".to_owned());
    let client = BinanceHttpClient::default().credentials(credentials);
    let request = wallet::withdraw("BNB", "address", dec!(1.01));
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
