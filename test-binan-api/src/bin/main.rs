use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;

use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error as BinanHyperError},
    market::klines::KlineInterval,
};
use test_binan_api::{credential, res, util};

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
    let log_file =
        util::env::EnvManager::get_env_var("BINAN_LOG_FILE").unwrap_or("log/output.log".into());
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
        .chain(fern::log_file(log_file)?)
        .apply()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), BinanHyperError> {
    setup_logger().expect("Can't setup logger");

    let account_info = res::api::get_account_info(&CLIENT, &CREDENTIALS)
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
) -> Vec<res::kline::KlineRes> {
    let start_time = util::time::TimeConverter::date_to_unix_time(start_time).unwrap();
    let end_time = util::time::TimeConverter::date_to_unix_time(end_time).unwrap();
    res::api::get_kline(&CLIENT, symbol, interval, start_time, end_time, 1000).await
}
