pub use credentials::get_credentials;
pub use hmac::BinanHmacSignature;

mod credentials;
mod hmac;

pub trait Signature {}
