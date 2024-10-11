use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::page::{PagedRequest, PagedResponse};

///
/// Defines the request to fetch seasons for a given show
///
pub type GetShowSeasonsRequest = PagedRequest<GetShowSeasonsRequestParams>;

///
/// Defines the response returned when fetching seasons for a given show
///
pub type GetShowSeasonsResponse = PagedResponse<GetShowSeasonResponseItem>;

///
/// The parameters supported when fetching seasons for a given show.
/// In addition to the parameters defined here, parameters from
/// `GetShowSeasonsRequestParams` are also supported.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct GetShowSeasonsRequestParams {
    ///
    /// The name of the show to fetch seasons for
    ///
    pub show: String,
}

///
/// The data that will be returned for a given season when fetching the seasons
/// for a given show.
///
#[derive(Serialize, Deserialize, FromQueryResult, Debug, Clone)]
pub struct GetShowSeasonResponseItem {
    ///
    /// The season number
    ///
    pub season_no: i32,
    ///
    /// The name of the season, if one exists
    ///
    pub season_name: Option<String>,
}

impl Default for GetShowSeasonsRequestParams {
    fn default() -> Self {
        Self {
            show: Default::default(),
        }
    }
}
