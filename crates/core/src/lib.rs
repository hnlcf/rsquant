pub mod api;
pub mod db;
mod error;
mod manager;
pub mod model;
pub mod util;

pub type Result<T> = core::result::Result<T, Error>;

pub use error::Error;
pub use manager::{
    QuantState,
    STATE,
};
