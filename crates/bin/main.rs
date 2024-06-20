#![allow(dead_code, unused)]
use std::{
    collections::VecDeque,
    path,
    str::FromStr,
    sync::{
        Arc,
        OnceLock,
    },
    time::Duration,
};

use binan_spot::{
    market::klines::KlineInterval,
    trade::order::{
        Side,
        TimeInForce,
    },
};
use clap::Parser;
use quant_core::{
    actor,
    api::basic::TradeSide,
    init_state,
    message::{
        KlineApiRequest,
        KlineStrategyRequest,
        NewOrderApiRequest,
        NormalRequest,
        TickerApiRequest,
    },
    model::kline::Kline,
    util::{
        config::ConfigBuilder,
        time::{
            DurationInterval,
            GetDuration,
            UtcTimeTool,
        },
    },
    Error,
    QuantState,
    Result,
};
use rust_decimal::{
    prelude::Signed,
    Decimal,
};
use rust_decimal_macros::dec;
use tokio::{
    sync::Mutex,
    time,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: path::PathBuf,
}

fn set_ctrlc_handler() {
    tokio::task::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for event");

        tracing::info!("Ctrl-C received, stop system");

        QuantState::get_addr()
            .send(NormalRequest::Stop)
            .await
            .unwrap();

        tracing::info!("Shutdown now");

        std::process::exit(0);
    });
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args: Cli = Cli::parse();
    let config = ConfigBuilder::build(args.config)?;

    init_state(config).await;
    set_ctrlc_handler();

    actor::run_web()
        .await
        .map_err(|e| Error::Custom(e.to_string()))?;

    Ok(())
}

async fn run_trade() -> Result<()> {
    tokio::spawn(async {
        loop {
            let res = run_impl().await;
            if let Err(e) = res {
                tracing::error!("{:?}", e);
            }

            time::sleep(Duration::from_secs(60)).await;
        }
    });

    Ok(())
}

async fn run_impl() -> Result<()> {
    let symbol = "BTCUSDT";
    let total_currency = dec!(100);

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
    let quantity = total_currency / price;

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
    }

    Ok(())
}
