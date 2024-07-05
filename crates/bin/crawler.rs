use actix::Actor;
use barter_data::subscription::candle::Candle;
use binan_spot::market::klines::KlineInterval;
use clap::Parser;
use rsquant_core::{
    actor::BinanApiActor,
    message::KlineApiRequest,
    model::kline::Kline,
    util::config::QuantConfig,
    ConfigBuilder,
    Result,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: std::path::PathBuf,
}

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args: Cli = Cli::parse();
    let config = ConfigBuilder::build(args.config)?;

    let QuantConfig {
        api_credentials, ..
    } = config;

    let api = BinanApiActor::from_config(api_credentials).start();

    let req = KlineApiRequest {
        symbol: "BTCUSDT".to_string(),
        interval: KlineInterval::Days1,
        limit: 1000,
        start_time: None,
        end_time: None,
    };

    if let Ok(Ok(res)) = api.send(req).await {
        let kline = res.klines.into_iter().map(convert).collect::<Vec<_>>();

        // serialize klines and write to file
        let klines = serde_json::to_string(&kline).unwrap();
        std::fs::write("fixture/data/btcusdt_candles_1d.json", klines).unwrap();
    }

    Ok(())
}

fn convert(k: Kline) -> Candle {
    Candle {
        close_time: k.close_time.and_utc(),
        open: k.open_price.parse().unwrap(),
        high: k.high_price.parse().unwrap(),
        low: k.low_price.parse().unwrap(),
        close: k.close_price.parse().unwrap(),
        volume: k.volume.parse().unwrap(),
        trade_count: k.trades_num as u64,
    }
}
