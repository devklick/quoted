use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::page::{PagedData, PagedRequest};

pub type GetShowsRequest = PagedRequest<GetShowsRequestParams>;
pub type GetShowsResponse = PagedData<Show>;

#[derive(Serialize, Deserialize, FromQueryResult, Debug, Clone)]
pub struct Show {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetShowsRequestParams {}

impl Default for GetShowsRequestParams {
    fn default() -> Self {
        Self {}
    }
}
