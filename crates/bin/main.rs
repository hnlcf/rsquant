#![allow(dead_code, unused)]
use std::{
    collections::VecDeque,
    path,
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
    init_state,
    message::{
        KlineApiRequest,
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
};
use rust_decimal::prelude::Signed;
use rust_decimal_macros::dec;
use tokio::sync::Mutex;

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
async fn main() -> Result<(), quant_core::Error> {
    let args: Cli = Cli::parse();
    let config = ConfigBuilder::build(args.config)?;

    init_state(config).await;
    set_ctrlc_handler();

    actor::run_web()
        .await
        .map_err(|e| Error::Custom(e.to_string()))?;

    // run().await
    Ok(())
}

async fn run() -> Result<(), quant_core::Error> {
    let usdt = "USDT";
    let btc = "BTC";
    let symbol = "BTCUSDT";

    let total = dec!(100.0);
    let mut profit = dec!(0.0);

    let price_slot = Arc::new(Mutex::new(dec!(1.0)));

    let price = price_slot.clone();
    // tokio::task::spawn(async move {
    loop {
        // let ticker = manager
        //     .get_ticker(TickerApiRequest {
        //         symbol: symbol.to_owned(),
        //         interval: 0,
        //     })
        //     .await
        //     .unwrap();
        // tracing::info!("Ticker: {}: {}", ticker.symbol, ticker.price());
        // *price.lock().await = ticker.price();
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    // });

    // 'out: loop {
    //     let (start, end) = UtcTimeTool.get_duration(DurationInterval::Minutes1, 750);
    //     match manager
    //         .get_kline(KlineApiRequest {
    //             symbol: symbol.to_owned(),
    //             interval: KlineInterval::Minutes15,
    //             start_time: start,
    //             end_time: end,
    //             limit: 50,
    //         })
    //         .await
    //     {
    //         Ok(res) => {
    //             let signal = handle_klines_with_macd(&res);
    //             tracing::info!(
    //                 "Signal: {}",
    //                 signal.map(|s| s.to_string()).unwrap_or("Nil".to_string())
    //             );

    //             let origin_price = *price_slot.lock().await;
    //             tracing::info!("Ticker of {}: {}", symbol, origin_price);

    //             let account_info = manager.get_account_info().await?;
    //             let usdt_count = account_info.query_asset(usdt);
    //             let btc_count = account_info.query_asset(btc);
    //             match signal {
    //                 Some(Side::Buy) if usdt_count.is_some() => {
    //                     if usdt_count.unwrap() >= total {
    //                         let price = (origin_price + dec!(1.0)).round_dp(5);
    //                         let quantity = (total / price).round_dp(5);

    //                         let res = manager
    //                             .new_order(NewOrderApiRequest {
    //                                 symbol: symbol.to_owned(),
    //                                 side: Side::Buy,
    //                                 r#type: "LIMIT".to_owned(),
    //                                 time_in_force: TimeInForce::Gtc,
    //                                 quantity,
    //                                 price,
    //                             })
    //                             .await?;

    //                         tracing::info!("Order res: {}", res);
    //                     }
    //                 }
    //                 Some(Side::Sell) if btc_count.is_some() => {
    //                     if btc_count.unwrap() >= total / origin_price {
    //                         let price = (origin_price - dec!(1.0)).round_dp(5);
    //                         let quantity = (total / price).round_dp(5);

    //                         let res = manager
    //                             .new_order(NewOrderApiRequest {
    //                                 symbol: symbol.to_owned(),
    //                                 side: Side::Sell,
    //                                 r#type: "LIMIT".to_owned(),
    //                                 time_in_force: TimeInForce::Gtc,
    //                                 quantity,
    //                                 price,
    //                             })
    //                             .await?;

    //                         tracing::info!("Order res: {}", res);

    //                         let money = account_info.query_asset(usdt).unwrap_or_default()
    //                             + account_info.query_asset(btc).unwrap_or_default() * origin_price;
    //                         tracing::info!("Money: {}", money);
    //                     }
    //                 }
    //                 _ => {}
    //             }
    //         }
    //         Err(e) => {
    //             tracing::warn!("{}", e);
    //             break 'out;
    //         }
    //     }

    //     tokio::time::sleep(Duration::from_secs(60 * 15)).await;
    // }

    Ok(())
}
