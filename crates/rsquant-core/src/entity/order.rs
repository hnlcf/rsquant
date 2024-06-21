use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelBehavior,
    DeriveEntityModel,
    DerivePrimaryKey,
    DeriveRelation,
    EntityTrait,
    EnumIter,
    PrimaryKeyTrait,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::api::basic::TradeSide;

#[derive(Default, Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    pub order_id: u64,

    pub symbol: String,

    pub price: Decimal,

    pub quantity: Decimal,

    pub side: TradeSide,

    pub time_in_force: String,

    pub r#type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
