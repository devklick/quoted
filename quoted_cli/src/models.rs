use std::{fmt::Display, ops::Deref};

pub struct RandomQuote(pub quoted_api_models::quote::RandomQuoteResponse);

impl Deref for RandomQuote {
    type Target = quoted_api_models::quote::RandomQuoteResponse;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for RandomQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{}, season {}, episode {}",
            self.quote_text, self.character_name, self.show_name, self.season_no, self.episode_no,
        )
    }
}
