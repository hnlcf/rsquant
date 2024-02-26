use serde::Deserialize;

pub use account::{account_info, coin_info};
pub use market::{kline, ticker_price};
pub use trade::order;

pub mod account;
pub mod market;
pub mod trade;

pub mod schema;

pub trait DecodeFromStr<'a, T>
where
    T: Deserialize<'a>,
{
    fn decode_from_str(data: &'a str) -> Result<T, serde_json::Error> {
        match serde_json::from_str(data) {
            Ok(t) => {
                tracing::trace!("Deserialize response string to data structure.");
                Ok(t)
            }
            Err(e) => {
                tracing::error!(
                    "Failed to deserialize response string to data structure: {}.",
                    e
                );
                Err(e)
            }
        }
    }
}

pub trait IntoTarget<T> {
    fn into_target(self) -> T;
}
