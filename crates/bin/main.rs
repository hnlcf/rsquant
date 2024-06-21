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

    rsquant_core::init_state(config.clone()).await;
    rsquant_core::set_ctrlc_handler();
    rsquant_core::run_trade(config.basic).await?;
    rsquant_core::run_web()
        .await
        .map_err(|e| Error::Custom(e.to_string()))?;

    Ok(())
}
