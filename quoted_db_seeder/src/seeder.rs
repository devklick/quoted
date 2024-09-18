use quoted_db_entity::{episode, quote, season, show};
use sea_orm::{DatabaseConnection, Set};

// TODO: Refactor to insert many where possible.
// Need to try and reduce the calls to the DB during the seeding process.
// Perhaps rather than calling the DB to get an ID of a record we already know exists,
// it can be fetched once and stored in memory.

use crate::{
    helper::{
        create_character_for_show, get_id_for_character, get_id_for_episode, get_id_for_season,
        get_id_for_show, idempotent_insert,
    },
    parse_csv::{Episode, Quote, Season, Show},
    SeedError,
};

pub async fn seed_shows(db: &DatabaseConnection, shows: Vec<Show>) -> Result<(), SeedError> {
    for show in shows {
        seed_show(db, show).await?;
    }
    Ok(())
}

async fn seed_show(db: &DatabaseConnection, show: Show) -> Result<(), SeedError> {
    let show_id = get_id_for_show(db, &show.name, true).await?;
    let model = show::ActiveModel {
        name: Set(show.name),
        id: Set(show_id),
    };

    idempotent_insert(db, model, [show::Column::Id]).await?;

    seed_seasons(db, &show_id, show.seasons).await?;

    Ok(())
}

async fn seed_seasons(
    db: &DatabaseConnection,
    show_id: &i32,
    seasons: Vec<Season>,
) -> Result<(), SeedError> {
    for season in seasons {
        seed_season(db, show_id, season).await?;
    }
    Ok(())
}

async fn seed_season(
    db: &DatabaseConnection,
    show_id: &i32,
    season: Season,
) -> Result<(), SeedError> {
    let season_id = get_id_for_season(db, show_id, &season.no, true).await?;
    let model = season::ActiveModel {
        id: Set(season_id),
        show_id: Set(*show_id),
        season_no: Set(season.no),
    };
    let conflict_cols = [season::Column::ShowId, season::Column::SeasonNo];

    idempotent_insert(db, model, conflict_cols).await?;

    seed_episodes(db, show_id, &season_id, season.episodes).await?;

    Ok(())
}

async fn seed_episodes(
    db: &DatabaseConnection,
    show_id: &i32,
    season_id: &i32,
    episodes: Vec<Episode>,
) -> Result<(), SeedError> {
    for episode in episodes {
        seed_episode(db, show_id, season_id, episode).await?;
    }
    Ok(())
}

async fn seed_episode(
    db: &DatabaseConnection,
    show_id: &i32,
    season_id: &i32,
    episode: Episode,
) -> Result<(), SeedError> {
    let episode_id = get_id_for_episode(db, show_id, season_id, &episode.no, true).await?;

    let model = episode::ActiveModel {
        episode_no: Set(episode.no),
        name: Set(episode.name),
        show_id: Set(*show_id),
        season_id: Set(*season_id),
        id: Set(episode_id),
    };

    let conflict_cols = [
        episode::Column::ShowId,
        episode::Column::SeasonId,
        episode::Column::EpisodeNo,
    ];

    idempotent_insert(db, model, conflict_cols).await?;
    Ok(())
}

pub async fn seed_quotes(db: &DatabaseConnection, quotes: Vec<Quote>) -> Result<(), SeedError> {
    for quote in quotes {
        seed_quote(db, quote).await?;
    }
    Ok(())
}

async fn seed_quote(db: &DatabaseConnection, quote: Quote) -> Result<(), SeedError> {
    let show_id = get_id_for_show(db, &quote.show_name, false).await?;
    let season_id = get_id_for_season(db, &show_id, &quote.season_no, false).await?;
    let episode_id = get_id_for_episode(db, &show_id, &season_id, &quote.episode_no, false).await?;
    let character_id = get_id_for_character(db, &show_id, &quote.character_name).await?;
    create_character_for_show(db, &show_id, &quote.character_name).await?;

    let model = quote::ActiveModel {
        character_id: Set(character_id),
        episode_id: Set(episode_id),
        season_id: Set(season_id),
        show_id: Set(show_id),
        value: Set(quote.quote_text),
        ..Default::default()
    };

    let conflict_cols = [
        quote::Column::CharacterId,
        quote::Column::EpisodeId,
        quote::Column::SeasonId,
        quote::Column::ShowId,
        quote::Column::Value,
    ];

    idempotent_insert(db, model, conflict_cols).await?;

    Ok(())
}
