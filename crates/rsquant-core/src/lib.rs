pub mod actor;
pub mod api;
pub mod db;
mod error;
mod manager;
pub mod message;
pub mod model;
pub mod trade;
pub mod util;

pub type Result<T> = core::result::Result<T, Error>;

use std::{
    str::FromStr,
    time::Duration,
};

pub use actor::run_web;
use binan_spot::{
    market::klines::KlineInterval,
    trade::order::{
        Side,
        TimeInForce,
    },
};
pub use error::Error;
pub use manager::{
    init_state,
    QuantState,
};
use rust_decimal::Decimal;
use tokio::time;
pub use util::config::ConfigBuilder;

use crate::{
    api::basic::TradeSide,
    message::{
        KlineApiRequest,
        KlineStrategyRequest,
        NewOrderApiRequest,
        NormalRequest,
        SendEmailRequest,
        TickerApiRequest,
    },
    util::config::BasicConfig,
};

pub fn set_ctrlc_handler() {
    tokio::task::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for event");

        tracing::info!("Ctrl-C received, stop system");

        if let Err(e) = QuantState::get_addr().send(NormalRequest::Stop).await {
            tracing::error!("Failed to send stop signal to state by: {:?}", e);
        }

        tracing::info!("Shutdown now");

        std::process::exit(0);
    });
}

pub async fn run_trade(config: BasicConfig) -> Result<()> {
    tokio::spawn(async {
        let config = config;
        loop {
            let res = run_impl(&config.symbol, config.total_currency).await;
            if let Err(e) = res {
                tracing::error!("{:?}", e);
            }

            time::sleep(Duration::from_secs(config.duration)).await;
        }
    });

    Ok(())
}

async fn run_impl(symbol: &str, currency: u64) -> Result<()> {
    let total_currency = Decimal::from(currency);

    // 1. Get data
    let data_req = KlineApiRequest {
        symbol: symbol.into(),
        interval: KlineInterval::Minutes1,
        limit: 100,
        start_time: None,
        end_time: None,
    };
    let kline_resp = QuantState::get_addr()
        .send(data_req)
        .await
        .map_err(|e| Error::Custom(e.to_string()))??;

    // 2. Analyze signal
    let kline_data: KlineStrategyRequest = kline_resp.into();
    let signal = QuantState::get_addr()
        .send(kline_data)
        .await
        .map_err(|e| Error::Custom(e.to_string()))??;

    // 3. Send Order
    let ticker_price = QuantState::get_addr()
        .send(TickerApiRequest {
            symbol: symbol.into(),
            interval: 0,
        })
        .await
        .map_err(|e| Error::Custom(e.to_string()))??;
    let price = Decimal::from_str(&ticker_price.ticker.price)?;
    let quantity = (total_currency / price).round_dp(5);

    let order_req_opt = match signal {
        TradeSide::Buy => {
            let req = NewOrderApiRequest {
                symbol: symbol.into(),
                side: Side::Buy,
                r#type: "LIMIT".into(),
                time_in_force: TimeInForce::Gtc,
                quantity,
                price,
            };
            Some(req)
        }
        TradeSide::Sell => {
            let req = NewOrderApiRequest {
                symbol: symbol.into(),
                side: Side::Sell,
                r#type: "LIMIT".into(),
                time_in_force: TimeInForce::Gtc,
                quantity,
                price,
            };
            Some(req)
        }
        TradeSide::Nop => None,
    };

    if let Some(order_req) = order_req_opt {
        let order_res = QuantState::get_addr()
            .send(order_req)
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;

        tracing::debug!("{}", order_res.res);

        QuantState::get_addr()
            .send(SendEmailRequest {
                subject: "Trade Signal".into(),
                content: format!("{:?}: {}", signal, order_res.res),
            })
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;
    }

    Ok(())
}
