use serde::{Deserialize, Serialize};

///
/// Defines the accepted request parameters when fetching a random quote.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct RandomQuoteRequestParams {
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
pub struct RandomQuotePart {
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
pub struct RandomQuoteResponse {
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
    pub parts: Vec<RandomQuotePart>,
}

impl Default for RandomQuoteRequestParams {
    fn default() -> Self {
        Self {
            show_name: Default::default(),
            season_no: Default::default(),
            episode_no: Default::default(),
            character_name: Default::default(),
        }
    }
}
