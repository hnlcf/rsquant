use binan_spot::{
    http::{request::Request, Credentials},
    market::{self, klines::KlineInterval},
    trade::account::Account,
    wallet,
};
use quant_model::{
    account_info::AccountInfo, kline::Kline, market::ticker_price::TickerPrice, DecodeFromStr,
};

use super::{handle_response::AsyncGetResp, BinanHttpClient};
use crate::{Error, Result};

pub struct GetResponse;

impl GetResponse {
    pub async fn get_account_snapshot(client: &BinanHttpClient) -> Result<String> {
        let request: Request = wallet::account_snapshot("SPOT").into();
        let data = request.get_response(client).await?;
        tracing::info!("{}", data);
        Ok(data)
    }

    pub async fn get_account_info(
        client: &BinanHttpClient,
        credentials: &Credentials,
    ) -> Result<AccountInfo> {
        let request: Request = Account::default()
            .credentials(credentials)
            .recv_window(5000)
            .into();

        request
            .get_response(client)
            .await
            .and_then(|ref res| AccountInfo::decode_from_str(res).map_err(Error::Serde))
    }

    pub async fn get_kline(
        client: &BinanHttpClient,
        symbol: &str,
        interval: KlineInterval,
        start_time: u64,
        end_time: u64,
        limit: u32,
    ) -> Result<Vec<Kline>> {
        let request: Request = market::klines(symbol, interval)
            .start_time(start_time)
            .end_time(end_time)
            .limit(limit)
            .into();

        request
            .get_response(client)
            .await
            .and_then(|ref res| Vec::decode_from_str(res).map_err(Error::Serde))
    }

    pub async fn get_ticker_price(client: &BinanHttpClient, symbol: &str) -> Result<TickerPrice> {
        let request: Request = market::ticker_price().symbol(symbol).into();
        request
            .get_response(client)
            .await
            .and_then(|ref res| TickerPrice::decode_from_str(res).map_err(Error::Serde))
    }
}
