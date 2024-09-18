mod api;
mod cli;
mod commands;
mod models;

use std::process::ExitCode;

use clap::Parser;
use cli::Commands;
use commands::common::RunCommand;
use fern::colors::{Color, ColoredLevelConfig};

#[tokio::main]
async fn main() -> ExitCode {
    let args = cli::Args::parse();

    if let Err(err) = init_logger(args.verbose) {
        println!("Failed to initialize logger\n{err}");
        return ExitCode::FAILURE;
    }

    let res = match args.command {
        Commands::Random(random) => random.run().await,
    };

    return match res {
        Err(e) => {
            println!("Failed!\n{e}");
            return ExitCode::FAILURE;
        }
        Ok(_) => ExitCode::SUCCESS,
    };
}

fn init_logger(verbose: bool) -> Result<(), String> {
    let log_level = match verbose {
        true => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };
    let colors = ColoredLevelConfig::new()
        .trace(Color::Magenta)
        .error(Color::Red)
        .debug(Color::Blue);

    let builder = fern::Dispatch::new()
        .format(move |out, message, record| {
            let level = record.level();
            if level == log::Level::Info {
                out.finish(format_args!("{}", message));
            } else {
                out.finish(format_args!("{} {}", colors.color(level), message));
            }
        })
        .level(log_level)
        .chain(std::io::stdout());

    return match builder.apply() {
        Err(e) => Err(e.to_string()),
        Ok(_) => Ok(()),
    };
}
