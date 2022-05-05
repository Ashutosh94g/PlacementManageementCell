use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};

use entity::sea_orm::prelude::ChronoDate;
use entity::sea_orm::{self, EntityTrait, FromQueryResult, QuerySelect, RelationTrait};
use entity::{
    entity::{prelude::*, *},
    sea_orm::Set,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[post("/student")]
async fn post_student(
    info: web::Json<student::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let student_model = student::ActiveModel {
        id: Set(info.id.to_owned()),
        personal_id: Set(info.personal_id.to_owned()),
        family_id: Set(info.family_id.to_owned()),
        pg_id: Set(info.pg_id.to_owned()),
    };

    let result = Student::insert(student_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Ok().json(model)),
        Err(error) => Either::Right(HttpResponse::InternalServerError().json(error.to_string())),
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, FromQueryResult)]
struct CStudent {
    pub id: String,
    // pub personal_id: i32,
    // pub family_id: i32,
    pub pg_id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pub dob: ChronoDate,
    pub gender: String,
    pub category: String,
    pub ug_qualification_id: i32,
    pub ug_specialization_id: i32,
    pub ug_startyear: i32,
    pub ug_endyear: i32,
    pub ug_cgpa: f32,

    pub father_name: String,
    pub mother_name: String,
    pub father_occupation: String,
    pub mother_occupation: String,
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

#[get("/student")]
async fn get_students(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
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
        .into_model::<CStudent>()
        .all(db_connection)
        .await;

    match students {
        Ok(students) => Either::Left(HttpResponse::Ok().json(students)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[get("/student/{id}")]
async fn get_student_by_id(
    student_id: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let student = Student::find_by_id(student_id.to_owned())
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
        .into_model::<CStudent>()
        .one(db_connection)
        .await;

    match student {
        Ok(student) => Either::Left(HttpResponse::Ok().json(student)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[delete("/student/{id}")]
async fn delete_student_by_id(
    student_id: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let student = Student::delete_by_id(student_id.to_owned())
        .exec(db_connection)
        .await;

    match student {
        Ok(student) => Either::Left(HttpResponse::Ok().json(student.rows_affected)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_student)
        .service(get_students)
        .service(get_student_by_id)
        .service(delete_student_by_id);
}
