use std::fmt;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccountInfoRes {
    #[serde(rename = "accountType")]
    account_type: String,
    balances: Vec<CoinInfo>,
}

#[derive(Clone, Deserialize)]
pub struct CoinInfo {
    asset: String,
    free: String,
    locked: String,
}

impl super::BinanResponse for AccountInfoRes {}

impl AccountInfoRes {
    pub fn account_type(&self) -> String {
        self.account_type.to_owned()
    }

    pub fn balances(&self) -> Vec<CoinInfo> {
        self.balances.to_owned()
    }
}

impl fmt::Display for AccountInfoRes {
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
