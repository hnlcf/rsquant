#![allow(clippy::enum_variant_names)]
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum SymbolFilters {
    PriceFilter(PriceFilterInfo),
    PercentPrice(PercentPriceInfo),
    PercentPriceBySide,
    LotSize,
    MinNotional,
    Notional,
    IcebergParts,
    MarketLotSize,
    MaxNumOrders,
    MaxNumAlgoOrders,
    MaxNumIcebergOrders,
    MaxPosition,
    TrailingDelta,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ExchangeFilters {
    ExchangeMaxNumOrders,
    ExchangeMaxNumAlgoOrders,
    ExchangeMaxNumIcebergOrders,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PriceFilterInfo {
    min_price: String,
    max_price: String,
    tick_size: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PercentPriceInfo {
    multiplier_up: String,
    multiplier_down: String,
    avg_price_mins: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PercentPriceBySideInfo {
    // TODO: Symbol Filter
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LotSizeInfo {
    // TODO: Symbol Filter
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MinNotionalInfo {
    // TODO: Symbol Filter
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NotionalInfo {
    // TODO: Symbol Filter
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IcebergPartsInfo {
    // TODO: Symbol Filter
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MarketLotSizeInfo {
    // TODO: Symbol Filter
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MaxNumOrdersInfo {
    // TODO: Symbol Filter
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MaxNumAlgoOrdersInfo {
    // TODO: Symbol Filter
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MaxNumIcebergOrdersInfo {
    // TODO: Symbol Filter
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MaxPositionInfo {
    // TODO: Symbol Filter
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrailingDeltaInfo {
    // TODO: Symbol Filter
}
