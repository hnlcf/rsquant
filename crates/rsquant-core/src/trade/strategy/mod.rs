use rsquant_tool::Name;
use ta::DataItem;

use crate::api::basic::TradeSide;

mod common;

pub use common::CommonMacdAndRsiStrategy;

pub trait Strategy: Name {
    fn check(&self, data: &[DataItem]) -> TradeSide;
}
