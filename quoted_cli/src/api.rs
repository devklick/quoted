use reqwest::Client;

use quoted_api_models::quote as models;

const BASE_URL: &str = "https://quoted-delta.vercel.app/api/quote";

pub async fn get_random(
    show: Option<String>,
    season: Option<i32>,
    episode: Option<i32>,
    character: Option<String>,
) -> Result<models::RandomQuoteResponse, String> {
    let request = models::RandomQuoteRequest {
        show_name: show,
        season_no: season,
        episode_no: episode,
        character_name: character,
    };

    let query_string = serde_urlencoded::to_string(request).or(Err("Error building query"))?;
    let url = BASE_URL.to_owned() + "/random" + "?" + &query_string;
    let client = Client::new();
    let request = client.get(url);
    let response = request
        .send()
        .await
        .or_else(|e| Err(format!("Error calling API, {e}")))?;

    let quote = response
        .json::<models::RandomQuoteResponse>()
        .await
        .or_else(|e| Err(format!("Invalid API Response, {e}")))?;

    Ok(quote)
}
