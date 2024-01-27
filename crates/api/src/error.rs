use binan_spot::hyper::Error as BinanceHttpError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to send HTTP request to Binance by `{0}`.")]
    BinanceHttp(#[from] BinanceHttpError),

    #[error("Failed to deserialize json str by `{0}`")]
    Serde(#[from] serde_json::Error),
}
