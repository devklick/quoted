use clap::Parser;

#[derive(Parser)]
#[command(name = "quoted-db-seeder")]
#[command(version)]
#[command(author = "devklick")]
#[command(about = "Seed the quoted database")]
pub struct Args {
    #[arg(
        long,
        help = "The path to the google service account key to be used when authenticating with google services",
        default_value = ".keys/quoted_db_seeder_google_priv_key.json"
    )]
    pub key_path: String,

    #[arg(
        long,
        help = "The ID of the google sheet containing shows, seasons and episodes",
        env = "QUOTED_DB_SEEDER__SHOWS_GOOGLE_SHEET_ID"
    )]
    pub shows_sheet_id: String,

    #[arg(
        long,
        help = "The ID of the google sheet containing quotes",
        env = "QUOTED_DB_SEEDER__QUOTES_GOOGLE_SHEET_ID"
    )]
    pub quotes_sheet_id: String,

    #[arg(
        long,
        help = "Whether or not to seed shows, seasons and episodes",
        default_value_t = false
    )]
    pub shows: bool,

    #[arg(long, help = "Whether or not to seed quotes", default_value_t = false)]
    pub quotes: bool,
}
