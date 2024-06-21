use std::{
    fs,
    path,
};

use binan_spot::market::klines::KlineInterval;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    Error,
    Result,
};

pub struct ConfigBuilder;

impl ConfigBuilder {
    pub fn build(path: path::PathBuf) -> Result<QuantConfig> {
        let content = fs::read_to_string(path).map_err(Error::from)?;
        serde_json::from_str::<QuantConfig>(&content).map_err(Error::from)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuantConfig {
    pub basic: BasicConfig,
    pub api_credentials: CredentialsConfig,
    pub email: EmailConfig,
    pub network: NetworkConfig,
    pub log: LogConfig,
    pub database: DatabaseConfig,
    pub market: MarketConfig,
    pub strategy: StrategyConfig,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BasicConfig {
    pub symbol: String,
    pub interval: KlineInterval,
    pub total_currency: u64,
    pub duration: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CredentialsConfig {
    Binance(BinanCredentialsConfig),
    #[default]
    Okx,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BinanCredentialsConfig {
    pub signature_type: String,
    pub api_key: String,
    pub api_secret: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub from_email: String,
    pub to_emails: Vec<String>,
    pub from_passwd: String,
    pub smtp_addr: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub proxy: Option<ProxyConfig>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub https_proxy: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub log_path: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub db_url: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PostgresqlConfig {
    pub pg_addr: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SqliteConfig {
    pub db_path: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MarketData {
    #[default]
    Kline,
    Ticker,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MarketConfig {
    pub data: MarketData,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StrategySignal {
    #[default]
    Macd,
    Rsi,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub signal: StrategySignal,
}
