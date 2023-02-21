pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230220_143646_add_tasks;
mod m20230221_144251_add_gum_assets;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230220_143646_add_tasks::Migration),
            Box::new(m20230221_144251_add_gum_assets::Migration),
        ]
    }
}
