use std::env;

use binance_spot_connector_rust::http::Credentials;

pub struct CredentialBuilder;

impl CredentialBuilder {
    /// Get Credentials from environment varibales.
    ///
    /// ## Examples
    ///
    /// Your local envs need to be set as below.
    ///
    /// ```bash
    /// # Hmac
    /// BINAN_SIG_TYPE=HMAC
    /// BINAN_API_KEY=xxx
    /// BINAN_API_SECRET=xxx
    ///
    /// # Rsa
    /// BINAN_SIG_TYPE=RSA
    /// BINAN_API_KEY=xxx
    /// BINAN_SIG_KEY=xxx
    /// BINAN_SIG_PASSWD=xxx # Maybe absent
    /// ```
    pub fn from_env() -> Option<Credentials> {
        let sig_type = CredentialBuilder::get_env_var("BINAN_SIG_TYPE")?;
        let api_key = CredentialBuilder::get_env_var("BINAN_API_KEY")?;
        match sig_type.as_str() {
            "HMAC" => {
                let api_secret = CredentialBuilder::get_env_var("BINAN_API_SECRET")?;
                let hmac_credential = Credentials::from_hmac(api_key, api_secret);
                Some(hmac_credential)
            }
            "RSA" => {
                let sig_key = CredentialBuilder::get_env_var("BINAN_SIG_KEY")?;
                let sig_passwd = CredentialBuilder::get_env_var("BINAN_SIG_PASSWD");
                let rsa_credential = match sig_passwd {
                    Some(sig_passwd) => {
                        Credentials::from_rsa_protected(api_key, sig_key, sig_passwd)
                    }
                    None => Credentials::from_rsa(api_key, sig_key),
                };

                Some(rsa_credential)
            }
            _ => None,
        }
    }

    fn get_env_var(key: &str) -> Option<String> {
        env::var(key).ok().map_or_else(
            || {
                log::error!("Environment variable `{}` is unset!", key);
                None
            },
            Some,
        )
    }
}
