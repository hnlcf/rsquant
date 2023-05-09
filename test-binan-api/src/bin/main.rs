use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;

use binance_spot_connector_rust::http::Credentials;
use binance_spot_connector_rust::hyper::BinanceHttpClient;
use binance_spot_connector_rust::hyper::Error as BinanHyperError;
use binance_spot_connector_rust::market::klines::KlineInterval;
use test_binan_api::credential;
use test_binan_api::res;
use test_binan_api::util;

type BinanHttpClient = BinanceHttpClient<HttpsConnector<HttpConnector>>;

lazy_static! {
    static ref CREDENTIALS: Credentials =
        credential::CredentialBuilder::from_env().expect("Failed to get credential from envs.");
    static ref CLIENT: BinanHttpClient = BinanceHttpClient::default();
}

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .debug(Color::White)
        .error(Color::Red)
        .trace(Color::Blue);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file("log/output.log")?)
        .apply()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), BinanHyperError> {
    setup_logger().expect("Can't setup logger");

    let account_info = res::get_account_info(&CLIENT, &CREDENTIALS)
        .await
        .remove_blank_coin();
    log::info!("Account info:\n{}", account_info);

    let kline = get_kline(
        "ETHUSDT",
        KlineInterval::Hours1,
        "2023-05-08 11:00:00",
        "2023-05-09 11:00:00",
    )
    .await;
    log::info!("{:#?}", kline);

    Ok(())
}

async fn get_kline(
    symbol: &str,
    interval: KlineInterval,
    start_time: &str,
    end_time: &str,
) -> Vec<res::KlineRes> {
    let start_time = util::TimeConverter::date_to_unix_time(start_time).unwrap();
    let end_time = util::TimeConverter::date_to_unix_time(end_time).unwrap();
    res::get_kline(&CLIENT, symbol, interval, start_time, end_time, 1000).await
}
