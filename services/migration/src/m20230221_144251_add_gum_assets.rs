use crate::sea_orm::strum::Display;
use enum_iterator::{all, Sequence};
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

// TODO:  <21-02-23, shek>
// 1. Add post, connection, reaction tables
// 2. Set up the relations properly
// 3. Add the tree config PDA
// 4. Add compression specific columns

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).binary().not_null().primary_key())
                    .col(ColumnDef::new(Users::Authority).binary().not_null())
                    .col(ColumnDef::new(Users::RandomHash).binary().not_null())
                    .to_owned(),
            )
            .await?;

        //create namepsace type
        manager
            .create_type(
                Type::create()
                    .as_enum(Profile::Namespace)
                    .values(vec![
                        Namespace::Personal,
                        Namespace::Professional,
                        Namespace::Degen,
                        Namespace::Gaming,
                    ])
                    .to_owned(),
            )
            .await?;

        // create profile table
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Profile::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Profile::User).binary().not_null())
                    .col(ColumnDef::new(Profile::Namespace).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .from_col(Profile::User),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Authority,
    RandomHash,
}

#[derive(Iden)]
enum Profile {
    Table,
    Id,
    User,
    Namespace,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
enum Namespace {
    Professional,
    Personal,
    Gaming,
    Degen,
}
