use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::page::{PagedRequest, PagedResponse};

///
/// Defines a request to fetch a page of shows.
///
pub type GetShowsRequest = PagedRequest<GetShowsRequestParams>;

///
/// Defines a response returned when fetching a page of shows.
///
pub type GetShowsResponse = PagedResponse<GetShowsResponseItem>;

///
/// Defines the accepted request parameters when fetching a page of shows.
/// These are in addition to the parameters provided by `PagedRequest`.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct GetShowsRequestParams {}

///
/// Defines the structure of a show will be included in the response.
///
#[derive(Serialize, Deserialize, FromQueryResult, Debug, Clone)]
pub struct GetShowsResponseItem {
    pub name: String,
}

impl Default for GetShowsRequestParams {
    fn default() -> Self {
        Self {}
    }
}
