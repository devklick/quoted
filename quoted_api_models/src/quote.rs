use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, FromQueryResult)]
pub struct QuoteDto {
    pub show_name: String,
    pub character_name: String,
    pub season_no: i32,
    pub episode_no: i32,
    pub episode_name: Option<String>,
    pub quote_text: String,
}
