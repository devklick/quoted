pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_show_table;
mod m20240914_000500_create_character_table;
mod m20240914_080125_create_season_n_episode_tables;
mod m20240914_162612_create_show_character_table;
mod m20240914_200407_create_quote_table;
mod m20240914_214423_add_unique_constraint_to_quote;
mod m20241001_185027_add_season_name;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_show_table::Migration),
            Box::new(m20240914_000500_create_character_table::Migration),
            Box::new(m20240914_080125_create_season_n_episode_tables::Migration),
            Box::new(m20240914_162612_create_show_character_table::Migration),
            Box::new(m20240914_200407_create_quote_table::Migration),
            Box::new(m20240914_214423_add_unique_constraint_to_quote::Migration),
            Box::new(m20241001_185027_add_season_name::Migration),
        ]
    }
}
