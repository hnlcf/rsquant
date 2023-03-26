use std::fs;

use binance_spot_connector_rust::http::Credentials;

use super::BinanHmacSignature;

static SIGNATURE_FILE: &str = "binance-signature.json";

pub fn get_credentials() -> Credentials {
    let sig_json_file = fs::File::open(SIGNATURE_FILE).expect("Can't open signature file");
    let sig: BinanHmacSignature =
        serde_json::from_reader(sig_json_file).expect("Can't parse signature file");
    Credentials::from_hmac(sig.api_key(), sig.api_secret())
}
