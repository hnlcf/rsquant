use std::{
    error,
    fmt,
};

use http::{
    uri::InvalidUri,
    Error as HttpError,
};
use hyper::Error as HyperError;

use crate::http::error::{
    ClientError,
    HttpError as BinanceHttpError,
};

/// Communication error with the server.
#[derive(Debug)]
pub enum Error {
    /// 4XX error from the server.
    Client(ClientError),
    /// 5XX error from the server.
    Server(BinanceHttpError<String>),
    /// The format of the API secret is invalid.
    InvalidApiSecret,
    Parse(HttpError),
    Send(HyperError),
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Error {
        Error::Parse(err.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::Client(ref e) => write!(f, "Client Error: {}", e),
            Error::Server(ref e) => write!(f, "Server Error: {}", e),
            Error::InvalidApiSecret => write!(f, "Invalid ApiSecret"),
            Error::Parse(ref e) => write!(f, "Parse Error: {}", e),
            Error::Send(ref e) => write!(f, "Send Error: {}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Client(..) => "Client error",
            Error::Server(..) => "Server error",
            Error::InvalidApiSecret => "InvalidApiSecret error",
            Error::Parse(..) => "Parse error",
            Error::Send(..) => "Send error",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Client(ref e) => Some(e),
            Error::Server(ref e) => Some(e),
            Error::InvalidApiSecret => None,
            Error::Parse(ref e) => Some(e),
            Error::Send(ref e) => Some(e),
        }
    }
}
