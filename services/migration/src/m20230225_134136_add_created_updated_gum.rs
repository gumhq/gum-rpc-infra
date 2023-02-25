use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // add created_at and updated_at to users table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("users"))
                    .add_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("default (now() at time zone 'utc')".to_owned()),
                    )
                    .add_column(
                        ColumnDef::new(Alias::new("slot_updated_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // add created_at and slot_updated_at columns to  profile table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("profile"))
                    .add_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("default (now() at time zone 'utc')".to_owned()),
                    )
                    .add_column(
                        ColumnDef::new(Alias::new("slot_updated_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // add created_at and slot_updated_at columns to  profile_metadata table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("profile_metadata"))
                    .add_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("default (now() at time zone 'utc')".to_owned()),
                    )
                    .add_column(
                        ColumnDef::new(Alias::new("slot_updated_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // add created_at and slot_updated_at columns to post table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("post"))
                    .add_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("default (now() at time zone 'utc')".to_owned()),
                    )
                    .add_column(
                        ColumnDef::new(Alias::new("slot_updated_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // add created_at and slot_updated_at columns to reaction table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("reaction"))
                    .add_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("default (now() at time zone 'utc')".to_owned()),
                    )
                    .add_column(
                        ColumnDef::new(Alias::new("slot_updated_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // add created_at and slot_updated_at columns to connection table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("connection"))
                    .add_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("default (now() at time zone 'utc')".to_owned()),
                    )
                    .add_column(
                        ColumnDef::new(Alias::new("slot_updated_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // drop created_at and slot_updated_at columns from users table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("users"))
                    .drop_column(Alias::new("created_at"))
                    .drop_column(Alias::new("slot_updated_at"))
                    .to_owned(),
            )
            .await?;

        // drop created_at and slot_updated_at columns from profile table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("profile"))
                    .drop_column(Alias::new("created_at"))
                    .drop_column(Alias::new("slot_updated_at"))
                    .to_owned(),
            )
            .await?;

        // drop created_at and slot_updated_at columns from profile_metadata table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("profile_metadata"))
                    .drop_column(Alias::new("created_at"))
                    .drop_column(Alias::new("slot_updated_at"))
                    .to_owned(),
            )
            .await?;

        // drop created_at and slot_updated_at columns from post table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("post"))
                    .drop_column(Alias::new("created_at"))
                    .drop_column(Alias::new("slot_updated_at"))
                    .to_owned(),
            )
            .await?;

        // drop created_at and slot_updated_at columns from reaction table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("reaction"))
                    .drop_column(Alias::new("created_at"))
                    .drop_column(Alias::new("slot_updated_at"))
                    .to_owned(),
            )
            .await?;

        // drop created_at and slot_updated_at columns from connection table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("connection"))
                    .drop_column(Alias::new("created_at"))
                    .drop_column(Alias::new("slot_updated_at"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
