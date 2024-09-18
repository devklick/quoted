use sea_orm_migration::prelude::*;

use crate::m20240914_200407_create_quote_table::Quote;

#[derive(DeriveMigrationName)]
pub struct Migration;

const INDEX_NAME: &str = "quote_unique";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                IndexCreateStatement::new()
                    .if_not_exists()
                    .name(INDEX_NAME)
                    .table(Quote::Table)
                    .col(Quote::ShowId)
                    .col(Quote::SeasonId)
                    .col(Quote::EpisodeId)
                    .col(Quote::CharacterId)
                    .col(Quote::Value)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                IndexDropStatement::new()
                    .if_exists()
                    .table(Quote::Table)
                    .name(INDEX_NAME)
                    .to_owned(),
            )
            .await
    }
}
