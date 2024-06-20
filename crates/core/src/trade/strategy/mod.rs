use ta::DataItem;

use crate::api::basic::TradeSide;

mod common;

pub use common::CommonMacdAndRsiStrategy;

pub trait Strategy {
    fn check(&self, data: &[DataItem]) -> TradeSide;
}
