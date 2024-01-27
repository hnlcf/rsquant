pub mod credential;
pub mod error;
pub mod req;
pub mod res;

pub use error::Error;
type Result<T> = core::result::Result<T, Error>;
