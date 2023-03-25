use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BinanHmacSignature {
    api_key: String,
    api_secret: String,
}

impl super::Signature for BinanHmacSignature {}

impl BinanHmacSignature {
    pub fn api_key(&self) -> String {
        self.api_key.to_owned()
    }

    pub fn api_secret(&self) -> String {
        self.api_secret.to_owned()
    }
}
