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

pub struct ShowIdFactory<'a> {
    cache: HashMap<ShowIdKey, i32>,
    db: &'a DatabaseConnection,
}
pub struct EpisodeIdFactory<'a> {
    cache: HashMap<EpisodeIdKey, i32>,
    db: &'a DatabaseConnection,
}
pub struct SeasonIdFactory<'a> {
    cache: HashMap<SeasonIdKey, i32>,
    db: &'a DatabaseConnection,
}
pub struct CharacterIdFactory<'a> {
    cache: HashMap<CharacterIdKey, i32>,
    db: &'a DatabaseConnection,
}

impl<'a> ShowIdFactory<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        ShowIdFactory {
            db,
            cache: HashMap::new(),
        }
    }

    pub async fn get_id(&mut self, show_name: &str, allow_new: bool) -> Result<i32, DBError> {
        println!("get_id_for_show (show_name={show_name})");
        let key = ShowIdKey {
            show_name: show_name.to_string(),
        };
        if self.cache.contains_key(&key) {
            let id = self.cache[&key];
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
            .one(self.db)
            .await?;

        if let Some(id) = id {
            println!("get_id_for_show - found existing id {id} for show_name={show_name}");
            self.cache.insert(key, id);
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
            .one(self.db)
            .await?;

        let id = match max {
            Some(m) => m + 1,
            None => 1,
        };
        self.cache.insert(key, id);

        println!("get_id_for_show - using next id {id} for show_name={show_name}");

        return Ok(id);
    }
}

impl<'a> SeasonIdFactory<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        SeasonIdFactory {
            db,
            cache: HashMap::new(),
        }
    }
    pub async fn get_id(
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
        if self.cache.contains_key(&key) {
            let id = self.cache[&key];
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
            .one(self.db)
            .await?;

        if let Some(id) = id {
            println!("get_id_for_season - found existing id {id} for show_id={show_id}, season_no={season_no}");
            self.cache.insert(key, id);
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
            .one(self.db)
            .await?;

        let id = match max {
            Some(m) => m + 1,
            None => 1,
        };

        println!(
            "get_id_for_season - using next id {id} for show_id={show_id}, season_no={season_no}"
        );
        self.cache.insert(key, id);

        return Ok(id);
    }
}

impl<'a> EpisodeIdFactory<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        EpisodeIdFactory {
            db,
            cache: HashMap::new(),
        }
    }
    pub async fn get_id(
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
        if self.cache.contains_key(&key) {
            let id = self.cache[&key];
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
            .one(self.db)
            .await?;

        if let Some(id) = id {
            println!("get_id_for_episode - found existing id {id} for show_id={show_id}, season_id={season_id}, episode_no={episode_no}");
            self.cache.insert(key, id);
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
            .one(self.db)
            .await?;

        let id = match max {
            Some(m) => m + 1,
            None => 1,
        };

        println!(
            "get_id_for_episode - using next id {id} for show_id={show_id}, season_id={season_id}, episode_no={episode_no}"
        );
        self.cache.insert(key, id);

        return Ok(id);
    }
}

impl<'a> CharacterIdFactory<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        CharacterIdFactory {
            db,
            cache: HashMap::new(),
        }
    }
    pub async fn get_id(&mut self, show_id: &i32, character_name: &str) -> Result<i32, DBError> {
        println!("get_id_for_character (show_id={show_id}, character_name={character_name})");

        let key = CharacterIdKey {
            character_name: character_name.to_string(),
            show_id: *show_id,
        };
        if self.cache.contains_key(&key) {
            let id = self.cache[&key];
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
            .one(self.db)
            .await?;

        if let Some(id) = id {
            println!("get_id_for_character - found existing id {id} for show_id={show_id}, character_name={character_name}");
            self.cache.insert(key, id);
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
            .one(self.db)
            .await?;

        let id = match max {
            Some(m) => m + 1,
            None => 1,
        };

        println!(
            "get_id_for_character - using next id {id} for show_id={show_id}, character_name={character_name}"
        );
        self.cache.insert(key, id);

        return Ok(id);
    }
}

pub struct IdFactory<'a> {
    pub show: ShowIdFactory<'a>,
    pub season: SeasonIdFactory<'a>,
    pub episode: EpisodeIdFactory<'a>,
    pub character: CharacterIdFactory<'a>,
}

impl<'a> IdFactory<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        IdFactory {
            show: ShowIdFactory::new(db),
            season: SeasonIdFactory::new(db),
            episode: EpisodeIdFactory::new(db),
            character: CharacterIdFactory::new(db),
        }
    }
}
