#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRes {
    #[serde(rename = "accountType")]
    account_type: String,
    balances: Vec<CoinInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinInfo {
    asset: String,
    free: String,
    locked: String,
}

impl super::Response for AccountRes {}

impl AccountRes {
    pub fn account_type(&self) -> String {
        self.account_type.to_owned()
    }

    pub fn balances(&self) -> Vec<CoinInfo> {
        self.balances.to_owned()
    }
}

impl fmt::Display for AccountRes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{: <10} {: <20}\t{: <20}", "NAME", "FREE", "LOCKED")?;
        for coin in self.balances.iter() {
            writeln!(f, "{}", coin)?;
        }
        write!(f, "")
    }
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
