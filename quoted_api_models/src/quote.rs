use serde::{Deserialize, Serialize};

use crate::page::{PagedRequest, PagedResponse};

///
/// Defines the accepted request parameters when fetching a random quote.
///
pub type GetRandomQuoteRequest = GetRandomQuoteRequestParams;

//HACK: Cant use PagedRequest<GetQuotesParams>
// because of bug: https://github.com/nox/serde_urlencoded/issues/33
// Workaround using DisplayFromStr doesnt work for Option fields.
// Only option I can see for now is just to not use the PagedRequest when the
// inner struct contains optional fields.
///
/// Defines the request parameters that are supported when fetching quotes for a
/// given episode.
///
pub type GetQuotesRequest = GetQuotesParams; // PagedRequest<GetQuotesParams>;

///
/// Defines the response returned when fetching quotes.
///
pub type GetQuotesResponse = PagedResponse<GetQuotesResponseItem>;

///
/// Defines the accepted request parameters when fetching a random quote.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct GetRandomQuoteRequestParams {
    ///
    /// The name of the show to fetch a quote for.
    ///
    pub show_name: Option<String>,

    ///
    /// The number of season within the show to fetch a quote for.
    ///
    pub season_no: Option<i32>,

    ///
    /// The number of episode within the season to fetch a quote for.
    ///
    pub episode_no: Option<i32>,

    ///
    /// The name of the character who is involved in the quote.
    /// Note that quotes may span multiple characters.
    ///
    pub character_name: Option<String>,
}

///
/// Defines part of a quote that will be included in the response.
/// A quote part can be considered a single characters line within a conversation.
///
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QuotePart {
    ///
    /// The character being quoted.
    ///
    pub character_name: String,

    ///
    /// The order of this part of the quote in relation to the entire quote.
    ///
    pub order: i32,

    ///
    /// The thing that was said.
    ///
    pub quote_text: String,
}

///
/// Defines a quote that was selected at random.
///
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetQuotesResponseItem {
    ///
    /// The name of the show the quote belongs to.
    ///
    pub show_name: String,

    ///
    /// The number of the season within the show that the quote belongs to.
    ///
    pub season_no: i32,

    ///
    /// The name of the season within the show that the quote belongs to.
    /// Not all seasons will have a name.
    ///
    pub season_name: Option<String>,

    ///
    /// The number of the episode within the season that the quote belongs to.
    ///
    pub episode_no: i32,

    ///
    /// The name of the episode within the season that the quote belongs to.
    /// Not all episodes will have a name.
    ///
    pub episode_name: Option<String>,

    ///
    /// The parts that make up the entire quote.
    ///
    pub parts: Vec<QuotePart>,
}

///
/// Defines the request parameters that are supported when fetching quotes in a
/// given episode.
///
/// Example request URL:
///      http://base-url/api/show/{show}/season/{season}/episode/{episode}/quotes
///
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GetQuotesParams {
    // Temporarily include page params on this struct due to bug in serde
    ///
    /// The page number to be fetched.
    /// Defaults to `1`.
    ///
    pub page: u64,

    ///
    /// The maximum number of items to include on the page.
    /// Defaults to `10``.
    ///
    pub limit: u64,

    ///
    /// The name of the show.
    ///
    pub show_name: Option<String>,

    ///
    /// The number of the season within the show.
    ///
    // #[serde_as(as = "DisplayFromStr")]
    pub season_no: Option<i32>,

    ///
    /// The number of the episode within the season.
    ///
    pub episode_no: Option<i32>,
}

// ///
// /// Defines the structure of a quote returned when fetching quotes for a given
// /// episode.
// ///
// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct GetQuotesInEpisodeResponseItem {
//     ///
//     /// The parts of the conversation that make up the quote.
//     ///
//     pub parts: Vec<QuotePart>,
// }

impl Default for GetRandomQuoteRequestParams {
    fn default() -> Self {
        Self {
            show_name: Default::default(),
            season_no: Default::default(),
            episode_no: Default::default(),
            character_name: Default::default(),
        }
    }
}

impl Default for GetQuotesParams {
    fn default() -> Self {
        let pagination = PagedRequest::<i32>::default();
        Self {
            show_name: Default::default(),
            season_no: Default::default(),
            episode_no: Default::default(),
            limit: pagination.limit,
            page: pagination.page,
        }
    }
}
