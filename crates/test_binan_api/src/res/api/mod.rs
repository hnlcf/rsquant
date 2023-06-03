use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde::Deserialize;

use binance_spot_connector_rust::hyper::BinanceHttpClient;
pub use get_response::GetResponse;

pub mod get_response;
pub mod handle_response;

pub trait BinanResponse<'a>: Deserialize<'a> {}

pub type BinanHttpClient = BinanceHttpClient<HttpsConnector<HttpConnector>>;
