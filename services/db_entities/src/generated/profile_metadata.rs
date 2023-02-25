//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "profile_metadata")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Vec<u8>,
    pub profile: Vec<u8>,
    pub metadata_uri: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub metadata_uri_content: Json,
    pub created_at: DateTimeWithTimeZone,
    pub slot_updated_at: i64,
    pub burnt: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::profile::Entity",
        from = "Column::Profile",
        to = "super::profile::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Profile,
}

impl Related<super::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
