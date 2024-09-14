use rsquant_derive::Name;
use rsquant_tool::Name;
use ta::Close;

use super::Strategy;
use crate::{
    entity::side::TradeSide,
    trade::{
        indicator::{
            ema::EmaOutputBuilder,
            rsi::RsiOutputBuilder,
        },
        Indicator,
    },
};

#[derive(Debug, Clone, Name)]
pub struct RsiAndDoubleEmaStrategy {
    short_ema: EmaOutputBuilder,
    long_ema: EmaOutputBuilder,
    rsi: RsiOutputBuilder,
    rsi_buy_limit: f64,
    rsi_sell_limit: f64,
}

impl RsiAndDoubleEmaStrategy {
    pub fn new(
        short_ema_period: usize,
        long_ema_period: usize,
        rsi_period: usize,
        rsi_buy_limit: f64,
        rsi_sell_limit: f64,
    ) -> Self {
        let short_ema = EmaOutputBuilder::new(short_ema_period);
        let long_ema = EmaOutputBuilder::new(long_ema_period);
        let rsi = RsiOutputBuilder::new(rsi_period);
        Self {
            short_ema,
            long_ema,
            rsi,
            rsi_buy_limit,
            rsi_sell_limit,
        }
    }
}

impl Strategy for RsiAndDoubleEmaStrategy {
    fn check(&mut self, data: &[ta::DataItem]) -> crate::entity::side::TradeSide {
        // 1. compute double ema and rsi
        // 2. if last close is upward of short ema and last rsi larger than rsi_buy_limit, then buy
        // 3. if last close is downward of short ema, sell last order

        let short_ema = self.short_ema.compute(data);
        let long_ema = self.long_ema.compute(data);
        let rsi = self.rsi.compute(data);

        let last_close = data.last().unwrap().close();
        let last_short_ema = short_ema
            .tail(Some(1))
            .get(0)
            .ok()
            .and_then(|v| v.extract())
            .unwrap_or_default();
        let last_long_ema = long_ema
            .tail(Some(1))
            .get(0)
            .ok()
            .and_then(|v| v.extract())
            .unwrap_or_default();
        let last_rsi: f64 = rsi
            .tail(Some(1))
            .get(0)
            .ok()
            .and_then(|v| v.extract())
            .unwrap_or_default();

        if last_close > last_short_ema
            && last_close > last_long_ema
            && last_rsi > self.rsi_buy_limit
        {
            return TradeSide::Buy;
        }
        if last_close < last_short_ema {
            return TradeSide::Sell;
        }
        TradeSide::Nop
    }
}
