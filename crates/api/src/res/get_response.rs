use binan_spot::{http::request::Request, market, trade, wallet};
use quant_core::{Error, Result};
use quant_model::{
    account_info::AccountInfo, kline::Kline, market::ticker_price::TickerPrice, DecodeFromStr,
};

use crate::message::{
    AccountInfoApiRequest, KlineApiRequest, NewOrderApiRequest, TickerApiRequest,
};

use super::{handle_response::AsyncGetResp, BinanHttpClient};

pub struct GetResponse;

impl GetResponse {
    pub async fn get_account_snapshot(client: &BinanHttpClient) -> Result<String> {
        let request: Request = wallet::account_snapshot("SPOT").into();
        request.get_response(client).await
    }

    pub async fn get_account_info(
        client: &BinanHttpClient,
        _req: AccountInfoApiRequest,
    ) -> Result<AccountInfo> {
        let request: Request = trade::account().into();

        request
            .get_response(client)
            .await
            .and_then(|ref res| AccountInfo::decode_from_str(res).map_err(Error::Serde))
    }

    pub async fn get_ticker_price(
        client: &BinanHttpClient,
        req: TickerApiRequest,
    ) -> Result<TickerPrice> {
        let TickerApiRequest { symbol } = req;
        let request: Request = market::ticker_price().symbol(&symbol).into();
        request
            .get_response(client)
            .await
            .and_then(|ref res| TickerPrice::decode_from_str(res).map_err(Error::Serde))
            .map(TickerPrice::from_ticker)
    }

    pub async fn get_kline(client: &BinanHttpClient, req: KlineApiRequest) -> Result<Vec<Kline>> {
        let KlineApiRequest {
            symbol,
            interval,
            start_time,
            end_time,
            limit,
        } = req;
        let request: Request = market::klines(&symbol, interval)
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
                    .map(|k| Kline::from_kline(&symbol, interval.to_string().as_str(), k))
                    .collect()
            })
    }

    pub async fn new_order(client: &BinanHttpClient, req: NewOrderApiRequest) -> Result<String> {
        let NewOrderApiRequest {
            symbol,
            side,
            r#type,
            time_in_force,
            quantity,
            price,
            stop_price,
        } = req;
        let request: Request = trade::new_order(&symbol, side, &r#type)
            .time_in_force(time_in_force)
            .quantity(quantity)
            .price(price)
            .stop_price(stop_price)
            .into();
        request.get_response(client).await
    }
}
