use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let init_file_path = env::var("INIT_FILE_PATH").expect("INIT_FILE_PATH must be set");
        let mut file = File::open(init_file_path).map_err(|e| DbErr::Custom(e.to_string()))?;
        let mut sql = String::new();
        file.read_to_string(&mut sql)
            .map_err(|e| DbErr::Custom(e.to_string()))?;
        let sqls: Vec<&str> = sql.split("-- @@@@@@").collect();
        for sqlst in sqls {
            let stmt = Statement::from_string(manager.get_database_backend(), sqlst.to_string());
            manager.get_connection().execute(stmt).await.map(|_| ())?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            DROP TABLE IF EXISTS `raw_txn` CASCADE;
            DROP TABLE IF EXISTS `cl_items` CASCADE;
            DROP TABLE IF EXISTS `backfill_items` CASCADE;
            DROP FUNCTION IF EXISTS `notify_new_backfill_item()`;
            DROP TRIGGER IF EXISTS `after_insert_item` ON `backfill_items`;
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
