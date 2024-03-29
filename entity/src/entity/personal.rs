//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "personal")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(unique)]
    pub phone: String,
    pub dob: Date,
    pub gender_id: i32,
    pub category_id: i32,
    pub ug_qualification_id: i32,
    pub ug_specialization_id: i32,
    pub ug_startyear: i32,
    pub ug_endyear: i32,
    pub ug_cgpa: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::gender::Entity",
        from = "Column::GenderId",
        to = "super::gender::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Gender,
    #[sea_orm(
        belongs_to = "super::qualification::Entity",
        from = "Column::UgQualificationId",
        to = "super::qualification::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Qualification,
    #[sea_orm(
        belongs_to = "super::specialization::Entity",
        from = "Column::UgSpecializationId",
        to = "super::specialization::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Specialization,
    #[sea_orm(has_many = "super::student::Entity")]
    Student,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::gender::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gender.def()
    }
}

impl Related<super::qualification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Qualification.def()
    }
}

impl Related<super::specialization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Specialization.def()
    }
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
