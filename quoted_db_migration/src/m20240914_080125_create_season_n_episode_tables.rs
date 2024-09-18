use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_show_table::Show;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Season::Table)
                    .if_not_exists()
                    .col(pk_auto(Season::Id))
                    .col(integer(Season::SeasonNo))
                    .col(integer(Season::ShowId))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Season::Table)
                            .from_col(Season::ShowId)
                            .to_tbl(Show::Table)
                            .to_col(Show::Id),
                    )
                    .index(
                        IndexCreateStatement::new()
                            .col(Season::ShowId)
                            .col(Season::SeasonNo)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        return manager
            .create_table(
                Table::create()
                    .table(Episode::Table)
                    .if_not_exists()
                    .col(pk_auto(Episode::Id))
                    .col(integer(Episode::EpisodeNo))
                    // issue https://github.com/SeaQL/sea-orm/issues/2337
                    .col(
                        ColumnDef::new_with_type(
                            Episode::Name,
                            ColumnType::String(StringLen::default()),
                        )
                        .null(),
                    )
                    .col(integer(Episode::ShowId))
                    .col(integer(Episode::SeasonId))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Episode::Table)
                            .from_col(Episode::ShowId)
                            .to_tbl(Show::Table)
                            .to_col(Show::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Episode::Table)
                            .from_col(Episode::SeasonId)
                            .to_tbl(Season::Table)
                            .to_col(Season::Id),
                    )
                    .index(
                        IndexCreateStatement::new()
                            .col(Episode::ShowId)
                            .col(Episode::SeasonId)
                            .col(Episode::EpisodeNo)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await;
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Episode::Table).to_owned())
            .await?;

        return manager
            .drop_table(Table::drop().table(Season::Table).to_owned())
            .await;
    }
}

#[derive(DeriveIden)]
pub enum Season {
    Table,
    Id,
    SeasonNo,
    ShowId,
}

#[derive(DeriveIden)]
pub enum Episode {
    Table,
    Id,
    EpisodeNo,
    Name,
    SeasonId,
    ShowId,
}
