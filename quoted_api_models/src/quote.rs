use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RandomQuoteRequest {
    pub show_name: Option<String>,
    pub season_no: Option<i32>,
    pub episode_no: Option<i32>,
    pub character_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RandomQuotePart {
    pub character_name: String,
    pub order: i32,
    pub quote_text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RandomQuoteResponse {
    pub show_name: String,
    pub season_no: i32,
    pub season_name: Option<String>,
    pub episode_no: i32,
    pub episode_name: Option<String>,
    pub parts: Vec<RandomQuotePart>,
}

impl Default for RandomQuoteRequest {
    fn default() -> Self {
        Self {
            show_name: Default::default(),
            season_no: Default::default(),
            episode_no: Default::default(),
            character_name: Default::default(),
        }
    }
}
