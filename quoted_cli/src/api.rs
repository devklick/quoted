use reqwest::Client;

use quoted_api_models::quote as api_models;

use crate::models::RandomQuote;

const BASE_URL: &str = "https://quoted-delta.vercel.app/api/quote";

pub async fn get_random(
    show: Option<String>,
    season: Option<i32>,
    episode: Option<i32>,
    character: Option<String>,
) -> Result<RandomQuote, String> {
    log::trace!("Building request to fetch random quote");
    let request = api_models::RandomQuoteRequest {
        show_name: show,
        season_no: season,
        episode_no: episode,
        character_name: character,
    };

    let query_string = serde_urlencoded::to_string(request)
        .or_else(|e| Err(format!("Error building query\n{e}")))?;

    let url = BASE_URL.to_owned() + "/random" + "?" + &query_string;

    log::trace!("Getting random quote from {url}");

    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .or_else(|e| Err(format!("Error calling API\n{e}")))?;

    let quote = response
        .json::<api_models::RandomQuoteResponse>()
        .await
        .or_else(|e| Err(format!("Error parsing response\n{e}")))?;

    log::trace!(
        "Found random quote {}",
        serde_json::to_string(&quote).unwrap()
    );

    Ok(RandomQuote(quote))
}
