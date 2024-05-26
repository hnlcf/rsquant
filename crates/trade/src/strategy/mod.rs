use quant_core::api::basic::TradeSide;
use ta::DataItem;

mod common;

pub trait Strategy {
    fn check(&self, data: &[DataItem]) -> TradeSide;
}
