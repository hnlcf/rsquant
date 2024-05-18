use serde::Deserialize;
use std::collections::HashMap;
use std::error;
use std::fmt;

/// Unsuccessful response from the Binance API.
#[derive(Debug)]
pub enum ClientError {
    /// API server error complying with the error schema.
    Structured(HttpError<BinanceApiError>),
    /// API server error not complying with the error schema.
    Raw(HttpError<String>),
}

/// Generic Http Error
#[derive(Debug)]
pub struct HttpError<T> {
    /// Http status code
    pub status_code: u16,
    /// Response body content
    pub data: T,
    /// Response headers
    pub headers: HashMap<String, String>,
}

impl<T> HttpError<T> {
    pub fn new(status_code: u16, data: T, headers: HashMap<String, String>) -> Self {
        Self {
            status_code,
            data,
            headers,
        }
    }
}

/// Structured Binance server error
#[derive(Deserialize, Debug)]
pub struct BinanceApiError {
    /// Error code
    ///
    /// [API Documentation](https://binance-docs.github.io/apidocs/spot/en/#error-codes)
    #[serde(rename(deserialize = "code"))]
    pub code: i16,

    ///Error description
    #[serde(rename(deserialize = "msg"))]
    pub message: String,
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ClientError::Structured(ref e) => write!(f, "Structured Error: {}", e),
            ClientError::Raw(ref e) => write!(f, "Raw Error: {}", e),
        }
    }
}

impl error::Error for ClientError {
    fn description(&self) -> &str {
        match *self {
            ClientError::Structured(..) => "Binan api server error with structure error schema",
            ClientError::Raw(..) => "Binance api server error with raw error message",
        }
    }
}

impl<T> fmt::Display for HttpError<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        write!(
            f,
            "status_code: {}, data: {}, headers: {{ ",
            self.status_code, self.data
        )?;
        for (k, v) in &self.headers {
            write!(f, "{{ key: {}, value: {} }} ", k, v)?;
        }
        write!(f, "}}")
    }
}

impl<T> error::Error for HttpError<T>
where
    T: fmt::Display + fmt::Debug,
{
    fn description(&self) -> &str {
        "Http error with status code: {self.status_code}"
    }
}

impl fmt::Display for BinanceApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ code: {}, message: {} }}", self.code, self.message)
    }
}

impl error::Error for BinanceApiError {
    fn description(&self) -> &str {
        "Binance api error"
    }
}
