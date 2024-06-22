use rsquant_tool::Name;
use ta::DataItem;

use crate::entity::side;

mod common;

pub use common::CommonMacdAndRsiStrategy;

pub trait Strategy: Name {
    fn check(&self, data: &[DataItem]) -> side::TradeSide;
}
