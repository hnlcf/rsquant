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

    let gen_strategy = || rsquant_core::CommonMacdAndRsiStrategy::new(12, 26, 9, 14, 30.0, 70.0);
    rsquant_core::init_state(config.clone(), gen_strategy).await;
    rsquant_core::set_ctrlc_handler();
    rsquant_core::run_trade(config.basic).await?;
    rsquant_core::run_web()
        .await
        .map_err(|e| Error::Custom(e.to_string()))?;

    Ok(())
}
