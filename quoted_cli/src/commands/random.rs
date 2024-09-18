use clap::Parser;

use crate::api;

use super::common::RunCommand;

// character, optional, allowed at any time
// show, optional, allowed at any time
// season, optional, allowed only if show specified
// episode, optional, allowed only if season specified
#[derive(Parser)]
pub struct RandomQuoteCommand {
    #[arg(long)]
    show: Option<String>,

    #[arg(long)]
    season: Option<i32>,

    #[arg(long)]
    episode: Option<i32>,

    #[arg(long)]
    character: Option<String>,
}

impl RunCommand for RandomQuoteCommand {
    async fn run(self) -> Result<(), String> {
        let quote = api::get_random(self.show, self.season, self.episode, self.character).await?;
        log::info!("{}", quote);
        Ok(())
    }
}
