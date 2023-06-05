use core::{fmt, num};

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct CoinInfo {
    /// 资产名称
    asset: String,
    /// 可用余额
    free: String,
    /// 不可用余额
    locked: String,
}

impl CoinInfo {
    pub fn asset(&self) -> String {
        self.asset.to_owned()
    }

    pub fn free(&self) -> String {
        self.free.to_owned()
    }

    pub fn locked(&self) -> String {
        self.locked.to_owned()
    }

    pub fn is_zero(&self) -> bool {
        let free: Result<f64, num::ParseFloatError> = self.free.parse();
        let locked: Result<f64, num::ParseFloatError> = self.locked.parse();
        if free.is_ok() && locked.is_ok() && (free.unwrap() != 0.0 || locked.unwrap() != 0.0) {
            return true;
        }
        false
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
