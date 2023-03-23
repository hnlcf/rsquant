use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Klines {
    data: Vec<Kline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kline {
    /// Kline open time
    open_time: u64,

    /// Kline close time
    close_time: u64,

    /// Open price
    open_price: String,

    /// High price
    high_price: String,

    /// Low price
    low_price: String,

    /// Close price
    close_price: String,

    /// Volume
    volume: String,

    /// Quote asset volume
    quote_asset_volume: String,

    /// Number of trades
    trades_num: u64,

    /// Taker buy base asset volume
    buy_base_volume: String,

    /// Taker buy quote asset volume
    buy_quote_volume: String,

    /// Unused field, ignore.
    ignored: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseKlineError;

impl FromStr for Klines {
    type Err = ParseKlineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .map(|s| s.split(',').map(|e| e.to_owned()).collect::<Vec<String>>())
            .map(|arr| {
                arr.into_iter()
                    .map(|s| {
                        let res: [String; 12] = s
                            .strip_prefix('[')
                            .and_then(|s| s.strip_suffix(']'))
                            .and_then(|s| {
                                s.split(',')
                                    .map(|e| e.to_owned())
                                    .collect::<Vec<String>>()
                                    .try_into()
                                    .ok()
                            })
                            .expect("");

                        let open_time = res[0]
                            .parse::<u64>()
                            .map_err(|_| ParseKlineError)
                            .expect("msg");
                        let close_time = res[1]
                            .parse::<u64>()
                            .map_err(|_| ParseKlineError)
                            .expect("");
                        let open_price = res[2].clone();
                        let high_price = res[3].clone();
                        let low_price = res[4].clone();
                        let close_price = res[5].clone();
                        let volume = res[6].clone();
                        let quote_asset_volume = res[7].clone();
                        let trades_num = res[9]
                            .parse::<u64>()
                            .map_err(|_| ParseKlineError)
                            .expect("");
                        let buy_base_volume = res[9].clone();
                        let buy_quote_volume = res[10].clone();
                        let ignored = res[11].clone();

                        Kline {
                            open_time,
                            close_time,
                            open_price,
                            high_price,
                            low_price,
                            close_price,
                            volume,
                            quote_asset_volume,
                            trades_num,
                            buy_base_volume,
                            buy_quote_volume,
                            ignored,
                        }
                    })
                    .collect::<Vec<Kline>>()
            })
            .ok_or(ParseKlineError)?;
        Ok(Klines { data })
    }
}
