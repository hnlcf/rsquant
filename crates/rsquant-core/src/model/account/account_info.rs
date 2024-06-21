use core::fmt;
use std::collections::BTreeMap;

use rust_decimal::Decimal;
use serde::Deserialize;

use super::coin_info::CoinInfo;
use crate::model::{
    DecodeFromStr,
    IntoTarget,
};

#[derive(Debug)]
pub struct AccountInfo {
    account_type: String,
    balances: BTreeMap<String, (Decimal, Decimal)>,
}

impl AccountInfo {
    pub fn account_type(&self) -> &str {
        &self.account_type
    }

    pub fn query_asset(&self, asset: &str) -> Option<Decimal> {
        self.balances.get(asset).map(|(f, _)| f.to_owned())
    }
}

#[derive(Debug, Deserialize)]
pub struct RawAccountInfo {
    /// 账户类型
    #[serde(rename = "accountType")]
    account_type: String,
    /// 资产
    balances: Vec<CoinInfo>,
}

impl DecodeFromStr<'_, RawAccountInfo> for RawAccountInfo {}

impl IntoTarget<AccountInfo> for RawAccountInfo {
    fn into_target(self) -> AccountInfo {
        let account_type = self.account_type;
        let balances = self
            .balances
            .into_iter()
            .map(|c| {
                let CoinInfo {
                    asset,
                    free,
                    locked,
                } = c;
                (asset, (free, locked))
            })
            .collect();
        AccountInfo {
            account_type,
            balances,
        }
    }
}

impl fmt::Display for RawAccountInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{: <10} {: <20}\t{: <20}", "NAME", "FREE", "LOCKED")?;
        for coin in self.balances.iter() {
            writeln!(f, "{}", coin)?;
        }
        write!(f, "")
    }
}
