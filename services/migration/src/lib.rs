pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230220_143646_add_tasks;
mod m20230221_144251_add_gum_assets;
mod m20230225_131241_add_tree_config;
mod m20230225_134136_add_created_updated_gum;
mod m20230225_143442_add_burnt_gum;
mod m20230225_144330_add_user_transfer_log;
mod m20230225_163111_add_gum_compression;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230220_143646_add_tasks::Migration),
            Box::new(m20230221_144251_add_gum_assets::Migration),
            Box::new(m20230225_131241_add_tree_config::Migration),
            Box::new(m20230225_134136_add_created_updated_gum::Migration),
            Box::new(m20230225_143442_add_burnt_gum::Migration),
            Box::new(m20230225_144330_add_user_transfer_log::Migration),
            Box::new(m20230225_163111_add_gum_compression::Migration),
        ]
    }
}
