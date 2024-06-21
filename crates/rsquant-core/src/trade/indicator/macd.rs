use polars::{
    df,
    frame::DataFrame,
};
use ta::{
    indicators::MovingAverageConvergenceDivergence as Macd,
    Close,
    DataItem,
    Next,
};

use super::Indicator;

#[derive(Default)]
pub struct MacdOutputBuilder {
    macd: Macd,
}

impl MacdOutputBuilder {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Self {
            macd: Macd::new(fast_period, slow_period, signal_period).unwrap(),
        }
    }
}

impl Indicator for MacdOutputBuilder {
    type Output = DataFrame;

    fn compute(&mut self, data: &[DataItem]) -> Self::Output {
        let f = |v: &DataItem| self.macd.next(v.close());
        let macd = data.iter().map(f).collect::<Vec<_>>();
        let macd_line = macd.iter().map(|m| m.macd).collect::<Vec<_>>();
        let signal_line = macd.iter().map(|m| m.signal).collect::<Vec<_>>();
        let histogram = macd.iter().map(|m| m.histogram).collect::<Vec<_>>();

        df! {
            "macd" => &macd_line,
            "signal" => &signal_line,
            "histogram" => &histogram,
        }
        .expect("create DataFrame failed")
    }
}
