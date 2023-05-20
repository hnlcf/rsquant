#![feature(async_closure)]
mod api;
mod manager;
mod task;
mod time;
mod trade;

use manager::Manager;

use clokwerk::{AsyncScheduler, TimeUnits};
use lazy_static::lazy_static;
use quant_util::time::TimeZoneConverter;

use std::sync::Arc;
use std::time::Duration;

lazy_static! {
    static ref MANAGER: Arc<Manager> = Arc::new(Manager::default());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scheduler = AsyncScheduler::with_tz(chrono::Local);

    MANAGER.init()?;

    scheduler.every(5.seconds()).run(|| async {
        let assets = vec!["ETHUSDT", "BTCUSDT"];
        for i in assets {
            MANAGER.get_ticker_price(i).await;
        }
    });
    scheduler.every(5.minutes()).run(|| async {
        let (_, end_unix_time) = time::DateTime::get_local_current();
        let start_unix_time = end_unix_time - 60000 * 5;

        MANAGER
            .get_kline(
                "ETHUSDT",
                binan_spot::market::klines::KlineInterval::Minutes1,
                TimeZoneConverter::convert_local_to_utc(start_unix_time),
                TimeZoneConverter::convert_local_to_utc(end_unix_time),
            )
            .await;
    });

    let task = tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
    task.await?;

    Ok(())
}
