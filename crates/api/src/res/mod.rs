use hyper::client::HttpConnector;
use hyper_proxy::ProxyConnector;
use hyper_tls::HttpsConnector;
use serde::Deserialize;

use binan_spot::hyper::BinanceHttpClient;

pub use get_response::GetResponse;
pub use handle_response::HandleResponse;

pub mod get_response;
pub mod handle_response;

pub trait BinanResponse<'a>: Deserialize<'a> {}

pub type BinanHttpClient = BinanceHttpClient<ProxyConnector<HttpsConnector<HttpConnector>>>;
