use std::time::Duration;

use clokwerk::{
    AsyncScheduler,
    Interval::*,
    Job,
    TimeUnits,
};
use itertools::Itertools;

use crate::{
    entity::side::TradeSide,
    util::{
        email,
        time::{
            CurrentTime,
            LocalTimeTool,
        },
    },
    Error,
    KlineApiRequest,
    KlineApiResponse,
    KlineInterval,
    KlineStrategyRequest,
    QuantState,
    Result,
    SendEmailRequest,
};

async fn send_kline_req(symbol: &str, interval: KlineInterval) -> Result<KlineApiResponse> {
    let data_req = KlineApiRequest {
        symbol: symbol.into(),
        interval,
        limit: 100,
        start_time: None,
        end_time: None,
    };
    let kline_resp = QuantState::get_addr()
        .send(data_req)
        .await
        .map_err(|e| Error::Custom(e.to_string()))??;

    Ok(kline_resp)
}

async fn compute_trend_signal_impl(symbol: &str, interval: KlineInterval) -> Result<TradeSide> {
    let kline_resp = send_kline_req(symbol, interval).await?;

    let kline_data: KlineStrategyRequest = kline_resp.into();
    let signal = QuantState::get_addr()
        .send(kline_data)
        .await
        .map_err(|e| Error::Custom(e.to_string()))??;

    Ok(signal)
}

async fn check_trend_signals(symbols: &[String], interval: KlineInterval) -> Result<()> {
    let mut res = vec![];
    for symbol in symbols {
        let signal = compute_trend_signal_impl(symbol, interval).await?;
        res.push((symbol.to_string(), signal));
    }

    let mut buy_symbols = vec![];
    let mut sell_symbols = vec![];
    for (key, chunk) in &res.into_iter().chunk_by(|(_, signal)| *signal) {
        match key {
            TradeSide::Buy => buy_symbols.extend(chunk.map(|(s, _)| s)),
            TradeSide::Sell => sell_symbols.extend(chunk.map(|(s, _)| s)),
            TradeSide::Nop => {}
        }
    }

    if !buy_symbols.is_empty() || !sell_symbols.is_empty() {
        let email_ctx = email::EmailMonitorReportContext {
            datetime: LocalTimeTool::get_date_time(),
            interval: interval.to_string(),
            headers: vec!["标的物".into()],
            buy_symbols,
            sell_symbols,
        };

        let email = email::generate_monitor_report(email_ctx)?;

        QuantState::get_addr()
            .send(SendEmailRequest {
                subject: format!("Rsquant {} 趋势跟踪", interval),
                content: email,
            })
            .await
            .map_err(|e| Error::Custom(e.to_string()))??;

        tracing::info!("Send {} monitor email", interval);
    }

    Ok(())
}

pub async fn run_monitor<F>(gen_symbols: F) -> Result<()>
where
    F: Fn() -> Vec<String> + Send,
{
    tracing::info!("Start monitor");

    let symbols = Box::leak(Box::new(gen_symbols()));
    // Create a new scheduler
    let mut scheduler = AsyncScheduler::new();

    // 15m
    scheduler.every(15.minutes()).run(|| async {
        let _res = check_trend_signals(symbols, KlineInterval::Minutes15).await;
    });

    // 30m
    scheduler.every(30.minutes()).run(|| async {
        let _res = check_trend_signals(symbols, KlineInterval::Minutes30).await;
    });

    // 1h
    scheduler.every(1.hours()).run(|| async {
        let _res = check_trend_signals(symbols, KlineInterval::Hours1).await;
    });

    // 4h
    scheduler.every(4.hours()).run(|| async {
        let _res = check_trend_signals(symbols, KlineInterval::Hours4).await;
    });

    // 8h
    scheduler.every(8.hours()).run(|| async {
        let _res = check_trend_signals(symbols, KlineInterval::Hours8).await;
    });

    // 12h
    scheduler.every(12.hours()).run(|| async {
        let _res = check_trend_signals(symbols, KlineInterval::Hours12).await;
    });

    // 1d
    scheduler.every(1.day()).at("8:00 am").run(|| async {
        let _res = check_trend_signals(symbols, KlineInterval::Days1).await;
    });

    // 3d
    scheduler
        .every(Wednesday)
        .at("8:00")
        .and_every(Saturday)
        .at("8:00")
        .run(|| async {
            let _res = check_trend_signals(symbols, KlineInterval::Days3).await;
        });

    // 1w
    scheduler.every(Sunday).at("8:00").run(|| async {
        let _res = check_trend_signals(symbols, KlineInterval::Weeks1).await;
    });

    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(5000)).await;
    }

    Ok(())
}
