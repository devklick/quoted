use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct Show {
    pub name: String,
}
