use quoted_db_entity::{episode, quote, quote_part, season, show};
use sea_orm::{ActiveValue::NotSet, DatabaseConnection, Set};

// TODO: Refactor to insert many where possible.
// Need to try and reduce the calls to the DB during the seeding process.

use crate::{
    db_helper::{create_character_for_show, idempotent_insert},
    id::IdFactory,
    SeedError,
};

#[derive(Debug)]
pub struct Show {
    pub name: String,
    pub seasons: Vec<Season>,
}
#[derive(Debug)]
pub struct Season {
    pub no: i32,
    pub name: Option<String>,
    pub episodes: Vec<Episode>,
}
#[derive(Debug)]
pub struct Episode {
    pub no: i32,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct Quote {
    pub show_name: String,
    pub season_no: i32,
    pub episode_no: i32,
    pub source_id: i32,
    pub parts: Vec<QuotePart>,
}

#[derive(Debug)]
pub struct QuotePart {
    pub character_name: String,
    pub quote_text: String,
    pub order: i32,
}

pub async fn seed_shows<'a>(
    db: &DatabaseConnection,
    id_factory: &mut IdFactory<'a>,
    shows: Vec<Show>,
) -> Result<(), SeedError> {
    for show in shows {
        seed_show(db, id_factory, show).await?;
    }
    Ok(())
}

async fn seed_show<'a>(
    db: &DatabaseConnection,
    id_factory: &mut IdFactory<'a>,
    show: Show,
) -> Result<(), SeedError> {
    let show_id = id_factory.show.get_id(&show.name, true).await?;

    let model = show::ActiveModel {
        name: Set(show.name),
        id: Set(show_id),
    };

    idempotent_insert(db, model, [show::Column::Id]).await?;

    seed_seasons(db, id_factory, &show_id, show.seasons).await?;

    Ok(())
}

async fn seed_seasons<'a>(
    db: &DatabaseConnection,
    id_factory: &mut IdFactory<'a>,
    show_id: &i32,
    seasons: Vec<Season>,
) -> Result<(), SeedError> {
    for season in seasons {
        seed_season(db, id_factory, show_id, season).await?;
    }
    Ok(())
}

async fn seed_season<'a>(
    db: &DatabaseConnection,
    id_factory: &mut IdFactory<'a>,
    show_id: &i32,
    season: Season,
) -> Result<(), SeedError> {
    let season_id = id_factory.season.get_id(show_id, &season.no, true).await?;

    let mut model = season::ActiveModel {
        id: Set(season_id),
        show_id: Set(*show_id),
        season_no: Set(season.no),
        name: NotSet,
    };
    if season.name.is_some() {
        model.name = Set(season.name)
    }
    let conflict_cols = [season::Column::ShowId, season::Column::SeasonNo];

    idempotent_insert(db, model, conflict_cols).await?;

    seed_episodes(db, id_factory, show_id, &season_id, season.episodes).await?;

    Ok(())
}

async fn seed_episodes<'a>(
    db: &DatabaseConnection,
    id_factory: &mut IdFactory<'a>,
    show_id: &i32,
    season_id: &i32,
    episodes: Vec<Episode>,
) -> Result<(), SeedError> {
    for episode in episodes {
        seed_episode(db, id_factory, show_id, season_id, episode).await?;
    }
    Ok(())
}

async fn seed_episode<'a>(
    db: &DatabaseConnection,
    id_factory: &mut IdFactory<'a>,
    show_id: &i32,
    season_id: &i32,
    episode: Episode,
) -> Result<(), SeedError> {
    let episode_id = id_factory
        .episode
        .get_id(show_id, season_id, &episode.no, true)
        .await?;

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

pub async fn seed_quotes<'a>(
    db: &DatabaseConnection,
    id_factory: &mut IdFactory<'a>,
    quotes: Vec<Quote>,
) -> Result<(), SeedError> {
    for quote in quotes {
        seed_quote(db, id_factory, quote).await?;
    }
    Ok(())
}

async fn seed_quote<'a>(
    db: &DatabaseConnection,
    id_factory: &mut IdFactory<'a>,
    quote: Quote,
) -> Result<(), SeedError> {
    let quote_id = id_factory.quote.get_id(&quote.source_id).await?;
    let show_id = id_factory.show.get_id(&quote.show_name, false).await?;
    let season_id = id_factory
        .season
        .get_id(&show_id, &quote.season_no, false)
        .await?;
    let episode_id = id_factory
        .episode
        .get_id(&show_id, &season_id, &quote.episode_no, false)
        .await?;

    let model = quote::ActiveModel {
        episode_id: Set(episode_id),
        season_id: Set(season_id),
        show_id: Set(show_id),
        source_id: Set(quote.source_id),
        id: Set(quote_id),
    };

    let conflict_cols = [quote::Column::SourceId];

    idempotent_insert(db, model, conflict_cols).await?;

    for part in quote.parts {
        let character_id = id_factory
            .character
            .get_id(&show_id, &part.character_name)
            .await?;

        create_character_for_show(db, id_factory, &show_id, &part.character_name).await?;

        let model = quote_part::ActiveModel {
            character_id: Set(character_id),
            order_no: Set(part.order),
            value: Set(part.quote_text),
            quote_id: Set(quote_id),
            ..Default::default()
        };

        let conflict_cols = [
            quote_part::Column::CharacterId,
            quote_part::Column::QuoteId,
            quote_part::Column::OrderNo,
        ];

        idempotent_insert(db, model, conflict_cols).await?;
    }

    Ok(())
}
