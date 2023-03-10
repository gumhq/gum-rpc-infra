//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "reaction")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Vec<u8>,
    pub from_profile: Vec<u8>,
    pub to_post: Vec<u8>,
    pub reaction_type: String,
    pub created_at: DateTimeWithTimeZone,
    pub slot_updated_at: i64,
    pub burnt: bool,
    pub compressed: bool,
    pub tree_id: Option<Vec<u8>>,
    pub leaf: Option<Vec<u8>>,
    pub nonce: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::ToPost",
        to = "super::post::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Post,
    #[sea_orm(
        belongs_to = "super::profile::Entity",
        from = "Column::FromProfile",
        to = "super::profile::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Profile,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
