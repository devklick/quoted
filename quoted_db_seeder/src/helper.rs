use prelude::Expr;
use quoted_db::error::DBError;
use quoted_db_entity::{character, character_show, episode, season, show};
use quoted_db_migration::{Func, IntoIden, OnConflict};
use sea_orm::{
    entity::*, query::*, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
};

pub async fn create_character_for_show(
    db: &DatabaseConnection,
    show_id: &i32,
    character_name: &str,
) -> Result<i32, DBError> {
    println!("create_character_for_show, show_id={show_id}, character_name={character_name}");
    let character_id = get_id_for_character(db, show_id, character_name).await?;

    let character = character::ActiveModel {
        id: Set(character_id),
        name: Set(character_name.to_owned()),
        show_id: Set(*show_id),
    };

    idempotent_insert(db, character, [character::Column::Id]).await?;

    let character_show = character_show::ActiveModel {
        character_id: Set(character_id),
        show_id: Set(*show_id),
    };

    idempotent_insert(
        db,
        character_show,
        [
            character_show::Column::CharacterId,
            character_show::Column::ShowId,
        ],
    )
    .await?;

    return Ok(character_id);
}

pub async fn get_id_for_show(
    db: &DatabaseConnection,
    show_name: &str,
    allow_new: bool,
) -> Result<i32, DBError> {
    println!("get_id_for_show (show_name={show_name})");
    let id = show::Entity::find()
        .select_only()
        .column(show::Column::Id)
        .filter(show::Column::Name.eq(show_name))
        .into_tuple::<i32>()
        .one(db)
        .await?;

    if let Some(id) = id {
        println!("get_id_for_show - found existing id {id} for show_name={show_name}");
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
        .one(db)
        .await?;

    let id = match max {
        Some(m) => m + 1,
        None => 1,
    };

    println!("get_id_for_show - using next id {id} for show_name={show_name}");

    return Ok(id);
}

pub async fn get_id_for_season(
    db: &DatabaseConnection,
    show_id: &i32,
    season_no: &i32,
    allow_new: bool,
) -> Result<i32, DBError> {
    println!("get_id_for_season (show_id={show_id}, season_no={season_no})");
    let id = season::Entity::find()
        .select_only()
        .column(season::Column::Id)
        .filter(
            season::Column::ShowId
                .eq(*show_id)
                .and(season::Column::SeasonNo.eq(*season_no)),
        )
        .into_tuple::<i32>()
        .one(db)
        .await?;

    if let Some(id) = id {
        println!("get_id_for_season - found existing id {id} for show_id={show_id}, season_no={season_no}");
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
        .one(db)
        .await?;

    let id = match max {
        Some(m) => m + 1,
        None => 1,
    };

    println!("get_id_for_season - using next id {id} for show_id={show_id}, season_no={season_no}");

    return Ok(id);
}

pub async fn get_id_for_episode(
    db: &DatabaseConnection,
    show_id: &i32,
    season_id: &i32,
    episode_no: &i32,
    allow_new: bool,
) -> Result<i32, DBError> {
    println!(
        "get_id_for_episode (show_id={show_id}, season_id={season_id}, episode_no={episode_no})"
    );
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
        .one(db)
        .await?;

    if let Some(id) = id {
        println!("get_id_for_episode - found existing id {id} for show_id={show_id}, season_id={season_id}, episode_no={episode_no}");
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
        .one(db)
        .await?;

    let id = match max {
        Some(m) => m + 1,
        None => 1,
    };

    println!(
        "get_id_for_episode - using next id {id} for show_id={show_id}, season_id={season_id}, episode_no={episode_no}"
    );

    return Ok(id);
}

pub async fn get_id_for_character(
    db: &DatabaseConnection,
    show_id: &i32,
    character_name: &str,
) -> Result<i32, DBError> {
    println!("get_id_for_character (show_id={show_id}, character_name={character_name})");

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
        .one(db)
        .await?;

    if let Some(id) = id {
        println!("get_id_for_character - found existing id {id} for show_id={show_id}, character_name={character_name}");
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
        .one(db)
        .await?;

    let id = match max {
        Some(m) => m + 1,
        None => 1,
    };

    println!(
        "get_id_for_character - using next id {id} for show_id={show_id}, character_name={character_name}"
    );

    return Ok(id);
}

pub async fn idempotent_insert<A, I, C>(
    db: &DatabaseConnection,
    model: A,
    conflict_cols: I,
) -> Result<(), DBError>
where
    A: ActiveModelTrait + ActiveModelBehavior + Send + 'static,
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    // E: EntityTrait + 'static,
    C: IntoIden,
    I: IntoIterator<Item = C>,
{
    let on_conflict = OnConflict::columns(conflict_cols).do_nothing().to_owned();

    <A::Entity as EntityTrait>::insert(model)
        .on_conflict(on_conflict)
        .do_nothing()
        .exec(db)
        .await?;

    return Ok(());
}
