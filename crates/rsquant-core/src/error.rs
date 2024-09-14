use binan_spot::hyper::Error as BinanceHttpError;
use sha2::digest::InvalidLength;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid length by `{0}`.")]
    InvalidLength(#[from] InvalidLength),

    #[error("Failed to parse url by `{0}`.")]
    Url(#[from] url::ParseError),

    #[error("Failed to send HTTP request by `{0}`.")]
    Http(#[from] http::Error),

    #[error("Failed to send HTTP request to Binance by `{0}`.")]
    BinanceHttp(#[from] BinanceHttpError),

    #[error("Failed to convert response body to string by `{0}`.")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Failed to deserialize json str by `{0}`")]
    Serde(#[from] serde_json::Error),

    #[error("IO error by `{0}`")]
    IO(#[from] std::io::Error),

    #[error("Failed to connect to database by `{0}`")]
    Database(#[from] sea_orm::error::DbErr),

    #[error("Email error by `{0}`")]
    Email(#[from] lettre::error::Error),

    #[error("Tera template error by `{0}`")]
    Template(#[from] tera::Error),

    #[error("Decimal error by `{0}`")]
    Decimal(#[from] rust_decimal::Error),

    #[error("Custom error by `{0}`")]
    Custom(String),
}

pub trait FlattenErr {
    type Error;
    type T;

    fn flatten_err(self) -> core::result::Result<Self::T, Self::Error>;
}

impl<T, E1: std::error::Error, E2: std::error::Error> FlattenErr
    for core::result::Result<core::result::Result<T, E1>, E2>
{
    type Error = Error;
    type T = T;

    fn flatten_err(self) -> core::result::Result<T, Error> {
        match self {
            Ok(Ok(t)) => Ok(t),
            Ok(Err(e)) => Err(Error::Custom(e.to_string())),
            Err(e) => Err(Error::Custom(e.to_string())),
        }
    }
}
