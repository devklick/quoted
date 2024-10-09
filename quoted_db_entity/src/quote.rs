//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "quote")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub show_id: i32,
    pub season_id: i32,
    pub episode_id: i32,
    #[sea_orm(unique)]
    pub source_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::episode::Entity",
        from = "Column::EpisodeId",
        to = "super::episode::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Episode,
    #[sea_orm(has_many = "super::quote_part::Entity")]
    QuotePart,
    #[sea_orm(
        belongs_to = "super::season::Entity",
        from = "Column::SeasonId",
        to = "super::season::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Season,
    #[sea_orm(
        belongs_to = "super::show::Entity",
        from = "Column::ShowId",
        to = "super::show::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Show,
}

impl Related<super::episode::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Episode.def()
    }
}

impl Related<super::quote_part::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QuotePart.def()
    }
}

impl Related<super::season::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Season.def()
    }
}

impl Related<super::show::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Show.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
