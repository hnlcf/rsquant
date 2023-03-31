use std::fmt;

use async_trait::async_trait;
use binance_spot_connector_rust::hyper::Error as BinanHyperError;
use serde::Deserialize;

use binance_spot_connector_rust::http::request::Request;
use binance_spot_connector_rust::http::Credentials;
use binance_spot_connector_rust::trade::account::Account;

use crate::res::BinanHttpClient;

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

#[async_trait]
impl super::BinanResponse for AccountInfoRes {
    async fn get(client: &BinanHttpClient, credentials: &Credentials) -> Self {
        let request: Request = Account::default()
            .credentials(&credentials)
            .recv_window(5000)
            .into();

        let res = client.send(request).await.map_err(BinanHyperError::Send);
        let data = res.into_body_str().await.map_err(BinanHyperError::Parse);
        serde_json::from_str(&data).expect("Can't parse account info response.")
    }
}

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
