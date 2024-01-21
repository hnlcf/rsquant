use serde::Deserialize;

pub use account::{account_info, coin_info};
pub use market::{kline, ticker_price};

pub mod account;
pub mod market;

pub mod schema;

pub trait DecodeFromStr<'a, T>
where
    T: Deserialize<'a>,
{
    fn decode_from_str(data: &'a str) -> Option<T> {
        match serde_json::from_str(data) {
            Ok(t) => {
                tracing::debug!("Deserialize response string to data structure.");
                Some(t)
            }
            Err(e) => {
                tracing::error!(
                    "Failed to deserialize response string to data structure: {}.",
                    e
                );
                None
            }
        }
    }
}
