use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

use binance_spot_connector_rust::{
    http::Credentials, hyper::BinanceHttpClient, market::klines::KlineInterval,
};
use test_binan_api::{credential, db::recorder::Recorder, res, util};

type BinanHttpClient = BinanceHttpClient<HttpsConnector<HttpConnector>>;

pub struct Event;

#[derive(Default)]
pub struct Executor {
    events: Vec<Event>,
}

pub struct Manager {
    credentials: Credentials,
    client: BinanHttpClient,
    recorder: Recorder,
    executor: Executor,
}

impl Default for Manager {
    fn default() -> Self {
        Self {
            credentials: credential::CredentialBuilder::from_env().expect(""),
            client: BinanHttpClient::default(),
            recorder: Recorder::default(),
            executor: Executor::default(),
        }
    }
}

impl Manager {
    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        util::log::Logger::setup_logger()?;

        self.recorder.init();

        Ok(())
    }

    pub fn recorder(&self) -> &Recorder {
        &self.recorder
    }

    pub async fn get_ticker_price(&self, symbol: &str) -> res::ticker_price::TickerPriceRes {
        let price = res::api::get_ticker_price(&self.client, symbol).await;

        log::info!("{}", price);
        price
    }

    /// # Get account information
    ///
    /// ## Examples
    ///
    /// ```
    /// let account_info = get_account_info().await;
    /// println!("{:#?}", account_info);
    /// ```
    pub async fn get_account_info(&self) -> res::account_info::AccountInfoRes {
        res::api::get_account_info(&self.client, &self.credentials)
            .await
            .remove_blank_coin()
    }

    /// # Get Kline data
    ///
    /// ## Examples
    ///
    /// ```
    /// let kline = get_kline(
    ///     "ETHUSDT",
    ///     KlineInterval::Hours1,
    ///     "2023-05-08 11:00:00",
    ///     "2023-05-09 11:00:00",
    /// )
    /// .await;
    /// println!("{:#?}", kline);
    /// ```
    pub async fn get_kline(
        &self,
        symbol: &str,
        interval: KlineInterval,
        start_time: &str,
        end_time: &str,
    ) -> Vec<res::kline::KlineRes> {
        let start_time = util::time::TimeTool::convert_to_unix_time(start_time).unwrap();
        let end_time = util::time::TimeTool::convert_to_unix_time(end_time).unwrap();
        res::api::get_kline(&self.client, symbol, interval, start_time, end_time, 1000).await
    }
}
