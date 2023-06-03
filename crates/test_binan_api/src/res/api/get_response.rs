use binance_spot_connector_rust::{
    http::{request::Request, Credentials},
    market::{self, klines::KlineInterval},
    trade::account::Account,
};

use crate::res::api::handle_response::HandleResponse;
use crate::res::{account_info, kline, ticker_price::TickerPriceRes, BinanHttpClient};

pub struct GetResponse;

impl GetResponse {
    pub async fn get_account_info(
        client: &BinanHttpClient,
        credentials: &Credentials,
    ) -> account_info::AccountInfoRes {
        let request: Request = Account::default()
            .credentials(credentials)
            .recv_window(5000)
            .into();

        let data = HandleResponse::get_response(client, request).await;
        HandleResponse::decode_response(&data).await
    }

    pub async fn get_kline(
        client: &BinanHttpClient,
        symbol: &str,
        interval: KlineInterval,
        start_time: u64,
        end_time: u64,
        limit: u32,
    ) -> Vec<kline::KlineRes> {
        let request = market::klines(symbol, interval)
            .start_time(start_time)
            .end_time(end_time)
            .limit(limit);

        let data = HandleResponse::get_response(client, request).await;
        HandleResponse::decode_response(&data).await
    }

    pub async fn get_ticker_price(client: &BinanHttpClient, symbol: &str) -> TickerPriceRes {
        let request = market::ticker_price().symbol(symbol);
        let data = HandleResponse::get_response(client, request).await;
        HandleResponse::decode_response(&data).await
    }
}
