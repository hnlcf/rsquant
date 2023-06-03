use std::{
    fs::{read_to_string, File},
    path::PathBuf,
};

use quant_util::{constants::DEFAULT_APP_NAME, env::EnvManager};
use serde::{Deserialize, Serialize};

pub struct ConfigBuilder;

impl ConfigBuilder {
    fn get_config_path() -> PathBuf {
        let home_dir = EnvManager::get_env_var("HOME").unwrap_or("/home/changfeng".into());
        let xdg_config_home =
            EnvManager::get_env_var("XDG_CONFIG_HOME").unwrap_or(format!("{}/.config", home_dir));

        [&xdg_config_home, DEFAULT_APP_NAME, "config.toml"]
            .iter()
            .collect()
    }

    fn read_config_file(path: PathBuf) -> Option<String> {
        if !path.exists() {
            let config_dir = path.parent()?;
            std::fs::create_dir_all(config_dir).ok()?;
            File::create(path).ok()?;
            None
        } else {
            read_to_string(path).ok()
        }
    }

    fn parse_config(config: String) -> Result<QuantConfig, Box<dyn std::error::Error>> {
        match toml::from_str::<QuantConfig>(&config) {
            Ok(e) => Ok(e),
            Err(_) => Err("Config parse error!".into()),
        }
    }

    pub fn build() -> Option<QuantConfig> {
        let config_content = ConfigBuilder::read_config_file(ConfigBuilder::get_config_path())?;
        match ConfigBuilder::parse_config(config_content) {
            Ok(c) => Some(c),
            Err(e) => {
                log::error!("Failed to parse config file with {}", e);
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct QuantConfig {
    pub api_credentials: CredentialsConfig,
    pub email: EmailConfig,
    pub network: NetworkConfig,
    pub log: LogConfig,
    pub database: DatabaseConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialsConfig {
    Binance(BinanCredentialsConfig),
    Okx,
}

#[derive(Serialize, Deserialize)]
pub struct BinanCredentialsConfig {
    pub signature_type: String,
    pub api_key: String,
    pub api_secret: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmailConfig {
    pub from_email: String,
    pub to_emails: Vec<String>,
    pub from_passwd: String,
    pub smtp_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkConfig {
    pub proxy: Option<ProxyConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ProxyConfig {
    pub https_proxy: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LogConfig {
    pub log_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseConfig {
    Postgresql(PostgresqlConfig),
    Sqlite(SqliteConfig),
}

#[derive(Serialize, Deserialize)]
pub struct PostgresqlConfig {
    pub pg_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct SqliteConfig {
    pub db_path: String,
}
