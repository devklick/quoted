use std::{collections::HashMap, str::FromStr};

use google_sheets4::{hyper::client::HttpConnector, hyper_rustls::HttpsConnector, Sheets};
use serde_json::Value;

use crate::{
    error::SeedError,
    seeder::{Episode, Quote, Season, Show},
};

struct RawShow {
    show_name: String,
    season_no: i32,
    episode_no: i32,
    episode_name: Option<String>,
}

pub async fn get_quotes(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    sheet_id: &String,
) -> Result<Vec<Quote>, SeedError> {
    let sheet = hub
        .spreadsheets()
        .values_get(sheet_id, "A:E")
        .doit()
        .await
        .or_else(|e| Err(SeedError::GoogleError(e)))?;

    parse_quotes(&sheet.1.values)
}

pub async fn get_shows(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    sheet_id: &String,
) -> Result<Vec<Show>, SeedError> {
    let sheet = hub
        .spreadsheets()
        .values_get(sheet_id, "A:D")
        .doit()
        .await
        .or_else(|e| Err(SeedError::GoogleError(e)))?;

    let raw_shows = parse_shows(&sheet.1.values)?;

    Ok(unflatten_shows(raw_shows)?)
}

fn unflatten_shows(shows: Vec<RawShow>) -> Result<Vec<Show>, SeedError> {
    // key is episode_no, value is optional episode_name
    type EpisodeMap = HashMap<i32, Option<String>>;
    // key is season_no, key is episode map
    type SeasonMap = HashMap<i32, EpisodeMap>;
    // key is show_name, value is season map
    type ShowMap = HashMap<String, SeasonMap>;

    let mut show_map: ShowMap = HashMap::new();

    for show in shows {
        let seasons_for_show = show_map
            .entry(show.show_name.clone())
            .or_insert(HashMap::new());

        let episodes_for_season = seasons_for_show
            .entry(show.season_no.clone())
            .or_insert(HashMap::new());

        episodes_for_season
            .entry(show.episode_no)
            .or_insert(show.episode_name);
    }

    let shows: Vec<Show> = show_map
        .iter()
        .map(|(show_name, seasons)| Show {
            name: show_name.clone(),
            seasons: seasons
                .iter()
                .map(|(season_no, episodes)| Season {
                    no: season_no.clone(),
                    episodes: episodes
                        .iter()
                        .map(|(episode_no, episode_name)| Episode {
                            no: episode_no.clone(),
                            name: episode_name.clone(),
                        })
                        .collect(),
                })
                .collect(),
        })
        .collect();

    Ok(shows)
}

fn parse_shows(rows: &Option<Vec<Vec<Value>>>) -> Result<Vec<RawShow>, SeedError> {
    let mut row_no = 0;
    let mut quotes = Vec::<RawShow>::new();
    if let Some(row) = rows {
        for cells in row {
            row_no += 1;
            let quote = match parse_show(cells) {
                Ok(val) => val,
                Err(e) => {
                    println!("Error reading row {row_no}, {}, skipping", e);
                    continue;
                }
            };
            quotes.push(quote);
        }
    } else {
        return Err(SeedError::InvalidSheetData(
            "No spreadsheet data found".to_owned(),
        ));
    }
    Ok(quotes)
}

fn parse_show(row_values: &Vec<Value>) -> Result<RawShow, SeedError> {
    // value be an array with 5 items:
    // 0 = ShowName
    let show_name = parse_row_value(&row_values, 0).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid ShowName, {e}"
        )))
    })?;
    // 1 = SeasonNo
    let season_no = parse_row_value::<i32>(&row_values, 1).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid SeasonNo, {e}"
        )))
    })?;
    // 2 = EpisodeNo
    let episode_no = parse_row_value::<i32>(&row_values, 2).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid EpisodeNo, {e}"
        )))
    })?;
    // 3 = EpisodeName
    let episode_name = parse_optional_row_value(&row_values, 4).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid QuoteText, {e}"
        )))
    })?;

    Ok(RawShow {
        episode_no,
        season_no,
        show_name,
        episode_name,
    })
}

fn parse_quotes(rows: &Option<Vec<Vec<Value>>>) -> Result<Vec<Quote>, SeedError> {
    let mut row_no = 0;
    let mut quotes = Vec::<Quote>::new();
    if let Some(row) = rows {
        for cells in row {
            row_no += 1;
            let quote = match parse_quote(cells) {
                Ok(val) => val,
                Err(e) => {
                    println!("Error reading row {row_no}, {}, skipping", e);
                    continue;
                }
            };
            quotes.push(quote);
        }
    } else {
        return Err(SeedError::InvalidSheetData(
            "No spreadsheet data found".to_owned(),
        ));
    }
    Ok(quotes)
}

fn parse_quote(row_values: &Vec<Value>) -> Result<Quote, SeedError> {
    // value be an array with 5 items:
    // 0 = ShowName
    let show_name = parse_row_value(&row_values, 0).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid ShowName, {e}"
        )))
    })?;
    // 1 = SeasonNo
    let season_no = parse_row_value::<i32>(&row_values, 1).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid SeasonNo, {e}"
        )))
    })?;
    // 2 = EpisodeNo
    let episode_no = parse_row_value::<i32>(&row_values, 2).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid EpisodeNo, {e}"
        )))
    })?;
    // 3 = CharacterName
    let character_name = parse_row_value(&row_values, 3).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid CharacterName, {e}"
        )))
    })?;
    // 4 = QuoteText
    let quote_text = parse_row_value(&row_values, 4).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid QuoteText, {e}"
        )))
    })?;

    Ok(Quote {
        character_name,
        episode_no,
        quote_text,
        season_no,
        show_name,
    })
}

fn parse_row_value<T>(cells: &Vec<Value>, cell_index: usize) -> Result<T, String>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let result = parse_optional_row_value(cells, cell_index)?;

    if let Some(result) = result {
        return Ok(result);
    }

    Err(format!("Cell {cell_index} is empty"))
}

fn parse_optional_row_value<T>(cells: &Vec<Value>, cell_index: usize) -> Result<Option<T>, String>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let raw = match cells.get(cell_index) {
        None => return Ok(None),
        Some(val) => val,
    };

    if raw.is_null() {
        return Ok(None);
    }

    Ok(Some(raw.as_str().unwrap().parse::<T>().or(Err(
        format!("Cell {cell_index} value {raw} is invalid"),
    ))?))
}
