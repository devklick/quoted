//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "show")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::character::Entity")]
    Character,
    #[sea_orm(has_many = "super::character_show::Entity")]
    CharacterShow,
    #[sea_orm(has_many = "super::episode::Entity")]
    Episode,
    #[sea_orm(has_many = "super::quote::Entity")]
    Quote,
    #[sea_orm(has_many = "super::season::Entity")]
    Season,
}

impl Related<super::character_show::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterShow.def()
    }
}

impl Related<super::episode::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Episode.def()
    }
}

impl Related<super::quote::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Quote.def()
    }
}

impl Related<super::season::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Season.def()
    }
}

impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        super::character_show::Relation::Character.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::character_show::Relation::Show.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}