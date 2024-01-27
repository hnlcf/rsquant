use binan_spot::hyper::Error as BinanceHttpError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to send HTTP request to Binance by `{0}`.")]
    BinanceHttp(#[from] BinanceHttpError),

    #[error("Failed to deserialize json str by `{0}`")]
    Serde(#[from] serde_json::Error),

    #[error("IO error by `{0}`")]
    IO(#[from] std::io::Error),

    #[error("Db connect error by `{0}`")]
    DbConnect(#[from] diesel::ConnectionError),

    #[error("Db execute error by `{0}`")]
    DbExecute(#[from] diesel::result::Error),

    #[error("Config parse error by `{0}`")]
    ParseConfig(#[from] toml::de::Error),

    #[error("Email error by `{0}`")]
    Email(#[from] lettre::error::Error),

    #[error("Custom error by `{0}`")]
    Custom(String),
}
