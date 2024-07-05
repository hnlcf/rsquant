use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

#[tokio::main]
async fn main() {
    let filter_layer = EnvFilter::try_from_env("QUANT_LOG_LEVEL")
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let tui_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_ansi(true)
        .with_file(false)
        .with_line_number(true)
        .with_thread_names(false)
        .with_thread_ids(false)
        .with_writer(std::io::stdout);

    tracing_subscriber::registry()
        .with(tui_layer)
        .with(filter_layer)
        .init();

    rsquant_bt::run_bt().await.unwrap();
}
