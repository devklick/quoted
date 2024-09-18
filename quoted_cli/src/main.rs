mod api;
mod cli;
mod commands;

use std::process::ExitCode;

use clap::Parser;
use cli::Commands;
use commands::common::RunCommand;

#[tokio::main]
async fn main() -> ExitCode {
    let args = cli::Args::parse();

    let res = match args.command {
        Commands::Random(random) => random.run().await,
    };

    return match res {
        Err(e) => {
            println!("{e}");
            return ExitCode::FAILURE;
        }
        Ok(_) => ExitCode::SUCCESS,
    };
}
