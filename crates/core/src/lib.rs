mod error;
pub mod model;
pub mod util;

pub use error::Error;

pub type Result<T> = core::result::Result<T, Error>;
