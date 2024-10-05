use clap::{command, Parser, Subcommand};
use crossterm::{
    event::{read, Event, KeyCode},
    terminal,
};

use crate::api;

use super::common::RunCommand;

#[derive(Parser)]
pub struct ShowArgs {
    #[command(subcommand)]
    pub command: ShowCommands,
}

#[derive(Subcommand)]
pub enum ShowCommands {
    #[command(about = "Lists available shows")]
    List(ListShowsCommand),
}

#[derive(Parser)]
pub struct ListShowsCommand {}

impl RunCommand for ListShowsCommand {
    async fn run(self) -> Result<(), String> {
        let mut page = 1;
        let mut shows = api::list_shows(page).await?;

        log::info!("{}", shows);

        let mut fetch_more = shows.has_more && prompt_for_fetch_more();

        while fetch_more {
            page += 1;
            shows = api::list_shows(page).await?;
            log::info!("{}", shows);
            fetch_more = shows.has_more && prompt_for_fetch_more();
        }

        Ok(())
    }
}

fn prompt_for_fetch_more() -> bool {
    log::info!("\nList more shows? (y/N)\n");

    terminal::enable_raw_mode().expect("Error enabling raw term mode");

    let mut more = false;
    loop {
        let event = read().expect("Error reading key events");

        if event == Event::Key(KeyCode::Char('y').into()) {
            more = true;
        }
        break;
    }
    terminal::disable_raw_mode().expect("Error disabling raw mode");
    more
}
