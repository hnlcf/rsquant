use core::fmt;

use super::coin_info::CoinInfo;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccountInfo {
    /// 账户类型
    #[serde(rename = "accountType")]
    account_type: String,
    /// 资产
    balances: Vec<CoinInfo>,
}

impl AccountInfo {
    pub fn account_type(&self) -> String {
        self.account_type.to_owned()
    }

    pub fn balances(&self) -> Vec<CoinInfo> {
        self.balances.to_owned()
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
