use clap::Parser;
use rsquant_core::{
    ConfigBuilder,
    Error,
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
    let args: Cli = Cli::parse();
    let config = ConfigBuilder::build(args.config)?;

    let gen_symbols = || {
        vec![
            "BTCUSDT".into(),
            "ETHUSDT".into(),
            "BNBUSDT".into(),
            "SOLUSDT".into(),
            "PEPEUSDT".into(),
            "XRPUSDT".into(),
            "DOGEUSDT".into(),
            "SHIBUSDT".into(),
            "ADAUSDT".into(),
            "TRXUSDT".into(),
            "AVAXUSDT".into(),
            "WBTCUSDT".into(),
            "DOTUSDT".into(),
            "LINKUSDT".into(),
            "BCHUSDT".into(),
            "DAIUSDT".into(),
            "MATICUSDT".into(),
            "LTCUSDT".into(),
            "ETCUSDT".into(),
            "PEOPLE".into(),
            "TON".into(),
            "NOT".into(),
            "ONDO".into(),
            "AXL".into(),
            "AEVO".into(),
            "WIF".into(),
        ]
    };
    let gen_strategy = || rsquant_core::DoubleEmaStrategy::new(20, 60);

    rsquant_core::init_state(config.clone(), gen_strategy).await;
    rsquant_core::set_ctrlc_handler();
    tokio::spawn(async move {
        rsquant_core::run_monitor(gen_symbols)
            .await
            .expect("Failed to run monitor");
    });
    rsquant_core::run_web()
        .await
        .map_err(|e| Error::Custom(e.to_string()))?;

    Ok(())
}
