#![feature(async_closure)]
mod api;
mod manager;
mod task;
mod time;
mod trade;

use manager::Manager;

use clokwerk::{AsyncScheduler, TimeUnits};
use lazy_static::lazy_static;

use std::sync::Arc;
use std::time::Duration;

lazy_static! {
    static ref MANAGER: Arc<Manager> = Arc::new(Manager::default());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scheduler = AsyncScheduler::with_tz(chrono::Local);

    MANAGER.init()?;
    MANAGER.get_account_info().await;

    scheduler.every(5.seconds()).run(get_eth_price);

    let task = tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
    task.await?;

    Ok(())
}

async fn get_eth_price() {
    let (date_time, unix_time) = time::DateTime::get_current();
    let eth_price = MANAGER.get_ticker_price("ETHUSDT").await;
    MANAGER.recorder().record_ticker_price_data(
        &["name", "price", "unix_time", "date_time"],
        (&eth_price.symbol, &eth_price.price, &unix_time, &date_time),
    );
}
