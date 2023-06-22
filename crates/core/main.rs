use std::sync::OnceLock;
use std::time::Duration;

use binan_spot::market::klines::KlineInterval;
use clokwerk::{AsyncScheduler, TimeUnits};
use quant_util::time::TimeZoneConverter;

use manager::Manager;

mod api;
mod manager;
mod time;

static MANAGER: OnceLock<Manager> = OnceLock::new();

static ASSETS: [&str; 13] = [
    "BTCUSDT",
    "ETHUSDT",
    "BNBUSDT",
    "XRPUSDT",
    "MDTUSDT",
    "DOGEUSDT",
    "GALAUSDT",
    "MATICUSDT",
    "PERLUSDT",
    "TRUUSDT",
    "CFXUSDT",
    "ARBUSDT",
    "LINAUSDT",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MANAGER.get_or_init(|| {
        let m = Manager::from_config();
        let _ = m.init();
        m
    });

    let mut scheduler = AsyncScheduler::with_tz(chrono::Local);

    for i in ASSETS {
        // Ticker price
        scheduler.every(1.seconds()).run(|| async {
            manager.get_ticker_price(i).await;
        });
        // Kline - 1m
        scheduler.every(1.minutes()).run(|| async {
            let (_, end_unix_time) = time::DateTime::get_local_current();
            let start_unix_time = end_unix_time.to_owned() - 60000;

            manager
                .get_kline(
                    i,
                    KlineInterval::Minutes1,
                    TimeZoneConverter::convert_local_to_utc(start_unix_time),
                    TimeZoneConverter::convert_local_to_utc(end_unix_time),
                )
                .await;
        });
        // Kline - 5m
        scheduler.every(5.minutes()).run(|| async {
            let (_, end_unix_time) = time::DateTime::get_local_current();
            let start_unix_time = end_unix_time.to_owned() - 60000 * 5;

            manager
                .get_kline(
                    i,
                    KlineInterval::Minutes5,
                    TimeZoneConverter::convert_local_to_utc(start_unix_time),
                    TimeZoneConverter::convert_local_to_utc(end_unix_time),
                )
                .await;
        });
        // Kline - 30m
        scheduler.every(30.minutes()).run(|| async {
            let (_, end_unix_time) = time::DateTime::get_local_current();
            let start_unix_time = end_unix_time.to_owned() - 60000 * 30;

            manager
                .get_kline(
                    i,
                    KlineInterval::Minutes30,
                    TimeZoneConverter::convert_local_to_utc(start_unix_time),
                    TimeZoneConverter::convert_local_to_utc(end_unix_time),
                )
                .await;
        });
        // Kline - 1h
        scheduler.every(1.hours()).run(|| async {
            let (_, end_unix_time) = time::DateTime::get_local_current();
            let start_unix_time = end_unix_time.to_owned() - 60000 * 60;

            manager
                .get_kline(
                    i,
                    KlineInterval::Hours1,
                    TimeZoneConverter::convert_local_to_utc(start_unix_time),
                    TimeZoneConverter::convert_local_to_utc(end_unix_time),
                )
                .await;
        });
        // Kline - 4h
        scheduler.every(4.hours()).run(|| async {
            let (_, end_unix_time) = time::DateTime::get_local_current();
            let start_unix_time = end_unix_time.to_owned() - 60000 * 60 * 4;

            manager
                .get_kline(
                    i,
                    KlineInterval::Hours4,
                    TimeZoneConverter::convert_local_to_utc(start_unix_time),
                    TimeZoneConverter::convert_local_to_utc(end_unix_time),
                )
                .await;
        });
        // Kline - 1d
        scheduler.every(1.days()).run(|| async {
            let (_, end_unix_time) = time::DateTime::get_local_current();
            let start_unix_time = end_unix_time.to_owned() - 60000 * 60 * 24;

            manager
                .get_kline(
                    i,
                    KlineInterval::Days1,
                    TimeZoneConverter::convert_local_to_utc(start_unix_time),
                    TimeZoneConverter::convert_local_to_utc(end_unix_time),
                )
                .await;
        });
    }

    let task = tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
    task.await?;

    Ok(())
}
