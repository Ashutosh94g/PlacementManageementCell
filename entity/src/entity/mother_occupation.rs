//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "mother_occupation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::family::Entity")]
    Family,
}

impl Related<super::family::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Family.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
