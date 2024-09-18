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
                    .table(Character::Table)
                    .if_not_exists()
                    .col(pk_auto(Character::Id))
                    .col(string(Character::Name))
                    .col(integer(Character::ShowId))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Character::Table)
                            .from_col(Character::ShowId)
                            .to_tbl(Show::Table)
                            .to_col(Show::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Character::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Character {
    Table,
    Id,
    ShowId,
    Name,
}
