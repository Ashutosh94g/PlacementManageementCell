use actix_web::{post, web, Either, HttpResponse, Responder};
use entity::entity::{prelude::*, *};
use entity::sea_orm::{EntityTrait, QueryFilter, QuerySelect, RelationTrait};
use entity::{entity::student, sea_orm::ColumnTrait};
use migration::{Condition, SimpleExpr};
use serde::{Deserialize, Serialize};

use crate::AppState;

use super::student::CStudent;

#[derive(Debug, Deserialize, Serialize)]
pub struct FilterStudent {
    pub id: String,
    pub pg_id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    // pub dob: ChronoDate,
    pub father_name: String,
    pub mother_name: String,
    pub city: String,
    pub state: String,
    pub zip: String,

    pub tenth_year: Vec<i32>,
    pub tenth_percentage: f32,
    pub twelfth_year: Vec<i32>,
    pub twelfth_percentage: f32,

    pub ug_startyear: Vec<i32>,
    pub ug_endyear: Vec<i32>,
    pub ug_cgpa: f32,

    pub father_occupation_id: Vec<i32>,
    pub mother_occupation_id: Vec<i32>,
    pub gender: Vec<i32>,
    pub category: Vec<i32>,
    pub ug_qualification_id: Vec<i32>,
    pub ug_specialization_id: Vec<i32>,
    pub tenth_board_id: Vec<i32>,
    pub twelfth_board_id: Vec<i32>,
}

fn create_conditions(conditions: Vec<SimpleExpr>) -> Condition {
    let mut condition_expr = Condition::all();
    for condition in conditions {
        condition_expr = condition_expr.add(condition);
    }

    condition_expr
}

#[post("/filter")]
async fn filter(info: web::Json<FilterStudent>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let tenth_year_vec = info
        .tenth_year
        .iter()
        .map(|year| {
            return family::Column::TenthYear.eq(year.to_owned());
        })
        .collect();

    let twelfth_year_vec = info
        .twelfth_year
        .iter()
        .map(|year| {
            return family::Column::TwelfthYear.eq(year.to_owned());
        })
        .collect();

    let ug_startyear_vec = info
        .ug_startyear
        .iter()
        .map(|year| {
            return personal::Column::UgStartyear.eq(year.to_owned());
        })
        .collect();

    let ug_endyear_vec = info
        .ug_endyear
        .iter()
        .map(|year| {
            return personal::Column::UgStartyear.eq(year.to_owned());
        })
        .collect();

    let father_occupation_id_vec = info
        .father_occupation_id
        .iter()
        .map(|id| {
            return family::Column::FatherOccupationId.eq(id.to_owned());
        })
        .collect();

    let mother_occupation_id_vec = info
        .mother_occupation_id
        .iter()
        .map(|id| {
            return family::Column::MotherOccupationId.eq(id.to_owned());
        })
        .collect();

    let gender_vec = info
        .gender
        .iter()
        .map(|gen| return personal::Column::GenderId.eq(gen.to_owned()))
        .collect();

    let category_vec = info
        .category
        .iter()
        .map(|cate| {
            return personal::Column::CategoryId.eq(cate.to_owned());
        })
        .collect();

    let ug_qualification_id_vec = info
        .ug_qualification_id
        .iter()
        .map(|id| {
            return personal::Column::UgQualificationId.eq(id.to_owned());
        })
        .collect();

    let ug_specialization_id_vec = info
        .ug_specialization_id
        .iter()
        .map(|id| {
            return personal::Column::UgSpecializationId.eq(id.to_owned());
        })
        .collect();

    let tenth_board_id_vec = info
        .tenth_board_id
        .iter()
        .map(|id| {
            return family::Column::TenthBoardId.eq(id.to_owned());
        })
        .collect();

    let twelfth_board_id_vec = info
        .twelfth_board_id
        .iter()
        .map(|id| {
            return family::Column::TwelfthBoardId.eq(id.to_owned());
        })
        .collect();

    let students = Student::find()
        .select_only()
        .column(student::Column::Id)
        .column(personal::Column::Firstname)
        .column(personal::Column::Lastname)
        .column(personal::Column::Email)
        .column(personal::Column::Phone)
        .column(personal::Column::Dob)
        .column_as(gender::Column::Value, "gender")
        .column_as(category::Column::Value, "category")
        .column(personal::Column::UgQualificationId)
        .column(personal::Column::UgSpecializationId)
        .column(personal::Column::UgStartyear)
        .column(personal::Column::UgEndyear)
        .column(personal::Column::UgCgpa)
        .column(family::Column::FatherName)
        .column(family::Column::MotherName)
        .column_as(father_occupation::Column::Value, "father_occupation")
        .column_as(mother_occupation::Column::Value, "mother_occupation")
        .column(family::Column::Address)
        .column(family::Column::City)
        .column(family::Column::State)
        .column(family::Column::Zip)
        .column(family::Column::TenthYear)
        .column(family::Column::TenthPercentage)
        .column(family::Column::TenthBoardId)
        .column(family::Column::TwelfthYear)
        .column(family::Column::TwelfthPercentage)
        .column(family::Column::TwelfthBoardId)
        .column(student::Column::PgId)
        .join(migration::JoinType::Join, student::Relation::Personal.def())
        .join(migration::JoinType::Join, student::Relation::Family.def())
        .join(migration::JoinType::Join, personal::Relation::Gender.def())
        .join(
            migration::JoinType::Join,
            personal::Relation::Category.def(),
        )
        .join(
            migration::JoinType::Join,
            family::Relation::FatherOccupation.def(),
        )
        .join(
            migration::JoinType::Join,
            family::Relation::MotherOccupation.def(),
        )
        .filter(personal::Column::Id.contains(&info.id))
        .filter(personal::Column::Firstname.contains(&info.firstname))
        .filter(personal::Column::Lastname.contains(&info.lastname))
        .filter(personal::Column::Email.contains(&info.email))
        .filter(personal::Column::Phone.contains(&info.phone))
        .filter(family::Column::FatherName.contains(&info.father_name))
        .filter(family::Column::MotherName.contains(&info.mother_name))
        .filter(family::Column::City.contains(&info.city))
        .filter(family::Column::State.contains(&info.state))
        .filter(family::Column::Zip.contains(&info.zip))
        .filter(create_conditions(tenth_year_vec))
        .filter(family::Column::TenthPercentage.gte(info.tenth_percentage.to_owned()))
        .filter(create_conditions(twelfth_year_vec))
        .filter(family::Column::TwelfthPercentage.gte(info.twelfth_percentage.to_owned()))
        .filter(create_conditions(ug_startyear_vec))
        .filter(create_conditions(ug_endyear_vec))
        .filter(personal::Column::UgCgpa.gte(info.ug_cgpa.to_owned()))
        .filter(create_conditions(father_occupation_id_vec))
        .filter(create_conditions(mother_occupation_id_vec))
        .filter(create_conditions(gender_vec))
        .filter(create_conditions(category_vec))
        .filter(create_conditions(ug_qualification_id_vec))
        .filter(create_conditions(ug_specialization_id_vec))
        .filter(create_conditions(tenth_board_id_vec))
        .filter(create_conditions(twelfth_board_id_vec))
        .into_model::<CStudent>()
        .all(db_connection)
        .await;

    match students {
        Ok(student) => Either::Left(HttpResponse::Ok().json(student)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(filter);
}
