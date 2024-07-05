use std::{
    collections::HashMap,
    fs,
    sync::Arc,
};

use barter::{
    data::historical,
    engine::{
        trader::Trader,
        Engine,
    },
    event::{
        Event,
        EventTx,
    },
    execution::{
        simulated::{
            Config as ExecutionConfig,
            SimulatedExecution,
        },
        Fees,
    },
    portfolio::{
        allocator::DefaultAllocator,
        portfolio::MetaPortfolio,
        repository::{
            in_memory::InMemoryRepository,
            StatisticHandler,
        },
        risk::DefaultRisk,
    },
    statistic::summary::{
        trading::{
            Config as StatisticConfig,
            TradingSummary,
        },
        Initialiser,
    },
    strategy::example::{
        Config as StrategyConfig,
        RSIStrategy,
    },
};
use barter_data::{
    event::{
        DataKind,
        MarketEvent,
    },
    subscription::candle::Candle,
};
use barter_integration::model::{
    instrument::{
        kind::InstrumentKind,
        Instrument,
    },
    Exchange,
    Market,
    MarketId,
};
use chrono::Utc;
use parking_lot::Mutex;
use tokio::sync::mpsc;
use uuid::Uuid;

pub async fn run_bt() -> Result<(), Box<dyn std::error::Error>> {
    // Setup Logger & Load Config For Engine & Trader Instances Here

    // Create channel to distribute Commands to the Engine & it's Traders (eg/ Command::Terminate)
    let (_command_tx, command_rx) = mpsc::channel(20);

    // Create Event channel to listen to all Engine Events in real-time
    let (event_tx, event_rx) = mpsc::unbounded_channel();
    let event_tx = EventTx::new(event_tx);

    // Generate unique identifier to associate an Engine's components
    let engine_id = Uuid::new_v4();

    // Create the Market(s) to be traded on (1-to-1 relationship with a Trader)
    let exchange = Exchange::from("binance");
    let instrument = Instrument::from(("btc", "usdt", InstrumentKind::Spot));
    let market_id = MarketId::new(&exchange, &instrument);
    let market = Market::new(exchange, instrument);

    // Build global shared-state MetaPortfolio (1-to-1 relationship with an Engine)
    let portfolio = Arc::new(Mutex::new(
        MetaPortfolio::builder()
            .engine_id(engine_id)
            .markets(vec![market.clone()])
            .starting_cash(10_000.0)
            .repository(InMemoryRepository::new())
            .allocation_manager(DefaultAllocator {
                default_order_value: 100.0,
            })
            .risk_manager(DefaultRisk {})
            .statistic_config(StatisticConfig {
                starting_equity: 10_000.0,
                trading_days_per_year: 365,
                risk_free_return: 0.0,
            })
            .build_and_init()
            .expect("failed to build & initialise MetaPortfolio"),
    ));

    // Build Trader(s)
    let mut traders = Vec::new();

    // Create channel for each Trader so the Engine can distribute Commands to it
    let (trader_command_tx, trader_command_rx) = mpsc::channel(10);

    traders.push(
        Trader::builder()
            .engine_id(engine_id)
            .market(market.clone())
            .command_rx(trader_command_rx)
            .event_tx(event_tx.clone())
            .portfolio(Arc::clone(&portfolio))
            .data(historical::MarketFeed::new(
                load_json_market_event_candles().into_iter(),
            ))
            .strategy(RSIStrategy::new(StrategyConfig { rsi_period: 14 }))
            .execution(SimulatedExecution::new(ExecutionConfig {
                simulated_fees_pct: Fees {
                    exchange: 0.1,
                    slippage: 0.05,
                    network: 0.0,
                },
            }))
            .build()
            .expect("failed to build trader"),
    );

    // Build Engine (1-to-many relationship with Traders)

    // Create HashMap<Market, trader_command_tx> so Engine can route Commands to Traders
    let trader_command_txs = HashMap::from_iter([(market, trader_command_tx)]);

    let engine = Engine::builder()
        .engine_id(engine_id)
        .command_rx(command_rx)
        .portfolio(portfolio.clone())
        .traders(traders)
        .trader_command_txs(trader_command_txs)
        .statistics_summary(TradingSummary::init(StatisticConfig {
            starting_equity: 1000.0,
            trading_days_per_year: 365,
            risk_free_return: 0.0,
        }))
        .build()
        .expect("failed to build engine");

    // Listen to Engine Events & do something with them
    // tokio::spawn(listen_to_events(event_rx));

    // --- Run Trading Session Until Remote Shutdown OR Data Feed ends naturally (ie/ backtest) ---
    engine.run().await;

    Ok(())
}

const DATA_HISTORIC_CANDLES_1H: &str = "fixture/data/btcusdt_candles_1d.json";

fn load_json_market_event_candles() -> Vec<MarketEvent<DataKind>> {
    let candles = fs::read_to_string(DATA_HISTORIC_CANDLES_1H).expect("failed to read file");

    let candles =
        serde_json::from_str::<Vec<Candle>>(&candles).expect("failed to parse candles String");

    candles
        .into_iter()
        .map(|candle| MarketEvent {
            exchange_time: candle.close_time,
            received_time: Utc::now(),
            exchange: Exchange::from("binance"),
            instrument: Instrument::from(("btc", "usdt", InstrumentKind::Spot)),
            kind: DataKind::Candle(candle),
        })
        .collect()
}
// Listen to Events that occur in the Engine. These can be used for updating event-sourcing,
// updating dashboard, etc etc.
async fn listen_to_events(mut event_rx: mpsc::UnboundedReceiver<Event>) {
    while let Some(event) = event_rx.recv().await {
        match event {
            Event::Market(_) => {
                // Market Event occurred in Engine
            }
            Event::Signal(signal) => {
                // Signal Event occurred in Engine
                tracing::info!("{signal:?}");
            }
            Event::SignalForceExit(_) => {
                // SignalForceExit Event occurred in Engine
            }
            Event::OrderNew(new_order) => {
                // OrderNew Event occurred in Engine
                tracing::info!("{new_order:?}");
            }
            Event::OrderUpdate => {
                // OrderUpdate Event occurred in Engine
            }
            Event::Fill(fill_event) => {
                // Fill Event occurred in Engine
                tracing::info!("{fill_event:?}");
            }
            Event::PositionNew(new_position) => {
                // PositionNew Event occurred in Engine
                tracing::info!("{new_position:?}");
            }
            Event::PositionUpdate(updated_position) => {
                // PositionUpdate Event occurred in Engine
                tracing::info!("{updated_position:?}");
            }
            Event::PositionExit(exited_position) => {
                // PositionExit Event occurred in Engine
                tracing::info!("{exited_position:?}");
            }
            Event::Balance(balance_update) => {
                // Balance update Event occurred in Engine
                tracing::info!("{balance_update:?}");
            }
        }
    }
}
