use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20220101_000001_create_show_table::Show,
    m20240914_000500_create_character_table::Character,
    m20240914_080125_create_season_n_episode_tables::{Episode, Season},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Quote::Table)
                    .if_not_exists()
                    .col(pk_auto(Quote::Id))
                    .col(string(Quote::Value))
                    .col(integer(Quote::CharacterId))
                    .col(integer(Quote::ShowId))
                    .col(integer(Quote::SeasonId))
                    .col(integer(Quote::EpisodeId))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Quote::Table)
                            .from_col(Quote::CharacterId)
                            .to_tbl(Character::Table)
                            .to_col(Character::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Quote::Table)
                            .from_col(Quote::ShowId)
                            .to_tbl(Show::Table)
                            .to_col(Show::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Quote::Table)
                            .from_col(Quote::SeasonId)
                            .to_tbl(Season::Table)
                            .to_col(Season::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Quote::Table)
                            .from_col(Quote::EpisodeId)
                            .to_tbl(Episode::Table)
                            .to_col(Episode::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Quote::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Quote {
    Table,
    Id,
    CharacterId,
    ShowId,
    EpisodeId,
    SeasonId,
    Value,
}
