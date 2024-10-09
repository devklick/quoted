use std::{collections::HashMap, str::FromStr};

use google_sheets4::{hyper::client::HttpConnector, hyper_rustls::HttpsConnector, Sheets};
use serde_json::Value;

use crate::{
    error::SeedError,
    seeder::{Episode, Quote, QuotePart, Season, Show},
};

struct RawShow {
    show_name: String,
    season_no: i32,
    season_name: Option<String>,
    episode_no: i32,
    episode_name: Option<String>,
}

struct RawQuote {
    pub show_name: String,
    pub season_no: i32,
    pub episode_no: i32,
    pub quote_grouping: i32,
    pub quote_order_in_group: i32,
    pub character_name: String,
    pub quote_text: String,
}

pub async fn get_quotes(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    sheet_id: &String,
) -> Result<Vec<Quote>, SeedError> {
    let sheet = hub
        .spreadsheets()
        .values_get(sheet_id, "A:G")
        .doit()
        .await
        .or_else(|e| Err(SeedError::GoogleError(e)))?;

    let raw_quotes = parse_quotes(&sheet.1.values)?;

    Ok(unflatten_quotes(raw_quotes)?)
}

pub async fn get_shows(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    sheet_id: &String,
) -> Result<Vec<Show>, SeedError> {
    let sheet = hub
        .spreadsheets()
        .values_get(sheet_id, "A:E")
        .doit()
        .await
        .or_else(|e| Err(SeedError::GoogleError(e)))?;

    let raw_shows = parse_shows(&sheet.1.values)?;

    Ok(unflatten_shows(raw_shows)?)
}

fn unflatten_shows(shows: Vec<RawShow>) -> Result<Vec<Show>, SeedError> {
    // key is episode_no, value is optional episode_name
    type EpisodeMap = HashMap<i32, Option<String>>;

    struct SeasonMapValue {
        season_name: Option<String>,
        episode_map: EpisodeMap,
    }
    // key is season_no, key is episode map
    type SeasonMap = HashMap<i32, SeasonMapValue>;
    // key is show_name, value is season map
    type ShowMap = HashMap<String, SeasonMap>;

    let mut show_map: ShowMap = HashMap::new();

    for show in shows {
        let seasons_for_show = show_map
            .entry(show.show_name.clone())
            .or_insert(HashMap::new());

        let episodes_for_season =
            seasons_for_show
                .entry(show.season_no.clone())
                .or_insert(SeasonMapValue {
                    episode_map: HashMap::new(),
                    season_name: show.season_name,
                });

        episodes_for_season
            .episode_map
            .entry(show.episode_no)
            .or_insert(show.episode_name);
    }

    let shows: Vec<Show> = show_map
        .iter()
        .map(|(show_name, seasons)| Show {
            name: show_name.clone(),
            seasons: seasons
                .iter()
                .map(|(season_no, season_map_val)| Season {
                    no: season_no.clone(),
                    name: season_map_val.season_name.clone(),
                    episodes: season_map_val
                        .episode_map
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

fn unflatten_quotes(quotes: Vec<RawQuote>) -> Result<Vec<Quote>, SeedError> {
    #[derive(PartialEq, Eq, Hash)]
    struct QuoteMapKey {
        show_name: String,
        season_no: i32,
        episode_no: i32,
        quote_grouping: i32,
    }
    struct QuoteMapValueItem {
        quote_order_in_group: i32,
        character_name: String,
        quote_text: String,
    }
    type QuoteMap = HashMap<QuoteMapKey, Vec<QuoteMapValueItem>>;

    let mut quote_map: QuoteMap = HashMap::new();
    for quote in quotes {
        let key = QuoteMapKey {
            episode_no: quote.episode_no,
            quote_grouping: quote.quote_grouping,
            season_no: quote.season_no,
            show_name: quote.show_name,
        };
        let value = QuoteMapValueItem {
            character_name: quote.character_name,
            quote_order_in_group: quote.quote_order_in_group,
            quote_text: quote.quote_text,
        };

        let quote_parts = quote_map.entry(key).or_insert(vec![]);

        quote_parts.push(value);
    }
    let quotes: Vec<Quote> = quote_map
        .iter()
        .map(|(key, value)| Quote {
            show_name: key.show_name.clone(),
            episode_no: key.episode_no,
            season_no: key.season_no,
            source_id: key.quote_grouping,
            parts: value
                .iter()
                .map(|v| QuotePart {
                    order: v.quote_order_in_group,
                    character_name: v.character_name.clone(),
                    quote_text: v.quote_text.clone(),
                })
                .collect(),
        })
        .collect();

    Ok(quotes)
}

fn parse_shows(rows: &Option<Vec<Vec<Value>>>) -> Result<Vec<RawShow>, SeedError> {
    let mut row_no = 0;
    let mut quotes = Vec::<RawShow>::new();
    if let Some(rows) = rows {
        for row in &rows[1..] {
            row_no += 1;
            let quote = match parse_show(row) {
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
    let season_name = parse_optional_row_value::<String>(&row_values, 2).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid SeasonName, {e}"
        )))
    })?;
    // 2 = EpisodeNo
    let episode_no = parse_row_value::<i32>(&row_values, 3).or_else(|e| {
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
        season_name,
        show_name,
        episode_name,
    })
}

fn parse_quotes(rows: &Option<Vec<Vec<Value>>>) -> Result<Vec<RawQuote>, SeedError> {
    println!("{:#?}", rows);
    let mut row_no = 0;
    let mut quotes = Vec::<RawQuote>::new();
    if let Some(rows) = rows {
        for row in &rows[1..] {
            row_no += 1;
            let quote = match parse_quote(row) {
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

fn parse_quote(row_values: &Vec<Value>) -> Result<RawQuote, SeedError> {
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
    // 3 = QuoteGrouping
    let quote_grouping = parse_row_value::<i32>(&row_values, 3).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid QuoteGrouping, {e}"
        )))
    })?;
    // 4 = QuoteOrderInGroup
    let quote_order_in_group = parse_row_value::<i32>(&row_values, 4).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid QuoteOrderInGroup, {e}"
        )))
    })?;
    // 5 = CharacterName
    let character_name = parse_row_value(&row_values, 5).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid CharacterName, {e}"
        )))
    })?;
    // 6 = QuoteText
    let quote_text = parse_row_value(&row_values, 6).or_else(|e| {
        Err(SeedError::InvalidSheetData(format!(
            "Invalid QuoteText, {e}"
        )))
    })?;

    Ok(RawQuote {
        character_name,
        episode_no,
        season_no,
        show_name,
        quote_order_in_group,
        quote_grouping,
        quote_text,
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

    let str = match raw.as_str() {
        None => return Ok(None),
        Some(s) => s,
    };

    if str.is_empty() {
        return Ok(None);
    }

    Ok(Some(str.parse::<T>().or(Err(format!(
        "Cell {cell_index} value {raw} is invalid"
    )))?))
}
