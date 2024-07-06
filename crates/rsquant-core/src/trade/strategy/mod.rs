use rsquant_tool::Name;
use ta::DataItem;

use crate::entity::side;

mod common_macd_and_rsi;
mod double_ema;

pub use common_macd_and_rsi::CommonMacdAndRsiStrategy;
pub use double_ema::DoubleEmaStrategy;

pub trait Strategy: Name {
    fn check(&self, data: &[DataItem]) -> side::TradeSide;
}
