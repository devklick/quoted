use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Show::Table)
                    .if_not_exists()
                    .col(pk_auto(Show::Id))
                    .col(string(Show::Name).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Show::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Show {
    Table,
    Id,
    Name,
}
