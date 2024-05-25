pub mod actor;
pub mod api;
pub mod db;
mod error;
mod manager;
pub mod message;
pub mod model;
pub mod util;

pub type Result<T> = core::result::Result<T, Error>;

pub use error::Error;
pub use manager::{
    init_state,
    QuantState,
    STATE,
};
