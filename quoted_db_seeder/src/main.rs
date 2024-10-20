mod auth;
mod cli;
mod client;
mod db_helper;
mod error;
mod id;
mod seeder;
mod sheets;

use clap::Parser;
use dotenvy::dotenv;
use error::SeedError;
use google_sheets4 as sheets4;
use id::IdFactory;
use quoted_db::{enable_query_logging, get_default_connection};
use quoted_db_migration::{Migrator, MigratorTrait};
use sheets4::Sheets;

#[tokio::main]
async fn main() -> Result<(), SeedError> {
    let _ = dotenv();

    let args = cli::Args::parse();

    let db = get_default_connection().await?;
    enable_query_logging();

    let mut id_factory = IdFactory::new(&db);

    let client = client::get();
    let key = auth::get_key(&args.key_path).await?;
    let auth = auth::get_authenticator(key, &client).await?;
    let hub = Sheets::new(client, auth);

    Migrator::up(&db, None).await?;

    if args.shows {
        let shows = sheets::get_shows(&hub, &args.shows_sheet_id).await?;

        seeder::seed_shows(&db, &mut id_factory, shows).await?;
    }

    if args.quotes {
        let quotes = sheets::get_quotes(&hub, &args.quotes_sheet_id).await?;

        seeder::seed_quotes(&db, &mut id_factory, quotes).await?;
    }

    return Ok(());
}
