use quoted_api_models::quote::{GetRandomQuoteResponse, QuotePart};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, FromQueryResult)]
pub struct RandomQuoteDBResult {
    pub quote_id: i32,
    pub show_name: String,
    pub season_no: i32,
    pub season_name: Option<String>,
    pub episode_no: i32,
    pub episode_name: Option<String>,
}

impl RandomQuoteDBResult {
    pub fn to_api_response(&self) -> GetRandomQuoteResponse {
        GetRandomQuoteResponse {
            show_name: self.show_name.clone(),
            episode_name: self.episode_name.clone(),
            episode_no: self.episode_no,
            season_no: self.season_no,
            season_name: self.season_name.clone(),
            parts: vec![],
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, FromQueryResult)]
pub struct QuotePartDBResult {
    pub quote_id: i32,
    pub character_name: String,
    pub order: i32,
    pub quote_text: String,
}

impl QuotePartDBResult {
    pub fn to_api_response(&self) -> QuotePart {
        QuotePart {
            character_name: self.character_name.clone(),
            order: self.order,
            quote_text: self.quote_text.clone(),
        }
    }
}
