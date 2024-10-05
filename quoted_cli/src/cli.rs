use clap::{Parser, Subcommand};

use crate::commands::{quote::QuoteArgs, show::ShowArgs};

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Interact with quotes")]
    Quote(QuoteArgs),

    #[command(about = "Interact with shows")]
    Show(ShowArgs),
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
