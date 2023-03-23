use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Api {
    api_key: String,
    api_secret: String,
}

impl Api {
    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn api_secret(&self) -> String {
        self.api_secret.clone()
    }
}
