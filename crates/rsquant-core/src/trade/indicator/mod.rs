pub mod data_item;
pub mod ema;
pub mod macd;
pub mod rsi;

pub use data_item::ToDataItem;
use ta::DataItem;

pub trait Indicator {
    type Output;
    fn compute(&mut self, data: &[DataItem]) -> Self::Output;
}
