use binan_spot::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    trade,
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    let credentials = Credentials::from_hmac("api-key".to_owned(), "api-secret".to_owned());
    let client = BinanceHttpClient::default().credentials(credentials);
    let request = trade::get_order("BNBUSDT").order_id(11);
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
