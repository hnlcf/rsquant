use sea_orm::{
    DeriveActiveEnum,
    EnumIter,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "trade_side")]
pub enum TradeSide {
    #[sea_orm(string_value = "buy")]
    Buy,
    #[sea_orm(string_value = "sell")]
    Sell,
    #[default]
    #[sea_orm(string_value = "nop")]
    Nop,
}
