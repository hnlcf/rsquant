use binan_spot::{
    http::{request::Request, Credentials},
    market::{self, klines::KlineInterval},
    trade::{
        self,
        account::Account,
        order::{Side, TimeInForce},
    },
    wallet,
};
use quant_core::{Error, Result};
use quant_model::{
    account_info::AccountInfo, kline::Kline, market::ticker_price::TickerPrice, DecodeFromStr,
};
use rust_decimal::Decimal;

use super::{handle_response::AsyncGetResp, BinanHttpClient};

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
            .map(|ks| {
                ks.into_iter()
                    .map(|k| Kline::from_kline(symbol, interval.to_string().as_str(), k))
                    .collect()
            })
    }

    pub async fn get_ticker_price(client: &BinanHttpClient, symbol: &str) -> Result<TickerPrice> {
        let request: Request = market::ticker_price().symbol(symbol).into();
        request
            .get_response(client)
            .await
            .and_then(|ref res| TickerPrice::decode_from_str(res).map_err(Error::Serde))
            .map(TickerPrice::from_ticker)
    }

    pub async fn new_order(
        client: &BinanHttpClient,
        symbol: &str,
        side: Side,
        r#type: &str,
        time_in_force: TimeInForce,
        quantity: Decimal,
        price: Decimal,
        stop_price: Decimal,
    ) -> Result<String> {
        let request: Request = trade::new_order(symbol, side, r#type)
            .time_in_force(time_in_force)
            .quantity(quantity)
            .price(price)
            .stop_price(stop_price)
            .into();
        let data = request.get_response(client).await?;
        tracing::info!("{}", data);
        Ok(data)
    }
}
