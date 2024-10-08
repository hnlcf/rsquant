use polars::series::ChunkCompare;
use rsquant_derive::Name;
use rsquant_tool::Name;

use super::Strategy;
use crate::{
    entity::side,
    trade::{
        indicator::ema::EmaOutputBuilder,
        Indicator,
    },
};

#[derive(Debug, Clone, Name)]
pub struct DoubleEmaStrategy {
    short_period: usize,
    long_period: usize,
}

impl DoubleEmaStrategy {
    pub fn new(short_period: usize, long_period: usize) -> Self {
        Self {
            short_period,
            long_period,
        }
    }
}

impl Strategy for DoubleEmaStrategy {
    fn check(&mut self, data: &[ta::DataItem]) -> side::TradeSide {
        assert!(data.len() >= self.long_period, "data length is too short");

        let short_ema = EmaOutputBuilder::new(self.short_period)
            .compute(data)
            .tail(Some(5));
        let long_ema = EmaOutputBuilder::new(self.long_period)
            .compute(data)
            .tail(Some(5));

        let diff = short_ema - long_ema;
        tracing::debug!("[{}]: diff: {:?}", self.get_name(), diff);

        let mask = diff.gt_eq(0).unwrap();
        tracing::debug!("[{}]: mask: {:?}", self.get_name(), mask);

        let downward = is_downward(mask.into_iter().flatten());
        let upward = is_upward(mask.into_iter().flatten());

        if downward {
            return side::TradeSide::Sell;
        }

        if upward {
            return side::TradeSide::Buy;
        }

        side::TradeSide::Nop
    }
}

fn is_downward(target: impl Iterator<Item = bool>) -> bool {
    let mut prefix_true = false;
    let mut suffix_false = false;

    for i in target {
        let check_prefix = i;
        let check_suffix = !i;

        if suffix_false && i {
            return false;
        }

        if check_prefix && !check_suffix {
            prefix_true = true;
        }

        if check_suffix && !check_prefix {
            suffix_false = true;
        }
    }

    prefix_true && suffix_false
}

fn is_upward(target: impl Iterator<Item = bool>) -> bool {
    let mut prefix_false = false;
    let mut suffix_true = false;

    for i in target {
        let check_prefix = !i;
        let check_suffix = i;

        if suffix_true && !i {
            return false;
        }

        if check_prefix && !check_suffix {
            prefix_false = true;
        }

        if check_suffix && !check_prefix {
            suffix_true = true;
        }
    }

    prefix_false && suffix_true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_downward() {
        assert!(super::is_downward(
            [true, true, true, true, false].into_iter()
        ));
        assert!(super::is_downward(
            [true, true, true, false, false].into_iter()
        ));
        assert!(super::is_downward(
            [true, true, false, false, false].into_iter()
        ));
        assert!(super::is_downward(
            [true, false, false, false, false].into_iter()
        ));

        assert!(!super::is_downward(
            [true, true, true, true, true].into_iter()
        ));
        assert!(!super::is_downward(
            [false, false, false, false, false].into_iter()
        ));
        assert!(!super::is_downward(
            [true, true, true, false, true].into_iter()
        ));
        assert!(!super::is_downward(
            [true, true, false, true, false].into_iter()
        ));
        assert!(!super::is_downward(
            [false, true, false, true, false].into_iter()
        ));
    }

    #[test]
    fn test_is_upward() {
        assert!(super::is_upward(
            [false, false, false, false, true].into_iter()
        ));
        assert!(super::is_upward(
            [false, false, false, true, true].into_iter()
        ));
        assert!(super::is_upward(
            [false, false, true, true, true].into_iter()
        ));
        assert!(super::is_upward(
            [false, true, true, true, true].into_iter()
        ));

        assert!(!super::is_upward(
            [false, false, false, false, false].into_iter()
        ));
        assert!(!super::is_upward(
            [true, true, true, true, true].into_iter()
        ));
        assert!(!super::is_upward(
            [false, false, false, true, false].into_iter()
        ));
        assert!(!super::is_upward(
            [false, false, true, false, true].into_iter()
        ));
        assert!(!super::is_upward(
            [true, false, true, false, true].into_iter()
        ));
    }
}
