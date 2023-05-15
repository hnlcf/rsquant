mod api;
mod manager;
mod time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = manager::Manager::default();
    manager.init()?;

    manager.get_account_info().await;

    let seconds = std::time::Duration::from_secs(2);
    for _ in 0..2000 {
        std::thread::sleep(seconds);

        let (date_time, unix_time) = time::DateTime::get_current();
        let eth_price = manager.get_ticker_price("ETHUSDT").await;
        manager.recorder().record_ticker_price_data(
            &["name", "price", "unix_time", "date_time"],
            (&eth_price.symbol, &eth_price.price, &unix_time, &date_time),
        );
    }

    Ok(())
}
