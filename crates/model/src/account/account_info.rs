use core::fmt;

use super::coin_info::CoinInfo;
use crate::DecodeFromStr;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccountInfo {
    /// 账户类型
    #[serde(rename = "accountType")]
    account_type: String,
    /// 资产
    balances: Vec<CoinInfo>,
}

impl DecodeFromStr<'_, AccountInfo> for AccountInfo {}

impl AccountInfo {
    pub fn account_type(&self) -> &str {
        &self.account_type
    }

    pub fn balances(&self) -> &[CoinInfo] {
        &self.balances
    }

    pub fn remove_blank_coin(self) -> Self {
        let new_balances = self.balances.into_iter().filter(|c| c.is_zero()).collect();

        AccountInfo {
            account_type: self.account_type,
            balances: new_balances,
        }
    }
}

impl fmt::Display for AccountInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{: <10} {: <20}\t{: <20}", "NAME", "FREE", "LOCKED")?;
        for coin in self.balances.iter() {
            writeln!(f, "{}", coin)?;
        }
        write!(f, "")
    }
}
