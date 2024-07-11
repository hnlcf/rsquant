pub mod actor;
pub mod api;
pub mod db;
pub mod entity;
mod error;
mod manager;
pub mod message;
pub mod model;
mod monitor;
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
pub use monitor::run_monitor;
use rust_decimal::Decimal;
use tokio::time;
pub use util::config::ConfigBuilder;

pub use crate::trade::{
    CommonMacdAndRsiStrategy,
    DoubleEmaStrategy,
};
use crate::{
    entity::{
        order,
        side,
    },
    message::{
        KlineApiRequest,
        KlineApiResponse,
        KlineStrategyRequest,
        NewOrderApiRequest,
        NormalRequest,
        RecordOrderRequest,
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
            let res = run_trade_impl(&config.symbol, config.total_currency).await;
            if let Err(e) = res {
                tracing::error!("{:?}", e);
            }

            time::sleep(Duration::from_secs(config.duration)).await;
        }
    });

    Ok(())
}

async fn run_trade_impl(symbol: &str, currency: u64) -> Result<()> {
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
    let kline_data = KlineStrategyRequest::from_klines("common", kline_resp);
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
        side::TradeSide::Buy => {
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
        side::TradeSide::Sell => {
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
        side::TradeSide::Nop => None,
    };

    if let Some(order_req) = order_req_opt {
        let order_res = QuantState::get_addr()
            .send(order_req)
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;

        let order_res = order_res.res;
        tracing::debug!("{:?}", order_res);

        let record_req = RecordOrderRequest {
            model: order::Model {
                order_id: order_res.order_id,
                symbol: symbol.into(),
                price,
                quantity,
                side: signal,
                time_in_force: order_res.time_in_force,
                r#type: order_res.r#type,
                ..Default::default()
            },
        };

        let record_res = QuantState::get_addr()
            .send(record_req)
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;

        QuantState::get_addr()
            .send(SendEmailRequest {
                subject: format!("Trade Signal for {}", symbol),
                content: format!("Record:\n{:?}", record_res),
            })
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;
    }

    Ok(())
}
