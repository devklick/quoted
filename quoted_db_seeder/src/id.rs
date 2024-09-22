use std::collections::HashMap;

use prelude::Expr;
use quoted_db_migration::Func;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

use quoted_db::error::DBError;
use quoted_db_entity::{character, character_show, episode, season, show};

#[derive(PartialEq, Eq, Hash)]
pub struct CharacterIdKey {
    show_id: i32,
    character_name: String,
}

#[derive(PartialEq, Eq, Hash)]
pub struct ShowIdKey {
    show_name: String,
}

#[derive(PartialEq, Eq, Hash)]
pub struct SeasonIdKey {
    show_id: i32,
    season_no: i32,
}

#[derive(PartialEq, Eq, Hash)]
pub struct EpisodeIdKey {
    show_id: i32,
    season_id: i32,
    episode_no: i32,
}

pub struct IdCache {
    character_ids: HashMap<CharacterIdKey, i32>,
    show_ids: HashMap<ShowIdKey, i32>,
    season_ids: HashMap<SeasonIdKey, i32>,
    episode_ids: HashMap<EpisodeIdKey, i32>,
}

impl IdCache {
    pub fn new() -> IdCache {
        IdCache {
            character_ids: HashMap::new(),
            episode_ids: HashMap::new(),
            season_ids: HashMap::new(),
            show_ids: HashMap::new(),
        }
    }
}

pub struct IdFactory {
    id_cache: IdCache,
    db: DatabaseConnection,
}

impl IdFactory {
    pub fn new(db: DatabaseConnection) -> Self {
        IdFactory {
            db,
            id_cache: IdCache::new(),
        }
    }

    pub async fn get_id_for_show(
        &mut self,
        show_name: &str,
        allow_new: bool,
    ) -> Result<i32, DBError> {
        println!("get_id_for_show (show_name={show_name})");
        let key = ShowIdKey {
            show_name: show_name.to_string(),
        };
        if self.id_cache.show_ids.contains_key(&key) {
            let id = self.id_cache.show_ids[&key];
            println!(
                "get_id_for_show - found existing id {id} in memory for show_name={show_name}"
            );
            return Ok(id);
        }

        let id = show::Entity::find()
            .select_only()
            .column(show::Column::Id)
            .filter(show::Column::Name.eq(show_name))
            .into_tuple::<i32>()
            .one(&self.db)
            .await?;

        if let Some(id) = id {
            println!("get_id_for_show - found existing id {id} for show_name={show_name}");
            self.id_cache.show_ids.insert(key, id);
            return Ok(id);
        }

        if !allow_new {
            return Err(DBError::MissingInsertDependency);
        }

        let max = show::Entity::find()
            .select_only()
            .expr(
                Func::coalesce([
                    Expr::col(episode::Column::Id).max().into(),
                    Expr::val(0).into(),
                ])
                .to_owned(),
            )
            .into_tuple::<i32>()
            .one(&self.db)
            .await?;

        let id = match max {
            Some(m) => m + 1,
            None => 1,
        };
        self.id_cache.show_ids.insert(key, id);

        println!("get_id_for_show - using next id {id} for show_name={show_name}");

        return Ok(id);
    }

    pub async fn get_id_for_season(
        &mut self,
        show_id: &i32,
        season_no: &i32,
        allow_new: bool,
    ) -> Result<i32, DBError> {
        println!("get_id_for_season (show_id={show_id}, season_no={season_no})");
        let key = SeasonIdKey {
            show_id: *show_id,
            season_no: *season_no,
        };
        if self.id_cache.season_ids.contains_key(&key) {
            let id = self.id_cache.season_ids[&key];
            println!(
                "get_id_for_season - found existing id {id} in memory for for show_id={show_id}, season_no={season_no}"
            );
            return Ok(id);
        }
        let id = season::Entity::find()
            .select_only()
            .column(season::Column::Id)
            .filter(
                season::Column::ShowId
                    .eq(*show_id)
                    .and(season::Column::SeasonNo.eq(*season_no)),
            )
            .into_tuple::<i32>()
            .one(&self.db)
            .await?;

        if let Some(id) = id {
            println!("get_id_for_season - found existing id {id} for show_id={show_id}, season_no={season_no}");
            self.id_cache.season_ids.insert(key, id);
            return Ok(id);
        }

        if !allow_new {
            return Err(DBError::MissingInsertDependency);
        }

        let max = season::Entity::find()
            .select_only()
            .expr(
                Func::coalesce([
                    Expr::col(season::Column::Id).max().into(),
                    Expr::val(0).into(),
                ])
                .to_owned(),
            )
            .into_tuple::<i32>()
            .one(&self.db)
            .await?;

        let id = match max {
            Some(m) => m + 1,
            None => 1,
        };

        println!(
            "get_id_for_season - using next id {id} for show_id={show_id}, season_no={season_no}"
        );
        self.id_cache.season_ids.insert(key, id);

        return Ok(id);
    }

