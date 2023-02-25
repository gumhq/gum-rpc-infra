use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // add compressed, tree_id, leaf, nonce columns to post table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("post"))
                    .add_column(
                        ColumnDef::new(Alias::new("compressed"))
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .add_column(ColumnDef::new(Alias::new("tree_id")).binary())
                    .add_column(ColumnDef::new(Alias::new("leaf")).binary())
                    .add_column(
                        ColumnDef::new(Alias::new("nonce"))
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        // create an index on tree_id for post table

        manager
            .create_index(
                Index::create()
                    .table(Alias::new("post"))
                    .name("post_tree_id")
                    .col(Alias::new("tree_id"))
                    .to_owned(),
            )
            .await?;

        // create an index on tree_id and leaf for post table

        manager
            .create_index(
                Index::create()
                    .table(Alias::new("post"))
                    .name("post_tree_id_leaf")
                    .col(Alias::new("tree_id"))
                    .col(Alias::new("leaf"))
                    .to_owned(),
            )
            .await?;

        // create an index on tree_id, leaf and nonce for post table

        manager
            .create_index(
                Index::create()
                    .table(Alias::new("post"))
                    .name("post_tree_id_leaf_nonce")
                    .col(Alias::new("tree_id"))
                    .col(Alias::new("leaf"))
                    .col(Alias::new("nonce"))
                    .to_owned(),
            )
            .await?;

        // add compressed, tree_id, leaf, nonce columns to connection table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("connection"))
                    .add_column(
                        ColumnDef::new(Alias::new("compressed"))
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .add_column(ColumnDef::new(Alias::new("tree_id")).binary())
                    .add_column(ColumnDef::new(Alias::new("leaf")).binary())
                    .add_column(
                        ColumnDef::new(Alias::new("nonce"))
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        // create an index on tree_id for connection table

        manager
            .create_index(
                Index::create()
                    .table(Alias::new("connection"))
                    .name("connection_tree_id")
                    .col(Alias::new("tree_id"))
                    .to_owned(),
            )
            .await?;

        // create an index on tree_id and leaf for connection table

        manager
            .create_index(
                Index::create()
                    .table(Alias::new("connection"))
                    .name("connection_tree_id_leaf")
                    .col(Alias::new("tree_id"))
                    .col(Alias::new("leaf"))
                    .to_owned(),
            )
            .await?;

        // create an index on tree_id, leaf and nonce for connection table

        manager
            .create_index(
                Index::create()
                    .table(Alias::new("connection"))
                    .name("connection_tree_id_leaf_nonce")
                    .col(Alias::new("tree_id"))
                    .col(Alias::new("leaf"))
                    .col(Alias::new("nonce"))
                    .to_owned(),
            )
            .await?;

        // add compressed, tree_id, leaf, nonce columns to reaction table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("reaction"))
                    .add_column(
                        ColumnDef::new(Alias::new("compressed"))
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .add_column(ColumnDef::new(Alias::new("tree_id")).binary())
                    .add_column(ColumnDef::new(Alias::new("leaf")).binary())
                    .add_column(
                        ColumnDef::new(Alias::new("nonce"))
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        // create an index on tree_id for reaction table

        manager
            .create_index(
                Index::create()
                    .table(Alias::new("reaction"))
                    .name("reaction_tree_id")
                    .col(Alias::new("tree_id"))
                    .to_owned(),
            )
            .await?;

        // create an index on tree_id and leaf for reaction table

        manager
            .create_index(
                Index::create()
                    .table(Alias::new("reaction"))
                    .name("reaction_tree_id_leaf")
                    .col(Alias::new("tree_id"))
                    .col(Alias::new("leaf"))
                    .to_owned(),
            )
            .await?;

        // create an index on tree_id, leaf and nonce for reaction table

        manager
            .create_index(
                Index::create()
                    .table(Alias::new("reaction"))
                    .name("reaction_tree_id_leaf_nonce")
                    .col(Alias::new("tree_id"))
                    .col(Alias::new("leaf"))
                    .col(Alias::new("nonce"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // drop compressed, tree_id, leaf, nonce columns from post table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("post"))
                    .drop_column(Alias::new("compressed"))
                    .drop_column(Alias::new("tree_id"))
                    .drop_column(Alias::new("leaf"))
                    .drop_column(Alias::new("nonce"))
                    .to_owned(),
            )
            .await?;

        // drop compressed, tree_id, leaf, nonce columns from connection table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("connection"))
                    .drop_column(Alias::new("compressed"))
                    .drop_column(Alias::new("tree_id"))
                    .drop_column(Alias::new("leaf"))
                    .drop_column(Alias::new("nonce"))
                    .to_owned(),
            )
            .await?;
        // drop compressed, tree_id, leaf, nonce columns from reaction table

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("reaction"))
                    .drop_column(Alias::new("compressed"))
                    .drop_column(Alias::new("tree_id"))
                    .drop_column(Alias::new("leaf"))
                    .drop_column(Alias::new("nonce"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
