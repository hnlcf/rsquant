use rsquant_tool::Name;
use ta::DataItem;

use crate::entity::side;

mod common_macd_and_rsi;
mod double_ema;
mod rsi_and_double_ema;

pub use common_macd_and_rsi::CommonMacdAndRsiStrategy;
pub use double_ema::DoubleEmaStrategy;
pub use rsi_and_double_ema::RsiAndDoubleEmaStrategy;

pub trait Strategy: Name {
    fn check(&mut self, data: &[DataItem]) -> side::TradeSide;
}
