use std::fs;

use serde::Deserialize;

use binance_spot_connector_rust::http::Credentials;

#[derive(Deserialize)]
pub struct CredentialBuilder {
    api_key: String,
    signature: Signature,
}

#[derive(Deserialize)]
#[serde(tag = "signature_type")]
enum Signature {
    Hmac(HmacSignature),
    Rsa(RsaSignature),
}

#[derive(Deserialize)]
struct HmacSignature {
    api_secret: String,
}

#[derive(Deserialize)]
struct RsaSignature {
    key: String,
    password: Option<String>,
}

impl CredentialBuilder {
    /// Get Credentials from json file.
    ///
    /// Read your local credential json file passed in as `credential_file` and return a [`Credentials`].
    ///
    /// ## Examples
    ///
    /// Your local credential file need to be as below.
    ///
    /// ```json
    /// // Hmac
    /// {
    ///     "api_key": "xxx",
    ///     "signature": {
    ///         "signature_type": "Hmac",
    ///         "api_secret": "xxx",
    ///     }
    /// }
    /// // Rsa
    /// {
    ///     "api_key": "xxx",
    ///     "signature": {
    ///         "signature_type": "Rsa",
    ///         "key": "xxx",
    ///         "password": "xxx", // Maybe absent
    ///     }
    /// }
    /// ```
    pub fn from_json(json_file: impl Into<String>) -> Option<Credentials> {
        let json_reader = fs::File::open(json_file.into()).expect("Can't open credential file");
        let builder: CredentialBuilder =
            serde_json::from_reader(json_reader).expect("Can't parse credential file");
        match builder.signature {
            Signature::Hmac(sig) => Some(Credentials::from_hmac(builder.api_key, sig.api_secret)),
            Signature::Rsa(sig) => match sig.password {
                Some(password) => Some(Credentials::from_rsa_protected(
                    builder.api_key,
                    sig.key,
                    password,
                )),
                None => Some(Credentials::from_rsa(builder.api_key, sig.key)),
            },
        }
    }
}
