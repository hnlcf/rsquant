use polars::series::Series;
use ta::{
    indicators::RelativeStrengthIndex as Rsi,
    Close,
    DataItem,
    Next,
};

use super::Indicator;

#[derive(Default)]
pub struct RsiOutputBuilder {
    rsi: Rsi,
}

impl RsiOutputBuilder {
    pub fn new(period: usize) -> Self {
        Self {
            rsi: Rsi::new(period).unwrap(),
        }
    }
}

impl Indicator for RsiOutputBuilder {
    type Output = Series;

    fn compute(&mut self, data: &[DataItem]) -> Self::Output {
        let f = |v: &DataItem| self.rsi.next(v.close());
        data.iter().map(f).collect()
    }
}
