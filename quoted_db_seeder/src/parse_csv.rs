use serde::Deserialize;
use std::{collections::HashMap, io::Cursor};
use thiserror::Error;

use crate::SeedError;

#[derive(Debug)]
pub struct Show {
    pub name: String,
    pub seasons: Vec<Season>,
}
#[derive(Debug)]
pub struct Season {
    pub no: i32,
    pub episodes: Vec<Episode>,
}
#[derive(Debug)]
pub struct Episode {
    pub no: i32,
    pub name: Option<String>,
}

#[derive(Error, Debug)]
pub enum CsvError {
    #[error("Error reading CSV")]
    CsvReadError(#[from] csv::Error),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RawShow {
    show_name: String,
    season_no: i32,
    episode_no: i32,
    episode_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Quote {
    pub show_name: String,
    pub season_no: i32,
    pub episode_no: i32,
    pub character_name: String,
    pub quote_text: String,
}

pub fn shows() -> Result<Vec<Show>, CsvError> {
    let csv_bytes = Cursor::new(include_bytes!("../data/shows.csv"));
    let mut reader = csv::Reader::from_reader(csv_bytes);

    // key is episode_no, value is optional episode_name
    type EpisodeMap = HashMap<i32, Option<String>>;
    // key is season_no, key is episode map
    type SeasonMap = HashMap<i32, EpisodeMap>;
    // key is show_name, value is season map
    type ShowMap = HashMap<String, SeasonMap>;

    let mut show_map: ShowMap = HashMap::new();

    for result in reader.deserialize::<RawShow>() {
        let raw = result?;

        let seasons_for_show = show_map
            .entry(raw.show_name.clone())
            .or_insert(HashMap::new());

        let episodes_for_season = seasons_for_show
            .entry(raw.season_no.clone())
            .or_insert(HashMap::new());

        episodes_for_season
            .entry(raw.episode_no)
            .or_insert(raw.episode_name);
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

    return Ok(shows);
}

pub fn quotes() -> Result<Vec<Quote>, SeedError> {
    let csv_bytes = Cursor::new(include_bytes!("../data/quotes.csv"));
    let mut reader = csv::Reader::from_reader(csv_bytes);
    let quotes = reader
        .deserialize::<Quote>()
        .into_iter()
        .map(|f| f.unwrap())
        .collect::<Vec<Quote>>();
    Ok(quotes)
}
