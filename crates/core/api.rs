use binan_spot::{http::Credentials, market::klines::KlineInterval};
use quant_api::res::api::GetResponse;
use quant_api::{credential, res, res::BinanHttpClient};

pub struct Api {
    credentials: Credentials,
    client: BinanHttpClient,
}

impl Default for Api {
    fn default() -> Self {
        Self {
            credentials: credential::CredentialBuilder::from_env()
                .expect("Failed to create credential from envs."),
            client: BinanHttpClient::default(),
        }
    }
}

impl Api {
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
    pub async fn get_account_info(&self) -> res::account_info::AccountInfoRes {
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
    pub async fn get_ticker_price(&self, symbol: &str) -> res::ticker_price::TickerPriceRes {
        let price = GetResponse::get_ticker_price(&self.client, symbol).await;

        log::info!("Get ticker price of {}: {}", symbol, price);
        price
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
    ) -> Vec<res::kline::KlineRes> {
        let klines =
            GetResponse::get_kline(&self.client, symbol, interval, start_time, end_time, 1000)
                .await;

        for i in &klines {
            log::info!("{}", i);
        }

        klines
    }
}
