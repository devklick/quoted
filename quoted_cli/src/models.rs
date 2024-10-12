use std::{fmt::Display, ops::Deref};

pub struct RandomQuote(pub quoted_api_models::quote::GetRandomQuoteResponse);

impl Deref for RandomQuote {
    type Target = quoted_api_models::quote::GetRandomQuoteResponse;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for RandomQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = self.parts.clone();
        let mut season = format!("season {}", self.season_no);
        if let Some(season_name) = &self.season_name {
            season += format!(" - {}", season_name).as_str();
        }
        let mut episode = format!("episode {}", self.episode_no);
        if let Some(episode_name) = &self.episode_name {
            episode += format!(" - {}", episode_name).as_str();
        }
        if parts.len() == 1 {
            // e.g
            // It's OK to lie to women. They're not people like us
            //
            // Peter Griffin
            // Family Guy
            // Season 1
            // Episode 1 - Death Has a Shadow
            return write!(
                f,
                "{}\n\n{}\n{}\nSeason {}\nEpisode {}",
                parts[0].quote_text, parts[0].character_name, self.show_name, season, episode
            );
        }

        parts.sort_by_key(|p| p.order);

        let quote_parts = self
            .parts
            .iter()
            .map(|p| p.character_name.clone() + ": " + p.quote_text.as_str())
            .collect::<Vec<String>>()
            .join("\n");

        // e.g.
        // Philip J. Fry: Does anybody else feel aroused and jealous and worried?
        // Bender Rodriguez: I have't felt much of anything since my guinea pig died.
        //
        // Futurama
        // Season 6
        // Episode 2 - In-A-Gadda-Da-Leela
        write!(
            f,
            "{}\n\n{}\nSeason {}\nEpisode {}",
            quote_parts, self.show_name, season, episode,
        )
    }
}

pub struct ShowsList(
    pub quoted_api_models::page::PagedResponse<quoted_api_models::show::GetShowsResponseItem>,
);

impl Deref for ShowsList {
    type Target =
        quoted_api_models::page::PagedResponse<quoted_api_models::show::GetShowsResponseItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ShowsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let shows = self
            .data
            .iter()
            .map(|s| format!("{}", s.name))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", shows)
    }
}
