use clap::{Parser, Subcommand};

use crate::commands::random::RandomQuoteCommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Fetches a random quote")]
    Random(RandomQuoteCommand),
}

#[derive(Parser)]
#[command(name = "quoted")]
#[command(version)]
#[command(author = "devklick")]
#[command(about = "Get quotes from your favorite TV shows")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, help = "Enables application logging", global = true)]
    pub verbose: bool,
}
