use binan_spot::http::Credentials;

use crate::util::{
    config,
    env,
};

pub struct CredentialBuilder;

impl CredentialBuilder {
    pub fn from_config(config: config::BinanCredentialsConfig) -> Option<Credentials> {
        let sig_type = config.signature_type;
        let api_key = config.api_key;
        let api_secret = config.api_secret?;
        match sig_type.as_str() {
            "HMAC" => Some(Credentials::from_hmac(api_key, api_secret)),
            _ => None,
        }
    }

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
        let sig_type = env::EnvManager::get_env_var("BINAN_SIG_TYPE")?;
        let api_key = env::EnvManager::get_env_var("BINAN_API_KEY")?;
        match sig_type.as_str() {
            "HMAC" => {
                let api_secret = env::EnvManager::get_env_var("BINAN_API_SECRET")?;
                let hmac_credential = Credentials::from_hmac(api_key, api_secret);
                Some(hmac_credential)
            }
            "RSA" => {
                let sig_key = env::EnvManager::get_env_var("BINAN_SIG_KEY")?;
                let sig_passwd = env::EnvManager::get_env_var("BINAN_SIG_PASSWD");
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
