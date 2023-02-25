use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create tree config table

        manager
            .create_table(
                Table::create()
                    .table(TreeConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TreeConfig::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TreeConfig::Authority)
                            .binary()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(TreeConfig::MerkleTree).binary().not_null())
                    .col(
                        ColumnDef::new(TreeConfig::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("default (now() at time zone 'utc')".to_owned()),
                    )
                    .col(
                        ColumnDef::new(TreeConfig::SlotUpdateAt)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TreeConfig::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum TreeConfig {
    Table,
    Id,
    Authority,
    MerkleTree,
    CreatedAt,
    SlotUpdateAt,
}
