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

static ASSETS: [&str; 2] = ["BTCUSDT", "ETHUSDT"];

async fn launch_data_server() -> Result<(), quant_core::Error> {
    let manager = MANAGER.get_or_init(|| {
        let m = Manager::from_config();
        let _ = m.init();
        m
    });

    let mut scheduler = AsyncScheduler::with_tz(chrono::Local);

    for i in ASSETS {
        // Ticker price
        scheduler.every(5.seconds()).run(|| async {
            manager
                .get_ticker_price(i)
                .await
                .expect("Failed to get ticker data");
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
                .await
                .expect("Failed to get kline data");
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
                .await
                .expect("Failed to get kline data");
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
                .await
                .expect("Failed to get kline data");
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
                .await
                .expect("Failed to get kline data");
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
                .await
                .expect("Failed to get kline data");
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
                .await
                .expect("Failed to get kline data");
        });
    }

    let _task = tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    Ok(())
}

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

async fn launch_web_server() -> Result<(), quant_core::Error> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
        .map_err(quant_core::Error::IO)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), quant_core::Error> {
    tokio::task::spawn(async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for event");

        tracing::info!("Ctrl-C received, shutting down");
    });

    let _ = launch_data_server().await;
    let _ = launch_web_server().await;

    Ok(())
}
