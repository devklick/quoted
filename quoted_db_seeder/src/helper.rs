use quoted_db::error::DBError;
use quoted_db_entity::{character, character_show};
use quoted_db_migration::{IntoIden, OnConflict};
use sea_orm::{entity::*, DatabaseConnection, EntityTrait};

use crate::id::IdFactory;

pub async fn create_character_for_show<'a>(
    db: &DatabaseConnection,
    id_factory: &mut IdFactory<'a>,
    show_id: &i32,
    character_name: &str,
) -> Result<i32, DBError> {
    println!("create_character_for_show, show_id={show_id}, character_name={character_name}");
    let character_id = id_factory.character.get_id(show_id, character_name).await?;

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

pub async fn idempotent_insert<A, I, C>(
    db: &DatabaseConnection,
    model: A,
    conflict_cols: I,
) -> Result<(), DBError>
where
    A: ActiveModelTrait + ActiveModelBehavior + Send + 'static,
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
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
