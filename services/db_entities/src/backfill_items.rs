//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "backfill_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub tree: Vec<u8>,
    pub seq: i64,
    pub slot: i64,
    pub force_chk: bool,
    pub backfilled: bool,
    pub failed: bool,
    pub locked: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
