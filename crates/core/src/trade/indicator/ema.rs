use polars::series::Series;
use ta::{
    indicators::ExponentialMovingAverage as Ema,
    Close,
    DataItem,
    Next,
};

use super::Indicator;

#[derive(Default)]
pub struct EmaOutputBuilder {
    ema: Ema,
}

impl EmaOutputBuilder {
    pub fn new(period: usize) -> Self {
        Self {
            ema: Ema::new(period).unwrap(),
        }
    }
}

impl Indicator for EmaOutputBuilder {
    type Output = Series;

    fn compute(&mut self, data: &[DataItem]) -> Self::Output {
        let f = |v: &DataItem| self.ema.next(v.close());
        data.iter().map(f).collect()
    }
}