    pub async fn get_id_for_episode(
        &mut self,
        show_id: &i32,
        season_id: &i32,
        episode_no: &i32,
        allow_new: bool,
    ) -> Result<i32, DBError> {
        println!(
            "get_id_for_episode (show_id={show_id}, season_id={season_id}, episode_no={episode_no})"
        );
        let key = EpisodeIdKey {
            show_id: *show_id,
            season_id: *season_id,
            episode_no: *episode_no,
        };
        if self.id_cache.episode_ids.contains_key(&key) {
            let id = self.id_cache.episode_ids[&key];
            println!(
                "get_id_for_episode - found existing id {id} in memory for show_id={show_id}, season_id={season_id}, episode_no={episode_no}"
            );
            return Ok(id);
        }

        let id = episode::Entity::find()
            .select_only()
            .column(episode::Column::Id)
            .filter(
                episode::Column::ShowId
                    .eq(*show_id)
                    .and(episode::Column::SeasonId.eq(*season_id))
                    .and(episode::Column::EpisodeNo.eq(*episode_no)),
            )
            .into_tuple::<i32>()
            .one(&self.db)
            .await?;

        if let Some(id) = id {
            println!("get_id_for_episode - found existing id {id} for show_id={show_id}, season_id={season_id}, episode_no={episode_no}");
            self.id_cache.episode_ids.insert(key, id);
            return Ok(id);
        }
        if !allow_new {
            return Err(DBError::MissingInsertDependency);
        }
        let max = episode::Entity::find()
            .select_only()
            .expr(
                Func::coalesce([
                    Expr::col(episode::Column::Id).max().into(),
                    Expr::val(0).into(),
                ])
                .to_owned(),
            )
            .into_tuple::<i32>()
            .one(&self.db)
            .await?;

        let id = match max {
            Some(m) => m + 1,
            None => 1,
        };

        println!(
            "get_id_for_episode - using next id {id} for show_id={show_id}, season_id={season_id}, episode_no={episode_no}"
        );
        self.id_cache.episode_ids.insert(key, id);

        return Ok(id);
    }

    pub async fn get_id_for_character(
        &mut self,
        show_id: &i32,
        character_name: &str,
    ) -> Result<i32, DBError> {
        println!("get_id_for_character (show_id={show_id}, character_name={character_name})");

        let key = CharacterIdKey {
            character_name: character_name.to_string(),
            show_id: *show_id,
        };
        if self.id_cache.character_ids.contains_key(&key) {
            let id = self.id_cache.character_ids[&key];
            println!(
                "get_id_for_character - found existing id {id} in memory for show_id={show_id}, character_name={character_name}"
            );
            return Ok(id);
        }

        let id = character::Entity::find()
            .select_only()
            .column(character::Column::Id)
            .join(
                JoinType::InnerJoin,
                character_show::Relation::Character.def().rev(),
            )
            .filter(
                character::Column::Name
                    .eq(character_name)
                    .and(character_show::Column::ShowId.eq(*show_id)),
            )
            .into_tuple::<i32>()
            .one(&self.db)
            .await?;

        if let Some(id) = id {
            println!("get_id_for_character - found existing id {id} for show_id={show_id}, character_name={character_name}");
            self.id_cache.character_ids.insert(key, id);
            return Ok(id);
        }
        let max = character::Entity::find()
            .select_only()
            .expr(
                Func::coalesce([
                    Expr::col(character::Column::Id).max().into(),
                    Expr::val(0).into(),
                ])
                .to_owned(),
            )
            .into_tuple::<i32>()
            .one(&self.db)
            .await?;

        let id = match max {
            Some(m) => m + 1,
            None => 1,
        };

        println!(
            "get_id_for_character - using next id {id} for show_id={show_id}, character_name={character_name}"
        );
        self.id_cache.character_ids.insert(key, id);

        return Ok(id);
    }
}
