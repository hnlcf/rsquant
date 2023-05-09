#![allow(dead_code)]
use std::{thread, time::Duration};

use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use rusqlite::{Connection, Result};

use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error as BinanHyperError},
    market::klines::KlineInterval,
};
use test_binan_api::{
    credential, res,
    util::{
        self,
        time::{CurrentTime, TimeConverter},
    },
};

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
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file)?)
        .apply()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), BinanHyperError> {
    setup_logger().expect("Can't setup logger");

    let conn = Connection::open("bitcoin.db").expect("Can't open database in sqlite");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS assets_price (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            price       TEXT NOT NULL,
            unix_time   INTEGER NOT NULL,
            date_time   TEXT NOT NULL
         )",
        (),
    )
    .expect("Can't create table in sqlite");

    let account_info = get_account_info().await;
    log::info!("Account info:\n{}", account_info);

    let seconds = Duration::from_secs(2);
    let mut count = 0;

    loop {
        if count > 10000 {
            break;
        }
        count += 1;

        thread::sleep(seconds);

        let current_date_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let current_unix_time = TimeConverter::date_to_unix_time(&current_date_time).unwrap_or(0);
        let eth_price = get_ticker_price("ETHUSDT").await;
        conn.execute(
            "INSERT INTO assets_price (name, price, unix_time, date_time) VALUES (?1, ?2, ?3, ?4)",
            (
                &eth_price.symbol,
                &eth_price.price,
                &current_unix_time,
                &current_date_time,
            ),
        )
        .expect("Can't insert ticker price into sqlite");

        log::info!("{:?}", eth_price);
    }

    Ok(())
}

async fn get_ticker_price(symbol: &str) -> res::ticker_price::TickerPriceRes {
    res::api::get_ticker_price(&CLIENT, symbol).await
}

/// # Get account information
///
/// ## Examples
///
/// ```
/// let account_info = get_account_info().await;
/// println!("{:#?}", account_info);
/// ```
async fn get_account_info() -> res::account_info::AccountInfoRes {
    res::api::get_account_info(&CLIENT, &CREDENTIALS)
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
