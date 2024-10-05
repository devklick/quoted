use reqwest::Client;

use quoted_api_models::{
    self as api_models,
    show::{GetShowsRequest, GetShowsResponse},
};

use crate::models::{RandomQuote, ShowsList};

const BASE_URL: &str = "https://quoted-delta.vercel.app/api";

pub async fn get_random(
    show: Option<String>,
    season: Option<i32>,
    episode: Option<i32>,
    character: Option<String>,
) -> Result<RandomQuote, String> {
    log::trace!("Building request to fetch random quote");
    let request = api_models::quote::RandomQuoteRequest {
        show_name: show,
        season_no: season,
        episode_no: episode,
        character_name: character,
    };

    let query_string = serde_urlencoded::to_string(request)
        .or_else(|e| Err(format!("Error building query\n{e}")))?;

    let url = BASE_URL.to_owned() + "/quote/random" + "?" + &query_string;

    log::trace!("Getting random quote from {url}");

    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .or_else(|e| Err(format!("Error calling API\n{e}")))?;

    let quote = response
        .json::<api_models::quote::RandomQuoteResponse>()
        .await
        .or_else(|e| Err(format!("Error parsing response\n{e}")))?;

    log::trace!(
        "Found random quote {}",
        serde_json::to_string(&quote).unwrap()
    );

    Ok(RandomQuote(quote))
}

pub async fn list_shows(page_no: u64) -> Result<ShowsList, String> {
    let request = GetShowsRequest {
        page: page_no,
        limit: 1,
        ..Default::default()
    };

    let query_string = serde_urlencoded::to_string(request)
        .or_else(|e| Err(format!("Error building query\n{e}")))?;

    let url = BASE_URL.to_owned() + "/shows" + "?" + &query_string;

    log::trace!("Getting shows from {url}");

    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .or_else(|e| Err(format!("Error calling API\n{e}")))?;

    let shows = response
        .json::<GetShowsResponse>()
        .await
        .or_else(|e| Err(format!("Error parsing response\n{e}")))?;

    log::trace!("Found shows {}", serde_json::to_string(&shows).unwrap());

    Ok(ShowsList(shows))
}
