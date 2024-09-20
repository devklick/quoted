use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromQueryResult, Debug, Clone)]
pub struct Show {
    pub name: String,
}
