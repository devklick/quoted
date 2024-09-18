use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20220101_000001_create_show_table::Show, m20240914_000500_create_character_table::Character,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CharacterShow::Table)
                    .if_not_exists()
                    .col(integer(CharacterShow::CharacterId))
                    .col(integer(CharacterShow::ShowId))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(CharacterShow::Table)
                            .from_col(CharacterShow::CharacterId)
                            .to_tbl(Character::Table)
                            .to_col(Character::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(CharacterShow::Table)
                            .from_col(CharacterShow::ShowId)
                            .to_tbl(Show::Table)
                            .to_col(Show::Id),
                    )
                    .primary_key(
                        IndexCreateStatement::new()
                            .col(CharacterShow::CharacterId)
                            .col(CharacterShow::ShowId)
                            .primary(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CharacterShow::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CharacterShow {
    Table,
    CharacterId,
    ShowId,
}
