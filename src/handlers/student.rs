use actix_web::{get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Student;
use entity::sea_orm::EntityTrait;
use entity::{entity::student, sea_orm::Set};

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

#[get("/student")]
async fn get_students(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let students = Student::find().all(db_connection).await;

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
        .one(db_connection)
        .await;

    match student {
        Ok(student) => Either::Left(HttpResponse::Ok().json(student)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_student)
        .service(get_students)
        .service(get_student_by_id);
}
