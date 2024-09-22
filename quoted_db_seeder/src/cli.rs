use clap::Parser;

#[derive(Parser)]
#[command(name = "quoted")]
#[command(version)]
#[command(author = "devklick")]
#[command(about = "Get quotes from your favorite TV shows")]
pub struct Args {
    #[arg(
        long,
        help = "The path to the google service account key to be used when authenticating with google services",
        default_value = ".keys/quoted_db_seeder_google_priv_key.json"
    )]
    pub key_path: String,

    #[arg(
        long,
        help = "The ID of the google sheet to read",
        env = "QUOTED_DB_SEEDER__QUOTE_GOOGLE_SHEET_ID"
    )]
    pub sheet_id: String,
}
