use sea_orm::Statement;
use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240914_000500_create_character_table::Character;

/*
   ==========================================
   This migration consists of multiple steps
   This migration DOWN will cause data loss.
   It will move from multi-part quotes to only single-part quotes.
   While single-part quotes will be preserved, multi-part quotes will be lost.
   ==========================================
*/

#[derive(DeriveMigrationName)]
pub struct Migration;

const INDEX_NAME: &str = "quote_unique";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create QuotePart table
        // copy all quotes from Quote to QuotePart as single-part quotes
        // drop columns Quote.Text, Quote.CharacterId

        // 1. create QuotePart table
        manager
            .create_table(
                Table::create()
                    .table(QuotePart::Table)
                    .if_not_exists()
                    .col(pk_auto(QuotePart::Id))
                    .col(integer(QuotePart::QuoteId))
                    .col(integer(QuotePart::OrderNo))
                    .col(integer(QuotePart::CharacterId))
                    .col(string(QuotePart::Value))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(QuotePart::Table)
                            .from_col(QuotePart::QuoteId)
                            .to_tbl(Quote::Table)
                            .to_col(Quote::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(QuotePart::Table)
                            .from_col(QuotePart::CharacterId)
                            .to_tbl(Character::Table)
                            .to_col(Character::Id),
                    )
                    .index(
                        IndexCreateStatement::new()
                            .if_not_exists()
                            .table(QuotePart::Table)
                            .col(QuotePart::QuoteId)
                            .col(QuotePart::CharacterId)
                            .col(QuotePart::OrderNo)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. copy all quotes from Quote to QuotePart as single-part quotes
        // I think this could be done without raw SQL, however it would involve splitting the
        // migration up into multiple part, where each part has to be run separately, and
        // entities would have to regenerated after running the first one.
        // I feel like this is more of a risk than using raw SQL, so I'll stick with raw SQL.
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            db.get_database_backend(),
            "
            insert into quote_part(quote_id, order_no, character_id, value)
            select id, 1, character_id, value from quote;
            ",
        ))
        .await?;

        // 3. drop columns Quote.Text, Quote.CharacterId,
        // add Quote.SourceId
        manager
            .alter_table(
                Table::alter()
                    .table(Quote::Table)
                    .drop_column(Quote::Value)
                    .drop_column(Quote::CharacterId)
                    .add_column_if_not_exists(
                        ColumnDef::new_with_type(Quote::SourceId, ColumnType::Integer)
                            .not_null()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .if_not_exists()
                    .name(INDEX_NAME)
                    .table(Quote::Table)
                    .col(Quote::SourceId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create column Quote.Part as non-null, create column Quote.CharacterId as non-null
        // update Quote, copying all single-part quotes value  to to Quote value and CharacterId to CharacterId
        // single-part quotes = (e.g. select QuoteId from QuotePart group by QuoteId having count(*) = 1)
        // Drop QuotePart table (***LOSS OF ALL MULTI-PART QUOTES***)
        // Make Quote.Part and Quote.CharacterId non-nullable

        // 1. create column Quote.Part as non-null, create column Quote.CharacterId as non-null,
        // drop Quote.SourceId
        manager
            .alter_table(
                Table::alter()
                    .table(Quote::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new_with_type(
                            Quote::Value,
                            ColumnType::String(StringLen::default()),
                        )
                        .null()
                        .to_owned(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new_with_type(Quote::CharacterId, ColumnType::Integer)
                            .null()
                            .to_owned(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .from_tbl(Quote::Table)
                            .from_col(Quote::CharacterId)
                            .to_tbl(Character::Table)
                            .to_col(Character::Id),
                    )
                    .drop_column(Quote::SourceId)
                    .to_owned(),
            )
            .await?;

        // 2. update Quote, copying all single-part quotes value  to to Quote value and CharacterId to CharacterId
        // single-part quotes = (e.g. select QuoteId from QuotePart group by QuoteId having count(*) = 1)
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            db.get_database_backend(),
            "
            with single_part_quote as (
                select quote_id from quote_part
                group by quote_id 
                having count(*) = 1
            )
            update quote as q
            set value = qp.value, character_id = qp.character_id
            from quote_part qp, single_part_quote sp
            where q.id = qp.quote_id and sp.quote_id = qp.quote_id;
            ",
        ))
        .await?;

        // 3.  Drop QuotePart table (***LOSS OF ALL MULTI-PART QUOTES***)
        manager
            .drop_table(Table::drop().table(QuotePart::Table).if_exists().to_owned())
            .await?;

        // 3. Make Quote.Part and Quote.CharacterId non-nullable
        manager
            .alter_table(
                Table::alter()
                    .table(Quote::Table)
                    .modify_column(ColumnDef::new(Quote::Value).string().to_owned())
                    .modify_column(ColumnDef::new(Quote::CharacterId).integer().to_owned())
                    .to_owned(),
            )
            .await?;

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

#[derive(DeriveIden)]
pub enum Quote {
    Table,
    Id,
    SourceId,
    CharacterId,
    Value,
}

#[derive(DeriveIden)]
pub enum QuotePart {
    Table,
    Id,
    QuoteId,
    OrderNo,
    CharacterId,
    Value,
}
