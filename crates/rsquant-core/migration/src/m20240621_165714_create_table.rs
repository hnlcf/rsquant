use rsquant_core::entity::order;
use sea_orm_migration::prelude::*;

use crate::{
    extension::postgres::Type,
    sea_orm::Schema,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        for stmt in schema.create_enum_from_entity(order::Entity) {
            manager.create_type(stmt).await?;
        }

        manager
            .create_table(
                schema
                    .create_table_from_entity(order::Entity)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(order::Entity.into_table_ref())
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        let ty_names = [Orders::TradeSide];
        let stmt = Type::drop()
            .if_exists()
            .names(ty_names)
            .cascade()
            .to_owned();

        manager.drop_type(stmt).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Orders {
    TradeSide,
}
