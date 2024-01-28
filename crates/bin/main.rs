use std::sync::OnceLock;
use std::time::Duration;

use manager::QuantState;

use self::message::ApiRequest;

mod api;
mod manager;
mod message;
mod time;

static STATE: OnceLock<QuantState> = OnceLock::new();

static ASSETS: [&str; 2] = ["BTCUSDT", "ETHUSDT"];

#[actix::main]
async fn main() -> Result<(), quant_core::Error> {
    tokio::task::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for event");

        tracing::info!("Ctrl-C received, shutting down");

        STATE.get().unwrap().stop().await;

        tracing::info!("Shutdown now");
    });
    let manager = STATE.get_or_init(|| {
        let mut m = QuantState::from_config().expect("Failed to create manager");
        let _ = m.init();
        m
    });

    'out: loop {
        tokio::time::sleep(Duration::from_secs(2)).await;

        for i in ASSETS {
            match manager
                .get_info(ApiRequest::Ticker {
                    symbol: i.to_owned(),
                })
                .await
            {
                Ok(res) => {
                    tracing::info!("{}", res);
                }
                Err(e) => {
                    tracing::warn!("{}", e);
                    break 'out;
                }
            }
        }
    }

    Ok(())
}
