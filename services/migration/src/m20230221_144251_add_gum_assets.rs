use enum_iterator::Sequence;
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

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

        // create profile metadata table
        manager
            .create_table(
                Table::create()
                    .table(ProfileMetadata::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProfileMetadata::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProfileMetadata::Profile).binary().not_null())
                    .col(
                        ColumnDef::new(ProfileMetadata::MetadataUri)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProfileMetadata::MetadataUriContent)
                            .json_binary()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to_tbl(Profile::Table)
                            .to_col(Profile::Id)
                            .from_col(ProfileMetadata::Profile),
                    )
                    .to_owned(),
            )
            .await?;

        // create post table
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Post::Id).binary().not_null().primary_key())
                    .col(ColumnDef::new(Post::Profile).binary().not_null())
                    .col(ColumnDef::new(Post::RandomHash).binary().not_null())
                    .col(ColumnDef::new(Post::ReplyTo).binary())
                    .col(ColumnDef::new(Post::MetadataUri).string().not_null())
                    .col(
                        ColumnDef::new(Post::MetadataUriContent)
                            .json_binary()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to_tbl(Profile::Table)
                            .to_col(Post::Id)
                            .from_col(Post::Profile),
                    )
                    .to_owned(),
            )
            .await?;

        // create post index on profile and reply to
        manager
            .create_index(
                Index::create()
                    .table(Post::Table)
                    .name("post_profile_reply_to")
                    .col(Post::Profile)
                    .col(Post::ReplyTo)
                    .to_owned(),
            )
            .await?;

        // create connection table
        manager
            .create_table(
                Table::create()
                    .table(Connection::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Connection::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Connection::FromProfile).binary().not_null())
                    .col(ColumnDef::new(Connection::ToProfile).binary().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .to_tbl(Profile::Table)
                            .to_col(Profile::Id)
                            .from_col(Connection::FromProfile),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to_tbl(Profile::Table)
                            .to_col(Profile::Id)
                            .from_col(Connection::ToProfile),
                    )
                    .to_owned(),
            )
            .await?;

        // create an index on from profile and to profile
        // this will be used to check if a connection exists
        manager
            .create_index(
                Index::create()
                    .table(Connection::Table)
                    .name("connection_from_to")
                    .col(Connection::FromProfile)
                    .col(Connection::ToProfile)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // create reaction type
        manager
            .create_type(
                Type::create()
                    .as_enum(Reaction::ReactionType)
                    .values(vec![
                        ReactionType::Like,
                        ReactionType::Dislike,
                        ReactionType::Love,
                        ReactionType::Haha,
                        ReactionType::Wow,
                        ReactionType::Sad,
                        ReactionType::Angry,
                    ])
                    .to_owned(),
            )
            .await?;

        // create reaction table
        manager
            .create_table(
                Table::create()
                    .table(Reaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reaction::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reaction::FromProfile).binary().not_null())
                    .col(ColumnDef::new(Reaction::ToPost).binary().not_null())
                    .col(ColumnDef::new(Reaction::ReactionType).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .to_tbl(Profile::Table)
                            .to_col(Profile::Id)
                            .from_col(Reaction::FromProfile),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to_tbl(Post::Table)
                            .to_col(Post::Id)
                            .from_col(Reaction::ToPost),
                    )
                    .to_owned(),
            )
            .await?;

        // create an index on from profile and to post
        manager
            .create_index(
                Index::create()
                    .table(Reaction::Table)
                    .name("reaction_from_to")
                    .col(Reaction::FromProfile)
                    .col(Reaction::ToPost)
                    .unique()
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
        manager
            .drop_table(Table::drop().table(ProfileMetadata::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Connection::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Reaction::Table).to_owned())
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

#[derive(Iden, Debug, PartialEq, Sequence)]
enum Namespace {
    Professional,
    Personal,
    Gaming,
    Degen,
}

#[derive(Iden)]
enum Profile {
    Table,
    Id,
    User,
    Namespace,
}

#[derive(Iden)]
enum ProfileMetadata {
    Table,
    Id,
    Profile,
    MetadataUri,
    MetadataUriContent,
}

#[derive(Iden)]
enum Post {
    Table,
    Id,
    Profile,
    RandomHash,
    MetadataUri,
    MetadataUriContent,
    ReplyTo,
}

#[derive(Iden)]
enum Connection {
    Table,
    Id,
    FromProfile,
    ToProfile,
}

#[derive(Iden)]
enum Reaction {
    Table,
    Id,
    FromProfile,
    ToPost,
    ReactionType,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum ReactionType {
    Like,
    Dislike,
    Love,
    Haha,
    Wow,
    Sad,
    Angry,
}
