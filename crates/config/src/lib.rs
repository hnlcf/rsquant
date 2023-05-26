use std::{
    fs::{read_to_string, File},
    path::PathBuf,
};

use quant_util::{constants::DEFAULT_APP_NAME, env::EnvManager};
use serde::{Deserialize, Serialize};

pub fn get_config_path() -> PathBuf {
    let home_dir = EnvManager::get_env_var("HOME").unwrap_or("/home/changfeng".into());
    let xdg_config_home =
        EnvManager::get_env_var("XDG_CONFIG_HOME").unwrap_or(format!("{}/.config", home_dir));

    [&xdg_config_home, DEFAULT_APP_NAME, "config.toml"]
        .iter()
        .collect()
}

pub fn read_config_file(path: PathBuf) -> Option<String> {
    if !path.exists() {
        let config_dir = path.parent()?;
        std::fs::create_dir_all(config_dir).ok()?;
        File::create(path).ok()?;
        None
    } else {
        read_to_string(path).ok()
    }
}

#[derive(Serialize, Deserialize)]
pub struct QuantConfig {
    binan_api_credentials: Option<CredentialsConfig>,
    email: Option<EmailConfig>,
    network: Option<NetworkConfig>,
    log: Option<LogConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkConfig {
    proxy: Option<ProxyConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ProxyConfig {
    https_proxy: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CredentialsConfig {
    signature_type: Option<String>,
    api_key: Option<String>,
    api_secret: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmailConfig {
    from_email: String,
    to_emails: Vec<String>,
    from_passwd: String,
    smtp_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogConfig {
    log_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseConfig {
    postgresql: Option<PostgresqlConfig>,
    sqlite: Option<SqliteConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct PostgresqlConfig {
    pg_addr: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SqliteConfig {
    db_path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config() {
        let actual = get_config_path();
        assert_eq!(
            actual,
            PathBuf::from("/home/changfeng/.config/quant/config.toml")
        );
    }
}
