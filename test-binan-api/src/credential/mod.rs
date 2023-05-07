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
        let sig_type = env::var("BINAN_SIG_TYPE").ok().map_or_else(
            || {
                log::error!("Environment variable `BINAN_SIG_TYPE` is unset!");
                None
            },
            Some,
        )?;
        let api_key = env::var("BINAN_API_KEY").ok().map_or_else(
            || {
                log::error!("Environment variable `BINAN_API_KEY` is unset!");
                None
            },
            Some,
        )?;

        match sig_type.as_str() {
            "HMAC" => {
                let api_secret = env::var("BINAN_API_SECRET").ok().map_or_else(
                    || {
                        log::error!("Environment variable `BINAN_API_SECRET` is unset!");
                        None
                    },
                    Some,
                )?;

                let hmac_credential = Credentials::from_hmac(api_key, api_secret);
                Some(hmac_credential)
            }
            "RSA" => {
                let sig_key = env::var("BINAN_SIG_KEY").ok().map_or_else(
                    || {
                        log::error!("Environment variable `BINAN_SIG_KEY` is unset!");
                        None
                    },
                    Some,
                )?;

                let sig_passwd = env::var("BINAN_SIG_PASSWD").ok();
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
}
