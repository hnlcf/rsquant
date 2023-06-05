use binan_spot::{
    http::{request::Request, Credentials},
    market::{self, klines::KlineInterval},
    trade::account::Account,
    wallet,
};
use quant_model::{account_info::AccountInfo, kline::Kline, market::ticker_price::TickerPrice};

use super::{BinanHttpClient, HandleResponse};

pub struct GetResponse;

impl GetResponse {
    pub async fn get_account_snapshot(client: &BinanHttpClient) -> String {
        let request = wallet::account_snapshot("SPOT");
        let data = HandleResponse::get_response(client, request).await;
        log::info!("{}", data);
        data
    }

    pub async fn get_account_info(
        client: &BinanHttpClient,
        credentials: &Credentials,
    ) -> AccountInfo {
        let request: Request = Account::default()
            .credentials(credentials)
            .recv_window(5000)
            .into();

        let data = HandleResponse::get_response(client, request).await;
        HandleResponse::decode_response(&data)
    }

    pub async fn get_kline(
        client: &BinanHttpClient,
        symbol: &str,
        interval: KlineInterval,
        start_time: u64,
        end_time: u64,
        limit: u32,
    ) -> Vec<Kline> {
        let request = market::klines(symbol, interval)
            .start_time(start_time)
            .end_time(end_time)
            .limit(limit);

        let data = HandleResponse::get_response(client, request).await;
        HandleResponse::decode_response(&data)
    }

    pub async fn get_ticker_price(client: &BinanHttpClient, symbol: &str) -> TickerPrice {
        let request = market::ticker_price().symbol(symbol);
        let data = HandleResponse::get_response(client, request).await;
        HandleResponse::decode_response(&data)
    }
}
