use rsquant_derive::Name;
use rsquant_tool::Name;

use super::{
    super::indicator::{
        macd::MacdOutputBuilder,
        rsi::RsiOutputBuilder,
        Indicator,
    },
    Strategy,
};
use crate::{
    entity::side,
    min,
};

/// 普通 MACD 和 RSI 的组合策略
///
/// - MACD 用于判断趋势买卖点
/// - RSI 用于过滤噪音，判断超买超卖点
#[derive(Debug, Clone, Name)]
pub struct CommonMacdAndRsiStrategy {
    macd_fast_period: usize,
    macd_slow_period: usize,
    macd_signal_period: usize,
    rsi_period: usize,
    rsi_buy_limit: f64,
    rsi_sell_limit: f64,
}

impl CommonMacdAndRsiStrategy {
    pub fn new(
        macd_fast_period: usize,
        macd_slow_period: usize,
        macd_signal_period: usize,
        rsi_period: usize,
        rsi_buy_limit: f64,
        rsi_sell_limit: f64,
    ) -> Self {
        Self {
            macd_fast_period,
            macd_slow_period,
            macd_signal_period,
            rsi_period,
            rsi_buy_limit,
            rsi_sell_limit,
        }
    }
}

impl Strategy for CommonMacdAndRsiStrategy {
    fn check(&mut self, data: &[ta::DataItem]) -> side::TradeSide {
        assert!(
            data.len()
                >= min!(
                    self.macd_fast_period,
                    self.macd_slow_period,
                    self.macd_signal_period,
                    self.rsi_period
                ),
            "data length is too short"
        );

        let macd = MacdOutputBuilder::new(
            self.macd_fast_period,
            self.macd_slow_period,
            self.macd_signal_period,
        )
        .compute(data);

        let macd_line = macd["macd"].clone();
        let signal_line = macd["signal"].clone();
        let histogram = macd["histogram"].clone();

        // 计算 MACD histogram 极值点
        let histogram_direction = (&histogram - &histogram.shift(1)).tail(Some(2));
        let last_flag: f64 = histogram_direction
            .get(0)
            .ok()
            .and_then(|v| v.extract())
            .unwrap_or_default();
        let current_flag: f64 = histogram_direction
            .get(1)
            .ok()
            .and_then(|v| v.extract())
            .unwrap_or_default();
        let extreme_min = current_flag > 0.0 && last_flag < 0.0;
        let extreme_max = current_flag < 0.0 && last_flag > 0.0;

        // 计算 MACD 快慢线整体位置与相对位置
        let last_fast_flag: f64 = macd_line
            .tail(Some(1))
            .get(0)
            .ok()
            .and_then(|v| v.extract())
            .unwrap_or_default();
        let last_slow_flag: f64 = signal_line
            .tail(Some(1))
            .get(0)
            .ok()
            .and_then(|v| v.extract())
            .unwrap_or_default();
        let last_bar: f64 = histogram
            .tail(Some(1))
            .get(0)
            .ok()
            .and_then(|v| v.extract())
            .unwrap_or_default();
        let low_pos = last_fast_flag < 0.0 && last_slow_flag < 0.0 && last_bar < 0.0;
        let high_pos = last_fast_flag > 0.0 && last_slow_flag > 0.0 && last_bar > 0.0;

        // 计算 RSI 信号
        let rsi = RsiOutputBuilder::new(self.rsi_period).compute(data);
        let last_rsi = rsi.tail(Some(2));
        let rsi_buy_signal = last_rsi.iter().any(|v| {
            let last: f64 = v.extract().unwrap_or_default();
            last <= self.rsi_buy_limit
        });
        let rsi_sell_signal = last_rsi.iter().any(|v| {
            let last: f64 = v.extract().unwrap_or_default();
            last >= self.rsi_sell_limit
        });

        // 信号判断
        if extreme_min && low_pos && rsi_buy_signal {
            return side::TradeSide::Buy;
        }

        if extreme_max && high_pos && rsi_sell_signal {
            return side::TradeSide::Sell;
        }

        side::TradeSide::Nop
    }
}
