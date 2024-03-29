//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "family")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub father_name: String,
    pub mother_name: String,
    pub father_occupation_id: i32,
    pub mother_occupation_id: i32,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub tenth_year: i32,
    pub tenth_percentage: f32,
    pub tenth_board_id: i32,
    pub twelfth_year: i32,
    pub twelfth_percentage: f32,
    pub twelfth_board_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::father_occupation::Entity",
        from = "Column::FatherOccupationId",
        to = "super::father_occupation::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    FatherOccupation,
    #[sea_orm(
        belongs_to = "super::mother_occupation::Entity",
        from = "Column::MotherOccupationId",
        to = "super::mother_occupation::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    MotherOccupation,
    #[sea_orm(
        belongs_to = "super::board::Entity",
        from = "Column::TenthBoardId",
        to = "super::board::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Board2,
    #[sea_orm(
        belongs_to = "super::board::Entity",
        from = "Column::TwelfthBoardId",
        to = "super::board::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Board1,
    #[sea_orm(has_many = "super::student::Entity")]
    Student,
}

impl Related<super::father_occupation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FatherOccupation.def()
    }
}

impl Related<super::mother_occupation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MotherOccupation.def()
    }
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
