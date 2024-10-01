use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let season_name_col =
            ColumnDef::new_with_type(Season::Name, ColumnType::String(StringLen::default()))
                .null()
                .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(Season::Table)
                    .add_column_if_not_exists(season_name_col)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Season::Table)
                    .drop_column(Season::Name)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Season {
    Table,
    Name,
}
