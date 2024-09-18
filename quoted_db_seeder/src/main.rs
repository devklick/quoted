mod helper;
mod parse_csv;
mod seeder;

use dotenvy::dotenv;
use quoted_db_migration::{Migrator, MigratorTrait};
use sea_orm::DbErr;
use thiserror::Error;

use parse_csv::CsvError;
use quoted_db::{enable_query_logging, error::DBError, get_default_connection};

#[derive(Error, Debug)]
enum SeedError {
    #[error("Database error")]
    DBError(#[from] DBError),

    #[error("Database error")]
    DbErr(#[from] DbErr),

    #[error("CSV Error")]
    Csv(#[from] CsvError),
}

#[tokio::main]
async fn main() -> Result<(), SeedError> {
    dotenv().expect("Failed to read .env file");

    let db = get_default_connection().await?;
    enable_query_logging();

    Migrator::up(&db, None).await?;

    let shows = parse_csv::shows()?;
    seeder::seed_shows(&db, shows).await?;

    let quotes = parse_csv::quotes()?;
    seeder::seed_quotes(&db, quotes).await?;

    return Ok(());
}
