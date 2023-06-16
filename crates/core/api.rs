use binan_spot::{http::Credentials, market::klines::KlineInterval};
use quant_api::res::GetResponse;
use quant_api::{credential, res::BinanHttpClient};
use quant_config::{CredentialsConfig, NetworkConfig};
use quant_model::{account_info, kline, ticker_price};
use quant_util::env::EnvManager;

pub struct Api {
    credentials: Credentials,
    client: BinanHttpClient,
}

impl Api {
    pub fn from_config(credentials: CredentialsConfig, network: NetworkConfig) -> Self {
        match credentials {
            CredentialsConfig::Binance(binan_credentials) => {
                let credentials = credential::CredentialBuilder::from_config(binan_credentials)
                    .expect("Failed to get credentials from config file.");
                match network.proxy {
                    Some(proxy_config) => {
                        let proxy_uri = proxy_config.https_proxy.unwrap_or("".into());
                        let client = BinanHttpClient::default_with_proxy(&proxy_uri);

                        Self {
                            credentials,
                            client,
                        }
                    }
                    None => Api::default_with_proxy(),
                }
            }
            _ => Api::default_with_proxy(),
        }
    }

    pub fn default_with_proxy() -> Self {
        let proxy = EnvManager::get_env_var_or("https_proxy", "");
        Self {
            credentials: credential::CredentialBuilder::from_env()
                .expect("Failed to create credential from envs."),
            client: BinanHttpClient::default_with_proxy(&proxy),
        }
    }

    pub async fn get_account_snapshot(&self) -> String {
        GetResponse::get_account_snapshot(&self.client).await
    }

    /// # Get account information
    ///
    /// ## Examples
    ///
    /// ```
    /// let api = Api::default();
    /// let account_info = api.get_account_info().await;
    ///
    /// println!("{:#?}", account_info);
    /// ```
    pub async fn get_account_info(&self) -> account_info::AccountInfo {
        let account_info = GetResponse::get_account_info(&self.client, &self.credentials)
            .await
            .remove_blank_coin();

        log::info!("Get account info:\n{}", account_info);
        account_info
    }

    /// # Get ticker price
    ///
    /// ## Examples
    ///
    /// ```
    /// let api = Api::default();
    /// let price = api.get_ticker_price("ETHUSDT").await;
    ///
    /// println!("{:#?}", price);
    /// ```
    pub async fn get_ticker_price(&self, symbol: &str) -> ticker_price::TickerPrice {
        let ticker_price = GetResponse::get_ticker_price(&self.client, symbol).await;

        log::info!("Get ticker price of {}: {}", symbol, ticker_price.price);
        ticker_price
    }

    /// # Get Kline data
    ///
    /// ## Examples
    ///
    /// ```
    /// let api = Api::default();
    /// let start_time = time::TimeTool::convert_to_unix_time("2023-05-08 11:00:00").unwrap();
    /// let end_time = time::TimeTool::convert_to_unix_time("2023-05-09 11:00:00").unwrap();
    ///
    /// let kline = api.get_kline(
    ///     "ETHUSDT",
    ///     KlineInterval::Hours1,
    ///     start_time,
    ///     end_time,
    /// )
    /// .await;
    ///
    /// println!("{:#?}", kline);
    /// ```
    pub async fn get_kline(
        &self,
        symbol: &str,
        interval: KlineInterval,
        start_time: u64,
        end_time: u64,
    ) -> Vec<kline::Kline> {
        let klines =
            GetResponse::get_kline(&self.client, symbol, interval, start_time, end_time, 1000)
                .await;

        for i in &klines {
            log::info!("{}", i);
        }

        klines
    }
}
