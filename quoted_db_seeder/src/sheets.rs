use std::str::FromStr;

use google_sheets4::{hyper::client::HttpConnector, hyper_rustls::HttpsConnector, Sheets};
use serde_json::Value;

use crate::{error::SeedError, seeder::Quote};

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

    parse_rows(&sheet.1.values)
}

fn parse_rows(rows: &Option<Vec<Vec<Value>>>) -> Result<Vec<Quote>, SeedError> {
    let mut row_no = 0;
    let mut quotes = Vec::<Quote>::new();
    if let Some(row) = rows {
        for cells in row {
            row_no += 1;
            let quote = match parse_row(cells) {
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

fn parse_row(row_values: &Vec<Value>) -> Result<Quote, SeedError> {
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
    let raw = match cells.get(cell_index) {
        None => return Err(format!("Cell {cell_index} is empty")),
        Some(val) => val,
    };

    Ok(raw
        .as_str()
        .unwrap()
        .parse::<T>()
        .or(Err(format!("Cell {cell_index} value {raw} is invalid")))?)
}
