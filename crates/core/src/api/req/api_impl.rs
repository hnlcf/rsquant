use binan_spot::{
    http::request::Request,
    market,
    trade,
};

use super::{
    send_req::AsyncSendReq,
    HttpClient,
};
use crate::{
    message::{
        AccountInfoApiRequest,
        KlineApiRequest,
        MultipleTickerApiRequest,
        NewOrderApiRequest,
        TickerApiRequest,
    },
    model::{
        account_info::{
            AccountInfo,
            RawAccountInfo,
        },
        kline::Kline,
        market::ticker_price::TickerPrice,
        DecodeFromStr,
        IntoTarget,
    },
    util::time::{
        CurrentTime,
        LocalTimeTool,
    },
    Error,
    Result,
};

pub struct ApiImpl;

impl ApiImpl {
    pub async fn get_account_info(
        client: &HttpClient,
        _req: AccountInfoApiRequest,
    ) -> Result<AccountInfo> {
        let request: Request = trade::account()
            .timestamp(LocalTimeTool::get_unix_time())
            .into();

        request.send_req(client).await.and_then(|ref res| {
            RawAccountInfo::decode_from_str(res)
                .map(|a| a.into_target())
                .map_err(Error::Serde)
        })
    }

    pub async fn get_ticker_price(
        client: &HttpClient,
        req: TickerApiRequest,
    ) -> Result<TickerPrice> {
        let TickerApiRequest { symbol, .. } = req;
        let request: Request = market::ticker_price().symbol(&symbol).into();
        request
            .send_req(client)
            .await
            .and_then(|ref res| TickerPrice::decode_from_str(res).map_err(Error::Serde))
    }

    pub async fn get_multi_ticker_price(
        client: &HttpClient,
        req: MultipleTickerApiRequest,
    ) -> Result<Vec<TickerPrice>> {
        let MultipleTickerApiRequest { symbols, .. } = req;
        let request: Request = market::ticker_price()
            .symbols(symbols.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            .into();
        request.send_req(client).await.and_then(|ref res| {
            <Vec<TickerPrice> as DecodeFromStr<Vec<TickerPrice>>>::decode_from_str(res)
                .map_err(Error::Serde)
        })
    }

    pub async fn get_kline(client: &HttpClient, req: KlineApiRequest) -> Result<Vec<Kline>> {
        let KlineApiRequest {
            symbol,
            interval,
            start_time,
            end_time,
            limit,
        } = req;
        let mut raw_req = market::klines(&symbol, interval).limit(limit);
        if let Some(start_time) = start_time {
            raw_req = raw_req.start_time(start_time);
        }
        if let Some(end_time) = end_time {
            raw_req = raw_req.end_time(end_time);
        }

        let request: Request = raw_req.into();

        request
            .send_req(client)
            .await
            .and_then(|ref res| Vec::decode_from_str(res).map_err(Error::Serde))
    }

    pub async fn new_order(client: &HttpClient, req: NewOrderApiRequest) -> Result<String> {
        let NewOrderApiRequest {
            symbol,
            side,
            r#type,
            time_in_force,
            quantity,
            price,
        } = req;
        let request: Request = trade::new_order(&symbol, side, &r#type)
            .time_in_force(time_in_force)
            .quantity(quantity)
            .price(price)
            .timestamp(LocalTimeTool::get_unix_time())
            .into();

        request.send_req(client).await
    }
}
