use core::fmt;

use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CoinInfo {
    /// 资产名称
    pub asset: String,
    /// 可用余额
    pub free: Decimal,
    /// 不可用余额
    pub locked: Decimal,
}

impl CoinInfo {
    pub fn asset(&self) -> String {
        self.asset.to_owned()
    }

    pub fn free(&self) -> Decimal {
        self.free.to_owned()
    }

    pub fn locked(&self) -> Decimal {
        self.locked.to_owned()
    }

    pub fn is_zero(&self) -> bool {
        self.free.is_zero() && self.locked.is_zero()
    }
}

impl fmt::Display for CoinInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{: <10} {:0<20}\t{:0<20}",
            self.asset, self.free, self.locked
        )
    }
}
