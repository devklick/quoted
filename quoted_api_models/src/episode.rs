use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::page::{PagedRequest, PagedResponse};

///
/// Defines a request to fetch episodes for a given show and season.
///
pub type GetEpisodesInSeasonRequest = PagedRequest<GetEpisodesInSeasonRequestParams>;

///
/// Defines the response that will be returned when fetching episodes for a given
/// show and season.
///
pub type GetEpisodesInSeasonResponse = PagedResponse<GetEpisodesInSeasonResponseItem>;

///
/// Defines the request parameters that are supported when fetching the episodes
/// that belong to a season for a given show.
///
#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct GetEpisodesInSeasonRequestParams {
    ///
    /// The name of the show to fetch episodes for.
    ///
    pub show: String,

    ///
    /// The number of season within the show to fetch episodes for.
    ///
    #[serde_as(as = "DisplayFromStr")]
    pub season: i32,
}

///
/// Defines a single episode that will be returned when fetching the episodes
/// within a given show and season.
///
#[derive(Serialize, Deserialize, Debug, FromQueryResult, Clone)]
pub struct GetEpisodesInSeasonResponseItem {
    ///
    /// The number of the episode within the season
    ///
    episode_no: i32,

    ///
    /// The name of the episode within the season.
    /// Not all episodes will have a name.
    ///
    episode_name: Option<String>,
}

impl Default for GetEpisodesInSeasonRequestParams {
    fn default() -> Self {
        Self {
            show: Default::default(),
            season: Default::default(),
        }
    }
}
